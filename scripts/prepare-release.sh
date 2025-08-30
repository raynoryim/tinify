#!/bin/bash

# Prepare release script for tinify-rs
# This script performs all necessary checks before releasing

set -euo pipefail

echo "🚀 Preparing release for tinify-rs..."

# Check if we're on the main/master branch
current_branch=$(git branch --show-current)
if [[ "$current_branch" != "main" && "$current_branch" != "master" ]]; then
    echo "❌ Error: Not on main/master branch. Current branch: $current_branch"
    exit 1
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
    echo "❌ Error: Working directory is not clean. Please commit or stash changes."
    exit 1
fi

echo "✅ Git status check passed"

# Run formatter check
echo "🎨 Checking code formatting..."
if ! cargo fmt --check; then
    echo "❌ Error: Code is not formatted. Run 'cargo fmt' to fix."
    exit 1
fi
echo "✅ Code formatting check passed"

# Run clippy
echo "📎 Running clippy lints..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Error: Clippy found issues. Please fix them."
    exit 1
fi
echo "✅ Clippy check passed"

# Check for typos
echo "🔍 Checking for typos..."
if command -v typos &> /dev/null; then
    if ! typos; then
        echo "❌ Error: Typos found. Run 'typos --write-changes' to fix or update _typos.toml"
        exit 1
    fi
    echo "✅ Typos check passed"
else
    echo "⚠️  Warning: typos tool not found. Install with 'cargo install typos-cli'"
fi

# Run tests
echo "🧪 Running tests..."
if ! cargo test; then
    echo "❌ Error: Tests failed. Please fix them."
    exit 1
fi
echo "✅ Tests passed"

# Check documentation
echo "📚 Checking documentation..."
if ! cargo doc --all --no-deps --document-private-items; then
    echo "❌ Error: Documentation build failed. Please fix doc comments."
    exit 1
fi
echo "✅ Documentation check passed"

# Run security audit
echo "🔒 Running security audit..."
if command -v cargo-audit &> /dev/null; then
    if ! cargo audit; then
        echo "❌ Error: Security vulnerabilities found. Please address them."
        exit 1
    fi
    echo "✅ Security audit passed"
else
    echo "⚠️  Warning: cargo-audit not found. Install with 'cargo install cargo-audit'"
fi

# Check dependencies
echo "📦 Checking dependencies..."
if command -v cargo-deny &> /dev/null; then
    if ! cargo deny check; then
        echo "❌ Error: Dependency issues found. Check deny.toml configuration."
        exit 1
    fi
    echo "✅ Dependencies check passed"
else
    echo "⚠️  Warning: cargo-deny not found. Install with 'cargo install cargo-deny'"
fi

# Test packaging
echo "📦 Testing package creation..."
if ! cargo package --allow-dirty; then
    echo "❌ Error: Package creation failed."
    exit 1
fi
echo "✅ Package creation test passed"

# Generate changelog
echo "📝 Generating changelog..."
if command -v git-cliff &> /dev/null; then
    git cliff --output CHANGELOG.md
    echo "✅ Changelog updated"
else
    echo "⚠️  Warning: git-cliff not found. Install with 'cargo install git-cliff'"
fi

echo ""
echo "🎉 All pre-release checks passed!"
echo ""
echo "Next steps:"
echo "1. Review CHANGELOG.md"
echo "2. Update version in Cargo.toml if needed"
echo "3. Commit any changes: git commit -am 'chore: prepare for release'"
echo "4. Create and push tag: git tag v<version> && git push origin v<version>"
echo "5. Or run: cargo publish"
echo ""
