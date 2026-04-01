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
echo "export ANTHROPIC_API_KEY='sk-kiro-rs-CiQ4a7JD7on7C5qYovuPxg6i'"
echo "export ANTHROPIC_BASE_URL='http://140.245.45.173:8990'"
echo "./target/release/scode -p 'Read README.md and summarize it'"
