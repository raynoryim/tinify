# Cloud Storage Testing Setup

This guide shows how to test the cloud storage examples with real S3-compatible services using free tiers and local development tools.

## 🚀 Quick Setup: Local MinIO (Recommended)

MinIO is a free, local S3-compatible server perfect for testing. It uses default credentials and provides full S3 API compatibility.

### Start MinIO Server

```bash
# Using Docker (recommended)
docker run -p 9000:9000 -p 9001:9001 \
  -e "MINIO_ROOT_USER=minioadmin" \
  -e "MINIO_ROOT_PASSWORD=minioadmin" \
  minio/minio server /data --console-address ":9001"

# Or install locally
brew install minio/stable/minio  # macOS
minio server ~/minio-data        # Start server
```

### Test Configuration

Add these environment variables to test with MinIO:

```bash
export TINIFY_API_KEY="XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq"
export AWS_ACCESS_KEY_ID="minioadmin"
export AWS_SECRET_ACCESS_KEY="minioadmin"
export AWS_ENDPOINT_URL="http://localhost:9000"  # MinIO endpoint
```

### Create Test Bucket

```bash
# Install AWS CLI if needed
pip install awscli

# Configure for MinIO
aws configure set aws_access_key_id minioadmin
aws configure set aws_secret_access_key minioadmin
aws configure set default.region us-east-1

# Create test bucket
aws --endpoint-url=http://localhost:9000 s3 mb s3://test-bucket
```

## ☁️ Free Tier Options

### AWS S3 Free Tier

AWS provides 5GB free storage for 12 months:

```bash
# Sign up at https://aws.amazon.com/free/
# After account setup:
export AWS_ACCESS_KEY_ID="your-actual-aws-key"
export AWS_SECRET_ACCESS_KEY="your-actual-aws-secret"
# No endpoint URL needed for real AWS
```

### Google Cloud Storage Free Tier

GCS provides $300 in credits + always-free tier:

```bash
# Sign up at https://cloud.google.com/free
# Get access token:
gcloud auth application-default print-access-token
export GCP_ACCESS_TOKEN="your-actual-gcp-token"
```

### Third-Party Playgrounds

- **KodeKloud AWS Playground**: [kodekloud.com/playgrounds/playground-aws](https://kodekloud.com/playgrounds/playground-aws)
- **Pluralsight Cloud Sandbox**: [pluralsight.com](https://pluralsight.com) (subscription required)
- **A Cloud Guru Sandboxes**: [acloudguru.com](https://acloudguru.com) (subscription required)

## 🧪 Testing the Examples

### Test S3 Storage with MinIO

```bash
# Start MinIO
docker run -p 9000:9000 -p 9001:9001 \
  -e "MINIO_ROOT_USER=minioadmin" \
  -e "MINIO_ROOT_PASSWORD=minioadmin" \
  minio/minio server /data --console-address ":9001"

# Set environment
export TINIFY_API_KEY="XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq"
export AWS_ACCESS_KEY_ID="minioadmin"
export AWS_SECRET_ACCESS_KEY="minioadmin"

# Run the test
cargo run --example test_s3_minio
```

### Test with Real AWS S3

```bash
# Use your real AWS credentials
export AWS_ACCESS_KEY_ID="AKIA..."
export AWS_SECRET_ACCESS_KEY="..."
export TINIFY_API_KEY="XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq"

# Run S3 example
cargo run --example 05_saving_to_s3
```

### Test with Google Cloud Storage

```bash
# Get GCP access token
export GCP_ACCESS_TOKEN=$(gcloud auth application-default print-access-token)
export TINIFY_API_KEY="XZmVxmxJxbx4PZbHyxwX74v8N0LLtvqq"

# Run GCS example
cargo run --example 06_saving_to_gcs
```

## 🔧 Custom Test Example

Create a comprehensive test with both local and cloud storage:
