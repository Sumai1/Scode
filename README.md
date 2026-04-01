# 🚀 Scode - AI Code Agent

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.6.1-green.svg)](https://github.com/yourusername/scode)

一个用 Rust 构建的 AI 代码助手，灵感来自 Claude Code。

## ✨ 特性

- 🛠️ **22 个实用工具** - 文件操作、Git、搜索、Web 等
- 💬 **15 个交互命令** - 完整的命令系统
- 🎨 **彩色 UI** - 美观的终端界面
- 🤖 **子 Agent 系统** - 并行任务执行
- 📋 **规划模式** - 智能任务规划
- 🎓 **Skill 系统** - 动态加载领域知识
- 📥 **Skill 安装** - 从 URL/GitHub 安装 skills
- 💾 **会话持久化** - 自动保存对话历史
- 🔄 **上下文压缩** - 智能管理上下文
- ⚙️ **配置文件** - 灵活的配置系统

## 📊 与 Claude Code 对比

| 指标 | Claude Code | Scode | 比率 |
|------|-------------|-------|------|
| 代码行数 | ~50,000 | ~3,500 | 0.7% |
| 工具数量 | 40+ | 22 | 55% |
| 功能完成度 | 100% | 87% | 87% |

**用 0.7% 的代码实现了 87% 的功能！**

## 🚀 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/yourusername/scode.git
cd scode

# 编译
cargo build --release

# 运行
./target/release/scode
```

### 使用

```bash
# 启动 Scode
./run.sh

# 或者直接运行
./target/release/scode

# 查看帮助
./target/release/scode --help
```

## 🛠️ 工具列表

### 文件操作（8 个）
- `file_read` - 读取文件
- `file_write` - 写入文件
- `file_edit` - 编辑文件
- `file_list` - 列出目录
- `file_move` - 移动文件
- `file_copy` - 复制文件
- `file_delete` - 删除文件
- `file_info` - 文件信息

### 搜索（2 个）
- `glob` - 文件模式匹配
- `grep` - 文本搜索

### Git（8 个）
- `git_status` - 状态
- `git_diff` - 差异
- `git_add` - 暂存
- `git_commit` - 提交
- `git_log` - 历史
- `git_branch` - 分支
- `git_pull` - 拉取
- `git_push` - 推送

### 系统（1 个）
- `bash` - 命令执行

### Web（3 个）
- `web_fetch` - 获取网页
- `web_search` - 搜索网页
- `http_request` - HTTP 请求

## 💬 命令列表

| 命令 | 功能 |
|------|------|
| `/help` | 显示帮助 |
| `/plan` | 规划模式 |
| `/exit-plan` | 退出规划 |
| `/spawn` | 创建子 Agent |
| `/agents` | 列出子 Agent |
| `/skills` | 列出 skills |
| `/load` | 加载 skill |
| `/unload` | 卸载 skill |
| `/install` | 安装 skill |
| `/remove` | 删除 skill |
| `/mode` | 显示模式 |
| `/clear` | 清空对话 |
| `/sessions` | 列出会话 |
| `/config` | 显示配置 |
| `/exit` | 退出 |

## 🎓 Skill 系统

### 内置 Skills

- **Rust Programming** - Rust 编程最佳实践
- **Web Development** - Web 开发指南
- **Git Workflow** - Git 工作流程

### 使用 Skills

```bash
# 列出所有 skills
You: /skills

# 加载 skill
You: /load Rust Programming

# 从 URL 安装 skill
You: /install
Enter URL: https://example.com/python-skill.md

# 从 GitHub 安装 skill
You: /install
Enter URL: username/repo

# 卸载 skill
You: /unload
```

## ⚙️ 配置

配置文件位置：`~/.scode/config.toml`

```toml
[api]
api_key = "your-api-key"
base_url = "http://140.245.45.173:8990"
model = "claude-sonnet-4-5-20250929"

[agent]
max_iterations = 25
auto_approve_read_only = true

[ui]
colors = true
show_tokens = true
show_progress = true
```

## 📖 文档

- [快速开始](QUICKSTART.md)
- [Skill 系统](SKILL_SYSTEM.md)
- [Skill 安装](SKILL_INSTALL.md)
- [版本历史](CHANGELOG.md)

## 🎯 版本历史

### v0.6.1 (2026-04-01)
- ✅ 新增 Skill 安装功能
- ✅ 支持从 URL/GitHub 安装 skills
- ✅ 新增 /install 和 /remove 命令

### v0.6.0 (2026-04-01)
- ✅ 新增 Skill 系统
- ✅ 3 个内置 skills
- ✅ 新增 /skills, /load, /unload 命令

### v0.5.2 (2026-04-01)
- ✅ 混合 Claude Code 风格的提示词
- ✅ 提示词长度 +109%

### v0.5.1 (2026-04-01)
- ✅ 专业级提示词系统
- ✅ 5 种工作模式

### v0.5.0 (2026-04-01)
- ✅ 新增 7 个工具（Web + 文件系统）
- ✅ 工具总数达到 22 个

### v0.4.0 (2026-04-01)
- ✅ 彩色 UI
- ✅ 子 Agent 系统
- ✅ 并发执行

### v0.3.0 (2026-04-01)
- ✅ 规划模式
- ✅ 配置文件
- ✅ 3 个 Git 工具

### v0.2.0 (2026-04-01)
- ✅ 会话持久化
- ✅ 上下文压缩
- ✅ 6 个新工具

### v0.1.0 (2026-04-01)
- ✅ 初始版本
- ✅ 6 个基础工具

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

## 📄 许可证

MIT License

## 🙏 致谢

- 灵感来自 [Claude Code](https://github.com/anthropics/claude-code)
- 使用 [kiro.rs](http://140.245.45.173:8990) API

## 📞 联系

- GitHub: [@yourusername](https://github.com/yourusername)
- Issues: [GitHub Issues](https://github.com/yourusername/scode/issues)

---

**用 Rust 构建，用 AI 驱动** 🚀
