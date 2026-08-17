#!/usr/bin/env bash
set -euo pipefail

# Configurations for sovereign server nodes
SOVEREIGN_IP="your.sovereign.node.ip"
TARGET_DIR="/var/www/somaos"

echo "🚀 Syncing core blueprints to sovereign bare-metal server node..."

# Synchronize essential deployment code files to host system
ssh root@"$SOVEREIGN_IP" "mkdir -p $TARGET_DIR"
rsync -avz --exclude='.git' ./docker-compose.yml ./Dockerfile ./public ./database ./services root@"$SOVEREIGN_IP":"$TARGET_DIR"

echo "🏗️  Rebuilding isolated local network environment containers on host location..."
ssh root@"$SOVEREIGN_IP" "cd $TARGET_DIR && docker compose down && docker compose up --build -d"

echo "🔒 Deployment execution completed successfully on sovereign architecture."
