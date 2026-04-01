# 🎓 Scode v0.6.1 - Skill 安装功能

## 新增功能

### ✅ Skill 安装系统

**现在可以从任何地方安装 skills！**

- 📥 从 URL 安装
- 🐙 从 GitHub 安装
- 🗑️ 卸载 skills
- 🔄 自动下载和配置

## 使用方法

### 1. 从 URL 安装

```bash
./run.sh

You: /install
ℹ Install Skill
   Options:
   1. From URL: /install https://example.com/skill.md
   2. From GitHub: /install username/repo
   3. From GitHub path: /install username/repo/path/to/skill.md

Enter URL or GitHub repo: https://raw.githubusercontent.com/user/repo/main/python.md
ℹ Installing skill...
✓ Installed skill: Python Programming
   Use /load Python Programming to activate it
```

### 2. 从 GitHub 安装

**简短格式（假设 skill.md 在根目录）：**
```bash
You: /install
Enter URL or GitHub repo: username/repo
ℹ Installing skill...
✓ Installed skill: Skill Name
```

**完整路径：**
```bash
You: /install
Enter URL or GitHub repo: username/repo/skills/python.md
ℹ Installing skill...
✓ Installed skill: Python Programming
```

### 3. 卸载 Skill

```bash
You: /remove
ℹ Installed skills:
   1. Rust Programming - Expert knowledge of Rust
   2. Web Development - Modern web development
   3. Python Programming - Python best practices
Enter skill name to remove: Python Programming
✓ Removed skill: Python Programming
```

## 完整工作流程

### 示例：安装 Python Skill

```bash
# 1. 查看当前 skills
You: /skills
ℹ Available Skills (3):
  [ ] Rust Programming
  [ ] Web Development
  [ ] Git Workflow

# 2. 安装 Python skill
You: /install
Enter URL or GitHub repo: https://example.com/python-skill.md
ℹ Installing skill...
✓ Installed skill: Python Programming

# 3. 查看更新后的 skills
You: /skills
ℹ Available Skills (4):
  [ ] Rust Programming
  [ ] Web Development
  [ ] Git Workflow
  [ ] Python Programming  ⭐ 新安装

# 4. 加载并使用
You: /load Python Programming
✓ Loaded skill: Python Programming

You: 如何使用 Python 列表推导式？
🤖 Scode: [基于 Python skill 的专业回答]

# 5. 如果不需要了，可以卸载
You: /remove
Enter skill name to remove: Python Programming
✓ Removed skill: Python Programming
```

## Skill 文件格式

创建自己的 skill 文件：

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

## 分享 Skills

### 1. 创建 Skill 仓库

```bash
# 在 GitHub 创建仓库
mkdir my-scode-skills
cd my-scode-skills

# 创建 skill 文件
cat > python.md << 'EOF'
---
name: Python Programming
description: Python best practices
category: programming
tags: [python]
version: 1.0.0
---

# Python Programming Skill
...
EOF

# 提交到 GitHub
git init
git add python.md
git commit -m "Add Python skill"
git push origin main
```

### 2. 分享给其他人

```bash
# 其他人可以这样安装：
You: /install
Enter URL or GitHub repo: your-username/my-scode-skills
✓ Installed skill: Python Programming
```

## 命令列表

| 命令 | 功能 | 示例 |
|------|------|------|
| /skills | 列出所有 skills | `/skills` |
| /load | 加载 skill | `/load Rust Programming` |
| /unload | 卸载 skill | `/unload` |
| /install | 安装 skill | `/install` |
| /remove | 删除 skill | `/remove` |

## 使用场景

### 场景 1：安装社区 Skill

```bash
# 发现一个很棒的 TypeScript skill
You: /install
Enter URL or GitHub repo: awesome-dev/typescript-skill
✓ Installed skill: TypeScript Programming

You: /load TypeScript Programming
✓ Loaded skill: TypeScript Programming

You: 帮我写一个 TypeScript 接口
🤖 Scode: [基于 TypeScript skill 的专业回答]
```

### 场景 2：创建团队 Skills

```bash
# 团队创建了内部最佳实践 skill
You: /install
Enter URL or GitHub repo: company/internal-best-practices
✓ Installed skill: Company Best Practices

You: /load Company Best Practices
✓ Loaded skill: Company Best Practices

# 现在 AI 了解公司的编码规范
You: 按照公司规范重构这段代码
🤖 Scode: [基于公司规范的重构建议]
```

### 场景 3：临时使用特定 Skill

```bash
# 需要处理 Docker 相关任务
You: /install
Enter URL or GitHub repo: devops/docker-skill
✓ Installed skill: Docker & Containers

You: /load Docker & Containers
✓ Loaded skill: Docker & Containers

# 完成任务后卸载
You: /unload
You: /remove
Enter skill name to remove: Docker & Containers
✓ Removed skill: Docker & Containers
```

## 技术实现

### 安装流程

```
User: /install username/repo
  ↓
SkillManager::install_skill_from_github()
  ↓
构建 GitHub raw URL
  ↓
下载 skill 文件
  ↓
解析 frontmatter
  ↓
保存到 ~/.scode/skills/
  ↓
加载到 skills map
  ↓
✓ 安装完成
```

### 文件存储

```
~/.scode/skills/
├── rust-programming.md      # 内置
├── web-development.md        # 内置
├── git-workflow.md           # 内置
├── python-programming.md     # 安装的
└── typescript-programming.md # 安装的
```

## 版本对比

| 功能 | v0.6.0 | v0.6.1 | 变化 |
|------|--------|--------|------|
| Skills | 3 | 3 | - |
| 安装功能 | ❌ | ✅ | 新增 ⭐ |
| 从 URL 安装 | ❌ | ✅ | 新增 |
| 从 GitHub 安装 | ❌ | ✅ | 新增 |
| 卸载功能 | ❌ | ✅ | 新增 |
| 命令数量 | 13 | 15 | +2 |

## 优势

### 1. 灵活性

- 从任何地方安装 skills
- 不限于内置 skills
- 支持自定义和社区 skills

### 2. 可扩展性

- 社区可以创建和分享 skills
- 团队可以维护内部 skills
- 个人可以创建专属 skills

### 3. 易用性

- 简单的命令
- 自动下载和配置
- 即装即用

### 4. 管理性

- 随时安装/卸载
- 不占用不必要的空间
- 清晰的 skill 列表

## 与 Claude Code 对比

| 功能 | Claude Code | Scode v0.6.1 |
|------|-------------|--------------|
| Skill 系统 | ✅ | ✅ |
| 内置 Skills | 多个 | 3 个 |
| 安装 Skills | ✅ | ✅ ⭐ |
| 从 URL 安装 | ✅ | ✅ ⭐ |
| 从 GitHub 安装 | ✅ | ✅ ⭐ |
| 社区 Skills | ✅ | ✅ ⭐ |

## 总结

**Scode v0.6.1 现在支持从任何地方安装 skills！**

**关键特性：**
- ✅ 从 URL 安装
- ✅ 从 GitHub 安装
- ✅ 卸载功能
- ✅ 自动下载和配置
- ✅ 社区 skill 支持

**使用流程：**
1. `/install` - 安装 skill
2. `/load <name>` - 加载 skill
3. 正常对话 - AI 拥有专业知识
4. `/remove` - 卸载 skill

**Scode v0.6.1 - 更灵活，更可扩展！** 🎓
