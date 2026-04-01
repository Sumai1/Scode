#!/bin/bash

# Scode 快速启动脚本

echo "🚀 Scode - AI Code Agent"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 设置环境变量
export ANTHROPIC_API_KEY="sk-kiro-rs-CiQ4a7JD7on7C5qYovuPxg6i"
export ANTHROPIC_BASE_URL="http://140.245.45.173:8990"

cd /root/.openclaw/workspace/scode

echo "✅ 环境配置："
echo "   API Key: ${ANTHROPIC_API_KEY:0:20}..."
echo "   Base URL: $ANTHROPIC_BASE_URL"
echo ""

# 检查是否有参数
if [ $# -eq 0 ]; then
    echo "💡 交互模式 - 输入你的请求（输入 'exit' 退出）"
    echo ""
    ./target/release/scode
else
    echo "📝 执行任务: $*"
    echo ""
    ./target/release/scode -p "$*"
fi
