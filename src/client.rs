use crate::error::{Result, TinifyError};
use base64::Engine;
use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use reqwest::{Client as ReqwestClient, Response};
use std::{num::NonZeroU32, sync::Arc, time::Duration};
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;
use tracing::{debug, info, instrument, warn};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_MAX_RETRIES: u32 = 3;
const DEFAULT_RATE_LIMIT: u32 = 100; // requests per minute

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_RETRIES,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_factor: 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub burst_capacity: u32,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            requests_per_minute: DEFAULT_RATE_LIMIT,
            burst_capacity: 10,
        }
    }
}

#[derive(Debug)]
pub struct Client {
    http_client: ReqwestClient,
    api_key: String,
    app_identifier: Option<String>,
    retry_config: RetryConfig,
    rate_limiter: Arc<
        RateLimiter<
            governor::state::direct::NotKeyed,
            governor::state::InMemoryState,
            governor::clock::DefaultClock,
        >,
    >,
}

impl Client {
    pub fn new(api_key: String) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    fn create_http_client(timeout: Duration) -> Result<ReqwestClient> {
        ReqwestClient::builder()
            .timeout(timeout)
            .build()
            .map_err(TinifyError::ConnectionError)
    }

    fn create_rate_limiter(
        rate_limit: &RateLimit,
    ) -> Arc<
        RateLimiter<
            governor::state::direct::NotKeyed,
            governor::state::InMemoryState,
            governor::clock::DefaultClock,
        >,
    > {
        let requests_per_minute =
            NonZeroU32::new(rate_limit.requests_per_minute).unwrap_or(nonzero!(100u32));
        let burst_capacity = NonZeroU32::new(rate_limit.burst_capacity).unwrap_or(nonzero!(10u32));
        let quota = Quota::per_minute(requests_per_minute).allow_burst(burst_capacity);
        Arc::new(RateLimiter::direct(quota))
    }

    #[instrument(skip(response), fields(status = %response.status()))]
    async fn handle_error_response(response: Response) -> Result<Response> {
        if response.status().is_success() {
            return Ok(response);
        }

        let status = response.status().as_u16();

        // Get headers before consuming response
        let retry_after = response
            .headers()
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);

        let error_body = response
            .json::<serde_json::Value>()
            .await
            .unwrap_or_default();

        let message = error_body
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Unknown error")
            .to_string();

        let error_type = error_body
            .get("error")
            .and_then(|e| e.as_str())
            .map(String::from);

        debug!("API error response: status={}, message={}", status, message);

        match status {
            401 => {
                if message.contains("credentials") {
                    Err(TinifyError::InvalidApiKey)
                } else {
                    Err(TinifyError::AccountError {
                        message,
                        error_type,
                        status: Some(status),
                    })
                }
            }
            429 => {
                if message.contains("quota") {
                    Err(TinifyError::QuotaExceeded)
                } else {
                    Err(TinifyError::RateLimitExceeded { retry_after })
                }
            }
            400..=499 => Err(TinifyError::ClientError {
                message,
                error_type,
                status: Some(status),
            }),
            500..=599 => Err(TinifyError::ServerError {
                message,
                error_type,
                status: Some(status),
            }),
            _ => Err(TinifyError::UnknownError { message }),
        }
    }

    fn add_common_headers(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let auth = format!("api:{}", self.api_key);
        let auth_header = format!(
            "Basic {}",
            Engine::encode(&base64::engine::general_purpose::STANDARD, auth)
        );
        let request = request.header("Authorization", auth_header);

        match &self.app_identifier {
            Some(app_id) => request.header("User-Agent", app_id),
            None => request,
        }
    }

    #[instrument(skip(self))]
    async fn check_rate_limit(&self) -> Result<()> {
        if self.rate_limiter.check().is_err() {
            warn!("Rate limit exceeded, waiting for next available slot");
            self.rate_limiter.until_ready().await;
        }
        Ok(())
    }

    async fn execute_request<F, Fut>(&self, request_fn: F) -> Result<Response>
    where
        F: Fn() -> Fut + Send,
        Fut: std::future::Future<Output = Result<Response>> + Send,
    {
        let mut delay = self.retry_config.base_delay;

        for attempt in 1..=self.retry_config.max_attempts {
            self.check_rate_limit().await?;

            match request_fn().await {
                Ok(response) => return Ok(response),
                Err(err) => {
                    if attempt == self.retry_config.max_attempts {
                        return Err(err);
                    }

                    match &err {
                        TinifyError::ConnectionError(_)
                        | TinifyError::ServerError { .. }
                        | TinifyError::RateLimitExceeded { .. } => {
                            warn!(
                                "Request failed (attempt {}/{}), retrying in {:?}: {}",
                                attempt, self.retry_config.max_attempts, delay, err
                            );
                            tokio::time::sleep(delay).await;

                            delay = std::cmp::min(
                                Duration::from_millis(
                                    (delay.as_millis() as f64 * self.retry_config.backoff_factor)
                                        as u64,
                                ),
                                self.retry_config.max_delay,
                            );
                        }
                        _ => return Err(err),
                    }
                }
            }
        }

        unreachable!()
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn set_app_identifier(&mut self, app_identifier: String) {
        self.app_identifier = Some(app_identifier);
    }

    #[instrument(skip(self, body))]
    pub async fn post<S: AsRef<str> + std::fmt::Debug>(
        &self,
        url: S,
        body: Option<Vec<u8>>,
    ) -> Result<Response> {
        let url = url.as_ref();
        info!("Making POST request to: {}", url);

        self.execute_request(|| {
            let request = self.http_client.post(url);
            let mut request = self.add_common_headers(request);

            if let Some(ref body_data) = body {
                if body_data.starts_with(b"{") || body_data.starts_with(b"[") {
                    request = request.header("Content-Type", "application/json");
                }
                request = request.body(body_data.clone());
            }

            async move {
                let response = request.send().await.map_err(TinifyError::ConnectionError)?;
                Self::handle_error_response(response).await
            }
        })
        .await
    }

    #[instrument(skip(self, stream))]
    pub async fn post_stream<S: AsRef<str> + std::fmt::Debug, R>(
        &self,
        url: S,
        stream: R,
        content_type: &str,
    ) -> Result<Response>
    where
        R: AsyncRead + Send + Sync + 'static,
    {
        let url = url.as_ref();
        info!("Making POST stream request to: {}", url);

        let reader_stream = ReaderStream::new(stream);
        let stream_body = reqwest::Body::wrap_stream(reader_stream);

        let request = self.http_client.post(url);
        let request = self
            .add_common_headers(request)
            .header("Content-Type", content_type)
            .body(stream_body);

        let response = request.send().await.map_err(TinifyError::ConnectionError)?;
        Self::handle_error_response(response).await
    }

    #[instrument(skip(self))]
    pub async fn get<S: AsRef<str> + std::fmt::Debug>(&self, url: S) -> Result<Response> {
        let url = url.as_ref();
        info!("Making GET request to: {}", url);

        self.execute_request(|| {
            let request = self.http_client.get(url);
            let request = self.add_common_headers(request);

            async move {
                let response = request.send().await.map_err(TinifyError::ConnectionError)?;
                Self::handle_error_response(response).await
            }
        })
        .await
    }
}

pub struct ClientBuilder {
    api_key: Option<String>,
    app_identifier: Option<String>,
    timeout: Duration,
    retry_config: RetryConfig,
    rate_limit: RateLimit,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            api_key: None,
            app_identifier: None,
            timeout: DEFAULT_TIMEOUT,
            retry_config: RetryConfig::default(),
            rate_limit: RateLimit::default(),
        }
    }

    pub fn api_key<S: Into<String>>(mut self, key: S) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn app_identifier<S: Into<String>>(mut self, identifier: S) -> Self {
        self.app_identifier = Some(identifier.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    pub fn rate_limit(mut self, limit: RateLimit) -> Self {
        self.rate_limit = limit;
        self
    }

    pub fn max_retry_attempts(mut self, attempts: u32) -> Self {
        self.retry_config.max_attempts = attempts;
        self
    }

    pub fn requests_per_minute(mut self, rpm: u32) -> Self {
        self.rate_limit.requests_per_minute = rpm;
        self
    }

    pub fn build(self) -> Result<Client> {
        let api_key = self.api_key.ok_or(TinifyError::InvalidApiKey)?;
        let http_client = Client::create_http_client(self.timeout)?;
        let rate_limiter = Client::create_rate_limiter(&self.rate_limit);

        Ok(Client {
            http_client,
            api_key,
            app_identifier: self.app_identifier,
            retry_config: self.retry_config,
            rate_limiter,
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
