#!/bin/bash

echo "🧪 Testing Scode..."
echo ""

cd /root/.openclaw/workspace/scode

# Test 1: Help
echo "Test 1: Show help"
./target/release/scode --help
echo ""

# Test 2: Version info
echo "Test 2: Binary info"
file target/release/scode
ls -lh target/release/scode
echo ""

echo "✅ Scode is ready!"
echo ""
echo "📝 Quick start:"
echo ""
echo "export ANTHROPIC_API_KEY='your-api-key-here'"
echo "export ANTHROPIC_BASE_URL='https://api.anthropic.com'"
echo "./target/release/scode -p 'Read README.md and summarize it'"
