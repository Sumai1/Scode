#!/bin/bash

# Scode Quick Start Script

echo "🚀 Scode - AI Code Agent"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Build the project
echo "🔨 Building Scode..."
cd /root/.openclaw/workspace/scode
cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "📝 Usage examples:"
    echo ""
    echo "# Interactive mode:"
    echo "export ANTHROPIC_API_KEY='your-key'"
    echo "export ANTHROPIC_BASE_URL='https://api.anthropic.com'"
    echo "./target/release/scode"
    echo ""
    echo "# Single prompt:"
    echo "./target/release/scode -p 'Read Cargo.toml and explain it'"
    echo ""
    echo "# With your kiro.rs endpoint:"
    echo "./target/release/scode \\"
    echo "  --api-key 'your-api-key-here' \\"
    echo "  --base-url 'https://api.anthropic.com' \\"
    echo "  -p 'Create a hello world program'"
else
    echo "❌ Build failed. Check the errors above."
    exit 1
fi
