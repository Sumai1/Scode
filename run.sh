#!/bin/bash

# Scode 快速启动脚本

echo "🚀 Scode - AI Code Agent"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 设置环境变量
export ANTHROPIC_API_KEY="your-api-key-here"
export ANTHROPIC_BASE_URL="https://api.anthropic.com"

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
