# 🎉 Scode v0.2.0 - 重大更新！

## 新增功能

### ✅ 会话持久化
- 自动保存所有对话到 `~/.scode/sessions/`
- 使用 JSONL 格式存储，易于查看和恢复
- 支持恢复之前的会话继续对话

**使用方法：**
```bash
# 列出所有会话
./target/release/scode --list-sessions

# 恢复特定会话
./target/release/scode --resume <session-id>
```

### ✅ 上下文自动压缩
- 当对话接近 200k tokens 时自动触发
- 使用 AI 总结旧消息，保留最近 10 条
- 节省 tokens，支持更长的对话

### ✅ 新增 6 个工具

**Git 工具（5个）：**
- `git_status` - 查看仓库状态
- `git_diff` - 查看文件变更
- `git_add` - 暂存文件
- `git_commit` - 创建提交
- `git_log` - 查看提交历史

**Web 工具（1个）：**
- `web_fetch` - 获取网页内容

### ✅ 命令系统
- `/help` - 显示帮助信息
- `/clear` - 清空当前对话
- `/sessions` - 列出所有会话
- `/exit` - 退出并保存

## 工具总览

现在 Scode 共有 **12 个工具**：

| 类别 | 工具 | 数量 |
|------|------|------|
| 文件操作 | file_read, file_write, file_edit | 3 |
| 搜索 | glob, grep | 2 |
| Git | git_status, git_diff, git_add, git_commit, git_log | 5 |
| 系统 | bash | 1 |
| Web | web_fetch | 1 |

## 使用示例

### 会话管理

```bash
# 开始新会话（自动）
./target/release/scode

# 列出所有会话
./target/release/scode --list-sessions

# 恢复会话
./target/release/scode --resume abc123-def456-...
```

### Git 工作流

```bash
./run.sh "检查 git 状态，如果有修改的文件，暂存它们并创建一个提交"
```

AI 会自动：
1. 运行 `git_status` 查看状态
2. 使用 `git_add` 暂存文件
3. 使用 `git_commit` 创建提交

### Web 内容获取

```bash
./run.sh "获取 https://example.com 的内容并总结"
```

### 长对话支持

现在可以进行非常长的对话，系统会自动压缩旧消息：

```bash
./target/release/scode

You: 创建一个 web 服务器
🤖 Scode: [创建代码...]

You: 添加数据库支持
🤖 Scode: [添加代码...]

You: 添加认证功能
🤖 Scode: [添加代码...]

# ... 继续对话，系统会自动压缩旧消息
```

## 性能提升

| 指标 | v0.1.0 | v0.2.0 | 提升 |
|------|--------|--------|------|
| 工具数量 | 6 | 12 | +100% |
| 最大对话长度 | ~50 轮 | 无限制 | ∞ |
| 会话持久化 | ❌ | ✅ | 新增 |
| 命令系统 | ❌ | ✅ | 新增 |

## 架构改进

```
scode/
├── src/
│   ├── agent/          # Agent 主循环
│   ├── api/            # API 客户端
│   ├── tools/          # 12 个工具
│   │   ├── bash.rs
│   │   ├── file.rs
│   │   ├── search.rs
│   │   ├── git.rs      # 新增
│   │   └── web.rs      # 新增
│   └── utils/          # 工具模块
│       ├── session.rs  # 新增：会话管理
│       └── context.rs  # 新增：上下文压缩
└── ~/.scode/
    └── sessions/       # 会话存储目录
        ├── uuid1.jsonl
        ├── uuid2.jsonl
        └── ...
```

## 升级指南

### 从 v0.1.0 升级

1. 重新编译：
```bash
cd /root/.openclaw/workspace/scode
cargo build --release
```

2. 旧版本的对话不会自动迁移（v0.1.0 没有会话功能）

3. 新功能立即可用，无需配置

### 配置

所有会话自动保存到 `~/.scode/sessions/`，无需手动配置。

## 下一步计划

### v0.3.0（计划中）
- [ ] 规划模式（/plan 命令）
- [ ] 配置文件支持（~/.scode/config.toml）
- [ ] 更好的 UI（颜色、进度条）
- [ ] 更多 Web 工具（web_search）

### v0.4.0（计划中）
- [ ] 子 Agent 支持
- [ ] TUI 全屏界面
- [ ] MCP 协议支持

## 已知问题

1. 会话文件可能会变大（长对话）
2. 上下文压缩需要额外的 API 调用
3. Git 工具需要在 git 仓库中使用

## 贡献

欢迎提交 Issue 和 PR！

## 更新日志

### v0.2.0 (2026-04-01)
- ✅ 新增会话持久化
- ✅ 新增上下文自动压缩
- ✅ 新增 6 个工具（5 个 Git + 1 个 Web）
- ✅ 新增命令系统
- ✅ 改进错误处理
- ✅ 优化用户体验

### v0.1.0 (2026-04-01)
- ✅ 初始版本
- ✅ 6 个基础工具
- ✅ Agent 主循环
- ✅ 权限系统

---

**Scode v0.2.0 - 更强大，更智能，更易用！** 🚀
