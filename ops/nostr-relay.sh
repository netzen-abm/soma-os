#!/usr/bin/env bash
# File Path: ops/nostr-relay.sh
set -euo pipefail

echo "🪐 Initializing local P2P Nostr Relay Environment..."

# Update package directories securely on host terminal node
sudo apt-get update && sudo apt-get install -y git curl build-essential

# Verify if Docker runtime environment architecture exists on machine location
if ! command -v docker &> /dev/null; then
    echo "🐳 Installing missing Docker runtime infrastructure..."
    curl -fsSL https://docker.com -o get-docker.sh
    sh get-docker.sh
fi

# Construct dedicated configuration layout directories for our Nostr relay node
mkdir -p "$HOME/.somaos/nostr-relay"
cd "$HOME/.somaos/nostr-relay"

# Generate production config profile parameters for the rust-based 'Nostr-RS-Relay' microservice
cat << 'EOF' > config.toml
[info]
relay_url = "ws://127.0.0.1:8077/"
name = "SomaOS Sovereign Medicine Node"
description = "A localized peer-to-peer distributed relay system handling open-source health protocols."
pubkey = "0000000000000000000000000000000000000000000000000000000000000000"

[network]
address = "0.0.0.0"
port = 8077

[database]
data_directory = "/usr/src/app/db"
in_memory = false

[limits]
max_event_size = 65536
max_ws_message_size = 131072
max_filters = 16
EOF

echo "🐳 Pulling and executing the lightweight Nostr infrastructure node service wrapper container..."
docker run -d \
  --name somaos-nostr-relay \
  -p 8077:8077 \
  -v "$PWD/config.toml:/usr/src/app/config.toml" \
  -v "$PWD/db:/usr/src/app/db" \
  --restart unless-stopped \
  scottswilliams/nostr-rs-relay:latest

echo "✅ Nostr relay successfully initialized. Operating locally on port ws://localhost:8077"
