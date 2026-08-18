#!/usr/bin/env bash
# File Path: scripts/download_precision_data.sh
set -euo pipefail

# Define source buckets and regional data storage paths
AWS_BUCKET_SOURCE_PATH="s3://sid-sijbrandij-osteosarc-dataset/curated-variants/"
LOCAL_TARGET_DIRECTORY="./database/bioinformatics_cache"

echo "🪐 SomaOS Bioinformatics Pipeline Core Initializing..."
echo "📂 Target Retrieval Point: $AWS_BUCKET_SOURCE_PATH"

# Ensure target storage directories exist on host system hardware lines
mkdir -p "$LOCAL_TARGET_DIRECTORY"

# Verify if the AWS CLI execution toolchain is available on the local terminal
if ! command -v aws &> /dev/null; then
    echo "⚠️ System Error: Missing AWS CLI architecture. Please run: sudo apt install awscli"
    exit 1
fi

echo "🚀 Syncing Variant Call Format (.vcf) structural files down to local memory workspace caches..."
# Execute the AWS sync command using anonymized read request protocols to bypass credential barriers
aws s3 sync "$AWS_BUCKET_SOURCE_PATH" "$LOCAL_TARGET_DIRECTORY" \
    --no-sign-request \
    --exclude "*" \
    --include "*.vcf" \
    --include "*.vcf.gz"

echo "✅ Multi-Omic Variant Synchronization execution completed successfully."
echo "📋 Buffered records written to location: $(realpath "$LOCAL_TARGET_DIRECTORY")"
