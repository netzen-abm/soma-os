#!/usr/bin/env bash
# File Path: scripts/pull_vcf_data.sh
set -euo pipefail

TARGET_OUT_DIR="./data/bioinformatics/variants"
BUCKET_MANIFEST="s3://sid-sijbrandij-osteosarc-dataset/variants/sample_manifest.vcf"

echo "🪐 SomaOS Bioinformatics Sync Init..."

# Enforce AWS CLI environment checks before executing network commands
if ! command -v aws &> /dev/null; then
    echo "❌ Dependencies Missing: AWS CLI is not installed on this system host."
    echo "Install via: curl 'https://amazonaws.com' -o 'awscliv2.zip'"
    exit 1
fi

mkdir -p "$TARGET_OUT_DIR"

echo "📡 Syncing curated genomic target data anonymously from S3 bucket pipelines..."
# Executes public access sync commands without requiring personal access token keys
aws s3 cp "$BUCKET_MANIFEST" "$TARGET_OUT_DIR/sample_manifest.vcf" --no-sign-request

if [ -f "$TARGET_OUT_DIR/sample_manifest.vcf" ]; then
    echo "✅ Success: Multi-omic variant footprint data buffered locally at: $TARGET_OUT_DIR/sample_manifest.vcf"
else
    echo "❌ Sync Failure: Target file transfer could not be validated."
    exit 1
fi
