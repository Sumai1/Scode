# 🎓 Scode v0.6.0 - Skill 系统

## 新增功能

### ✅ Skill 系统（类似 Claude Code）

**什么是 Skill？**

Skill 是领域特定的知识模块，可以动态加载到 AI 的上下文中，让 AI 拥有特定领域的专业知识。

**特点：**
- 📚 模块化知识管理
- 🔄 动态加载/卸载
- 📝 Markdown 格式
- 🏷️ 分类和标签
- ⚡ 即时生效

## Skill 系统架构

```
~/.scode/skills/
├── rust.md          # Rust 编程 skill
├── web.md           # Web 开发 skill
├── git.md           # Git 工作流 skill
└── custom.md        # 自定义 skill
```

### Skill 文件格式

```markdown
---
name: Rust Programming
description: Expert knowledge of Rust programming language
category: programming
tags: [rust, systems, memory-safety]
version: 1.0.0
---

# Rust Programming Skill

## Core Principles

- **Ownership**: Every value has a single owner
- **Borrowing**: References allow access without ownership transfer
...

## Common Patterns

### Error Handling
\`\`\`rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
\`\`\`

## Best Practices

1. Use `&str` for string slices
2. Prefer `impl Trait` for return types
...
```

## 内置 Skills

### 1. Rust Programming

**内容：**
- Core Principles (Ownership, Borrowing, Lifetimes)
- Common Patterns (Error Handling, Option, Async/Await)
- Best Practices
- Common Crates
- Memory Safety

**使用场景：**
- Rust 项目开发
- 系统编程
- 性能优化

### 2. Web Development

**内容：**
- Frontend Best Practices (React/Vue/Svelte)
- CSS/Styling
- Performance Optimization
- Backend Best Practices (RESTful API, Security)
- Database Patterns
- API Design (GraphQL, REST)
- Testing

**使用场景：**
- Web 应用开发
- API 设计
- 前后端开发

### 3. Git Workflow

**内容：**
- Commit Message Format (Conventional Commits)
- Branching Strategy (Git Flow, GitHub Flow)
- Best Practices
- Useful Commands
- Code Review Checklist

**使用场景：**
- 版本控制
- 团队协作
- 代码审查

## 使用方法

### 列出所有 Skills

```bash
./run.sh

You: /skills
ℹ Available Skills (3):
  [ ] Rust Programming - Expert knowledge of Rust programming language
      Category: programming | Tags: rust, systems, memory-safety
  [ ] Web Development - Modern web development practices and patterns
      Category: web | Tags: web, frontend, backend, api
  [✓] Git Workflow - Professional Git workflow and best practices
      Category: tools | Tags: git, version-control, workflow

Active skills: 1
Use /load <skill-name> to activate a skill
```

### 加载 Skill

```bash
You: /load Rust Programming
✓ Loaded skill: Rust Programming
   Expert knowledge of Rust programming language
```

### 卸载 Skill

```bash
You: /unload
ℹ Active skills:
   1. Rust Programming
   2. Web Development
Enter skill name to unload (or 'all'): Rust Programming
✓ Unloaded skill: Rust Programming
```

### 使用 Skill

加载 skill 后，AI 会自动拥有该领域的专业知识：

```bash
You: /load Rust Programming
✓ Loaded skill: Rust Programming

You: 如何在 Rust 中处理错误？
🤖 Scode: 在 Rust 中，错误处理主要使用 Result 类型：

\`\`\`rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// 使用 ? 操作符传播错误
fn process() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_file("data.txt")?;
    Ok(())
}
\`\`\`

这是 Rust 的最佳实践，遵循所有权和借用规则...
```

## 创建自定义 Skill

### 1. 创建 Skill 文件

```bash
cd ~/.scode/skills
nano python.md
```

### 2. 编写 Skill 内容

```markdown
---
name: Python Programming
description: Python best practices and patterns
category: programming
tags: [python, scripting, data-science]
version: 1.0.0
---

# Python Programming Skill

## Core Principles

- **Pythonic Code**: Write idiomatic Python
- **PEP 8**: Follow style guide
- **Type Hints**: Use for better code quality

## Common Patterns

### List Comprehensions
\`\`\`python
# Instead of:
result = []
for item in items:
    if condition(item):
        result.append(transform(item))

# Use:
result = [transform(item) for item in items if condition(item)]
\`\`\`

### Context Managers
\`\`\`python
with open('file.txt', 'r') as f:
    content = f.read()
\`\`\`

## Best Practices

1. Use virtual environments
2. Write docstrings
3. Use type hints
4. Follow PEP 8
5. Write tests with pytest
```

### 3. 重启 Scode

```bash
./run.sh
```

### 4. 加载自定义 Skill

```bash
You: /skills
ℹ Available Skills (4):
  [ ] Python Programming - Python best practices and patterns
      Category: programming | Tags: python, scripting, data-science
  ...

You: /load Python Programming
✓ Loaded skill: Python Programming
```

## Skill 工作原理

### 1. 加载时

```
User: /load Rust Programming
  ↓
SkillManager.activate_skill("Rust Programming")
  ↓
Skill 内容添加到 active_skills 列表
```

### 2. 对话时

```
User: 如何处理错误？
  ↓
Agent.get_system_prompt()
  ↓
Base Prompt + Active Skills Prompt
  ↓
AI 拥有 Rust 专业知识
  ↓
AI 回答基于 Rust 最佳实践
```

### 3. 卸载时

```
User: /unload Rust Programming
  ↓
SkillManager.deactivate_skill("Rust Programming")
  ↓
Skill 从 active_skills 移除
  ↓
AI 不再拥有该 skill 的知识
```

## 命令列表

| 命令 | 功能 | 示例 |
|------|------|------|
| /skills | 列出所有可用 skills | `/skills` |
| /load | 加载一个 skill | `/load Rust Programming` |
| /unload | 卸载 skill | `/unload` |

## 使用场景

### 场景 1：Rust 项目开发

```bash
# 加载 Rust skill
You: /load Rust Programming

# AI 现在是 Rust 专家
You: 帮我优化这段代码的性能
🤖 Scode: [使用 Rust 最佳实践优化]

You: 如何避免内存泄漏？
🤖 Scode: [基于 Rust 所有权系统解释]
```

### 场景 2：Web 开发

```bash
# 加载 Web skill
You: /load Web Development

# AI 现在是 Web 开发专家
You: 设计一个 RESTful API
🤖 Scode: [使用 REST 最佳实践设计]

You: 如何优化前端性能？
🤖 Scode: [基于 Web 性能优化知识]
```

### 场景 3：多 Skill 组合

```bash
# 加载多个 skills
You: /load Rust Programming
You: /load Web Development
You: /load Git Workflow

# AI 现在拥有多个领域的知识
You: 用 Rust 开发一个 Web API，并设置 Git 工作流
🤖 Scode: [结合三个 skill 的知识提供方案]
```

## 与 Claude Code 对比

| 功能 | Claude Code | Scode v0.6.0 |
|------|-------------|--------------|
| Skill 系统 | ✅ | ✅ |
| 动态加载 | ✅ | ✅ |
| Markdown 格式 | ✅ | ✅ |
| 自定义 Skill | ✅ | ✅ |
| 分类和标签 | ✅ | ✅ |
| 内置 Skills | 多个 | 3 个 |

## 优势

### 1. 模块化知识

- 按需加载，不占用不必要的上下文
- 每个 skill 独立管理
- 易于更新和维护

### 2. 动态切换

- 随时加载/卸载
- 适应不同任务需求
- 灵活组合

### 3. 易于扩展

- Markdown 格式，易于编写
- 支持自定义 skill
- 社区可以分享 skills

### 4. 提升专业性

- AI 拥有领域专业知识
- 遵循最佳实践
- 提供更准确的建议

## 版本演进

| 版本 | 工具 | 提示词 | Skills | 完成度 |
|------|------|--------|--------|--------|
| v0.5.2 | 22 | 6,700 字 | 0 | 80% |
| v0.6.0 | 22 | 6,700 字 | 3 | 85% ⭐ |

**新增：Skill 系统**

## 总结

**Scode v0.6.0 现在拥有 Claude Code 级别的 Skill 系统！**

**关键特性：**
- ✅ 3 个内置 skills
- ✅ 动态加载/卸载
- ✅ Markdown 格式
- ✅ 自定义 skill 支持
- ✅ 分类和标签
- ✅ 即时生效

**使用流程：**
1. `/skills` - 查看可用 skills
2. `/load <name>` - 加载 skill
3. 正常对话 - AI 拥有专业知识
4. `/unload` - 卸载 skill

**Scode v0.6.0 - 专业、灵活、可扩展！** 🎓
