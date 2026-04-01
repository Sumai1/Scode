# Scode 使用示例

## 快速开始

### 1. 使用你的 kiro.rs 端点

```bash
export ANTHROPIC_API_KEY="sk-kiro-rs-CiQ4a7JD7on7C5qYovuPxg6i"
export ANTHROPIC_BASE_URL="http://140.245.45.173:8990"

./target/release/scode
```

### 2. 单次提示模式

```bash
./target/release/scode -p "读取 Cargo.toml 并列出所有依赖"
```

### 3. 交互模式

```bash
./target/release/scode

You: 创建一个简单的 HTTP 服务器
🤖 Scode: [AI 会使用工具来完成任务]

You: 读取 src/main.rs 并解释代码
🤖 Scode: [AI 会读取文件并解释]
```

## 示例任务

### 代码分析
```bash
./target/release/scode -p "分析 src/ 目录下的所有 Rust 文件，找出潜在的性能问题"
```

### 创建项目
```bash
./target/release/scode -p "创建一个完整的 REST API 项目，包含路由、中间件和错误处理"
```

### 重构代码
```bash
./target/release/scode -p "重构 src/main.rs，将功能拆分到不同的模块中"
```

### 添加功能
```bash
./target/release/scode -p "在当前项目中添加日志功能，使用 tracing 库"
```

### 调试问题
```bash
./target/release/scode -p "运行测试并修复所有失败的测试用例"
```

## 工具说明

Scode 有 6 个核心工具：

1. **bash** - 执行 shell 命令
   - 示例：运行测试、编译代码、安装依赖

2. **file_read** - 读取文件内容
   - 示例：查看源代码、配置文件

3. **file_write** - 写入文件
   - 示例：创建新文件、覆盖现有文件

4. **file_edit** - 编辑文件（精确替换）
   - 示例：修改代码、更新配置

5. **glob** - 查找文件
   - 示例：找到所有 .rs 文件、所有测试文件

6. **grep** - 搜索文本
   - 示例：查找 TODO 注释、搜索函数定义

## 权限系统

当 AI 尝试执行需要权限的操作时（如写文件、执行命令），会提示你确认：

```
⚠️  Tool 'bash' requires permission
   Input: {"command": "rm -rf /"}
   Allow? [y/N]: 
```

只有输入 `y` 才会执行。

## 高级用法

### 使用不同的模型

```bash
./target/release/scode \
  --model "claude-opus-4-6" \
  -p "复杂的重构任务"
```

### 自定义 API 端点

```bash
./target/release/scode \
  --api-key "your-key" \
  --base-url "http://your-endpoint" \
  -p "your task"
```

## 提示技巧

1. **明确任务**：清楚地描述你想要什么
   - ❌ "改进代码"
   - ✅ "重构 main.rs，将 HTTP 处理逻辑提取到单独的模块"

2. **提供上下文**：告诉 AI 相关信息
   - "这是一个 web 服务器项目，使用 actix-web 框架"

3. **分步骤**：复杂任务可以分解
   - "首先读取所有 .rs 文件，然后分析依赖关系，最后生成文档"

4. **验证结果**：让 AI 验证自己的工作
   - "创建测试文件后，运行测试确保它们通过"

## 故障排除

### 编译失败
```bash
# 确保安装了 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装依赖
apt-get install pkg-config libssl-dev

# 重新编译
cargo build --release
```

### API 连接失败
```bash
# 检查端点是否可访问
curl http://140.245.45.173:8990/v1/models \
  -H "x-api-key: sk-kiro-rs-CiQ4a7JD7on7C5qYovuPxg6i"

# 确保环境变量正确
echo $ANTHROPIC_API_KEY
echo $ANTHROPIC_BASE_URL
```

### 权限问题
```bash
# 如果工具一直要求权限，可以修改代码
# 在 src/tools/*.rs 中将 requires_permission() 改为 false
```

## 下一步

- [ ] 添加会话持久化（保存/恢复对话）
- [ ] 实现上下文压缩（处理长对话）
- [ ] 添加更多工具（git、web search）
- [ ] 实现 TUI 界面
- [ ] 支持规划模式
- [ ] 添加子 Agent 功能
