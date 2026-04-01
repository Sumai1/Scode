#!/bin/bash
# Scode Installer
# 一键安装 Scode，让你可以在任意终端输入 scode 使用

set -e

echo "🚀 Scode 安装脚本"
echo "==================="

# 检测系统
OS=$(uname -s)
ARCH=$(uname -m)

echo "检测到系统: $OS $ARCH"

# 设置安装路径
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
    echo "将安装到系统目录: $INSTALL_DIR"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    echo "将安装到用户目录: $INSTALL_DIR"
    
    # 自动将 ~/.local/bin 添加到 PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        SHELL_RC=""
        if [ -f "$HOME/.zshrc" ]; then
            SHELL_RC="$HOME/.zshrc"
        elif [ -f "$HOME/.bashrc" ]; then
            SHELL_RC="$HOME/.bashrc"
        fi
        
        if [ -n "$SHELL_RC" ]; then
            echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$SHELL_RC"
            echo "已自动将 $INSTALL_DIR 添加到 PATH（$SHELL_RC）"
        fi
    fi
fi

BINARY_NAME="scode"
INSTALL_PATH="$INSTALL_DIR/$BINARY_NAME"

# 构建项目
echo "正在编译 Scode（这可能需要一点时间）..."
cargo build --release --quiet

# 复制二进制文件
cp "target/release/$BINARY_NAME" "$INSTALL_PATH"
chmod +x "$INSTALL_PATH"

echo ""
echo "✅ 安装完成！"
echo ""
echo "现在你可以直接在终端输入以下命令："
echo "   scode"
echo ""
echo "其他可用命令："
echo "   scode --help          查看帮助"
echo "   scode --init-config   初始化配置文件"
echo ""
echo "配置文件位置: ~/.scode/config.toml"
echo ""
echo "🎉 祝使用愉快！"
echo "项目地址: https://github.com/Sumai1/Scode"
