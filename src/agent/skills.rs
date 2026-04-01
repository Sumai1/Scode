use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub version: String,
}

impl Skill {
    pub fn from_markdown(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_markdown_content(&content)
    }

    pub fn from_markdown_content(content: &str) -> Result<Self> {
        // Parse frontmatter (YAML between --- markers)
        let mut name = "unknown".to_string();
        let mut description = String::new();
        let mut category = "general".to_string();
        let mut tags = Vec::new();
        let mut version = "1.0.0".to_string();
        let mut skill_content = content.to_string();

        if content.starts_with("---") {
            if let Some(end) = content[3..].find("---") {
                let frontmatter = &content[3..end + 3];
                skill_content = content[end + 6..].trim().to_string();

                // Simple YAML parsing
                for line in frontmatter.lines() {
                    let line = line.trim();
                    if let Some((key, value)) = line.split_once(':') {
                        let key = key.trim();
                        let value = value.trim();
                        match key {
                            "name" => name = value.to_string(),
                            "description" => description = value.to_string(),
                            "category" => category = value.to_string(),
                            "version" => version = value.to_string(),
                            "tags" => {
                                tags = value
                                    .trim_matches(|c| c == '[' || c == ']')
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .collect();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(Self {
            name,
            description,
            content: skill_content,
            category,
            tags,
            version,
        })
    }

    pub fn to_prompt_section(&self) -> String {
        format!(
            r#"
# Skill: {}

{}

{}
"#,
            self.name, self.description, self.content
        )
    }
}

pub struct SkillManager {
    skills: HashMap<String, Skill>,
    active_skills: Vec<String>,
    skills_dir: PathBuf,
}

impl SkillManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        let skills_dir = home.join(".scode").join("skills");
        
        Ok(Self {
            skills: HashMap::new(),
            active_skills: Vec::new(),
            skills_dir,
        })
    }

    pub async fn init(&mut self) -> Result<()> {
        // Create skills directory if it doesn't exist
        fs::create_dir_all(&self.skills_dir).await?;
        
        // Create default skills
        self.create_default_skills().await?;
        
        // Load all skills
        self.load_all_skills().await?;
        
        Ok(())
    }

    async fn create_default_skills(&self) -> Result<()> {
        // Rust skill
        let rust_skill = r#"---
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
- **Lifetimes**: Ensure references are valid
- **Zero-cost abstractions**: High-level features with no runtime overhead

## Common Patterns

### Error Handling
```rust
// Use Result for recoverable errors
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

// Use ? operator for error propagation
fn process() -> Result<(), Box<dyn std::error::Error>> {
    let content = read_file("data.txt")?;
    Ok(())
}
```

### Option Handling
```rust
// Use Option for nullable values
fn find_user(id: u32) -> Option<User> {
    // ...
}

// Pattern matching
match find_user(42) {
    Some(user) => println!("Found: {}", user.name),
    None => println!("Not found"),
}

// Combinators
let name = find_user(42)
    .map(|u| u.name)
    .unwrap_or_else(|| "Unknown".to_string());
```

### Async/Await
```rust
// Async functions
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}

// Tokio runtime
#[tokio::main]
async fn main() {
    let data = fetch_data("https://api.example.com").await;
}
```

## Best Practices

1. **Use `&str` for string slices, `String` for owned strings**
2. **Prefer `impl Trait` for return types when possible**
3. **Use `#[derive]` for common traits**
4. **Leverage the type system for compile-time guarantees**
5. **Use `cargo clippy` for linting**
6. **Use `cargo fmt` for formatting**
7. **Write tests with `#[cfg(test)]`**

## Common Crates

- **serde**: Serialization/deserialization
- **tokio**: Async runtime
- **anyhow**: Error handling
- **clap**: CLI argument parsing
- **reqwest**: HTTP client
- **sqlx**: SQL database access

## Memory Safety

- No null pointers (use `Option`)
- No data races (enforced by borrow checker)
- No use-after-free (enforced by ownership)
- No buffer overflows (bounds checking)
"#;

        let rust_path = self.skills_dir.join("rust.md");
        if !rust_path.exists() {
            fs::write(&rust_path, rust_skill).await?;
        }

        // Web Development skill
        let web_skill = r#"---
name: Web Development
description: Modern web development practices and patterns
category: web
tags: [web, frontend, backend, api]
version: 1.0.0
---

# Web Development Skill

## Frontend Best Practices

### React/Vue/Svelte
- Component-based architecture
- State management (Redux, Vuex, Svelte stores)
- Hooks for side effects
- Virtual DOM optimization

### CSS/Styling
- Use CSS modules or CSS-in-JS
- Mobile-first responsive design
- CSS Grid and Flexbox
- CSS variables for theming

### Performance
- Code splitting and lazy loading
- Image optimization
- Minimize bundle size
- Use CDN for static assets

## Backend Best Practices

### RESTful API Design
```
GET    /api/users       - List users
GET    /api/users/:id   - Get user
POST   /api/users       - Create user
PUT    /api/users/:id   - Update user
DELETE /api/users/:id   - Delete user
```

### HTTP Status Codes
- 200 OK - Success
- 201 Created - Resource created
- 400 Bad Request - Invalid input
- 401 Unauthorized - Authentication required
- 403 Forbidden - No permission
- 404 Not Found - Resource not found
- 500 Internal Server Error - Server error

### Security
- Use HTTPS everywhere
- Validate and sanitize all inputs
- Use parameterized queries (prevent SQL injection)
- Implement rate limiting
- Use CORS properly
- Store passwords with bcrypt/argon2
- Use JWT or sessions for authentication

## Database Patterns

### SQL Best Practices
- Use indexes for frequently queried columns
- Normalize data to reduce redundancy
- Use transactions for atomic operations
- Avoid N+1 queries

### NoSQL Best Practices
- Denormalize for read performance
- Use appropriate data models
- Plan for scalability
- Implement proper indexing

## API Design

### GraphQL
- Single endpoint
- Client specifies data requirements
- Strong typing
- Efficient data fetching

### REST
- Resource-based URLs
- HTTP methods for operations
- Stateless communication
- Versioning (v1, v2)

## Testing

- Unit tests for business logic
- Integration tests for APIs
- E2E tests for critical flows
- Test coverage > 80%
"#;

        let web_path = self.skills_dir.join("web.md");
        if !web_path.exists() {
            fs::write(&web_path, web_skill).await?;
        }

        // Git workflow skill
        let git_skill = r#"---
name: Git Workflow
description: Professional Git workflow and best practices
category: tools
tags: [git, version-control, workflow]
version: 1.0.0
---

# Git Workflow Skill

## Commit Message Format

### Conventional Commits
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting)
- **refactor**: Code refactoring
- **test**: Adding or updating tests
- **chore**: Maintenance tasks

### Examples
```
feat(auth): add JWT authentication

Implement JWT-based authentication system with refresh tokens.

Closes #123
```

```
fix(api): resolve memory leak in user service

The user service was not properly closing database connections.

Fixes #456
```

## Branching Strategy

### Git Flow
- `main` - Production-ready code
- `develop` - Integration branch
- `feature/*` - New features
- `hotfix/*` - Production fixes
- `release/*` - Release preparation

### GitHub Flow (Simpler)
- `main` - Always deployable
- `feature/*` - Feature branches
- Pull requests for code review
- Deploy from main

## Best Practices

1. **Commit Often**: Small, focused commits
2. **Write Clear Messages**: Explain what and why
3. **Review Before Commit**: Use `git diff`
4. **Pull Before Push**: Stay updated
5. **Use Branches**: Isolate changes
6. **Rebase vs Merge**: Choose based on team preference
7. **Tag Releases**: Use semantic versioning

## Useful Commands

```bash
# Interactive rebase (clean up history)
git rebase -i HEAD~3

# Amend last commit
git commit --amend

# Stash changes
git stash
git stash pop

# Cherry-pick commit
git cherry-pick <commit-hash>

# Reset to previous state
git reset --hard HEAD~1

# View history
git log --oneline --graph --all
```

## Code Review Checklist

- [ ] Code follows style guide
- [ ] Tests are included
- [ ] Documentation is updated
- [ ] No sensitive data in commits
- [ ] Commit messages are clear
- [ ] No merge conflicts
"#;

        let git_path = self.skills_dir.join("git.md");
        if !git_path.exists() {
            fs::write(&git_path, git_skill).await?;
        }

        Ok(())
    }

    async fn load_all_skills(&mut self) -> Result<()> {
        let mut entries = fs::read_dir(&self.skills_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                match Skill::from_markdown(&path) {
                    Ok(skill) => {
                        self.skills.insert(skill.name.clone(), skill);
                    }
                    Err(e) => {
                        eprintln!("Failed to load skill from {:?}: {}", path, e);
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn list_skills(&self) -> Vec<&Skill> {
        self.skills.values().collect()
    }

    pub fn get_skill(&self, name: &str) -> Option<&Skill> {
        self.skills.get(name)
    }

    pub fn activate_skill(&mut self, name: &str) -> Result<()> {
        if !self.skills.contains_key(name) {
            anyhow::bail!("Skill not found: {}", name);
        }
        
        if !self.active_skills.contains(&name.to_string()) {
            self.active_skills.push(name.to_string());
        }
        
        Ok(())
    }

    pub fn deactivate_skill(&mut self, name: &str) {
        self.active_skills.retain(|s| s != name);
    }

    pub fn get_active_skills_prompt(&self) -> String {
        if self.active_skills.is_empty() {
            return String::new();
        }

        let mut prompt = String::from("\n# Active Skills\n\n");
        prompt.push_str("You have access to the following specialized knowledge:\n\n");

        for skill_name in &self.active_skills {
            if let Some(skill) = self.skills.get(skill_name) {
                prompt.push_str(&skill.to_prompt_section());
                prompt.push_str("\n---\n");
            }
        }

        prompt
    }

    pub fn get_active_skills(&self) -> &[String] {
        &self.active_skills
    }

    pub async fn install_skill_from_url(&mut self, url: &str) -> Result<String> {
        // Download skill from URL
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Failed to download skill: HTTP {}", response.status());
        }
        
        let content = response.text().await?;
        
        // Parse skill to get name
        let skill = Skill::from_markdown_content(&content)?;
        
        // Save to skills directory
        let skill_path = self.skills_dir.join(format!("{}.md", skill.name.to_lowercase().replace(" ", "-")));
        fs::write(&skill_path, content).await?;
        
        // Load the skill
        self.skills.insert(skill.name.clone(), skill.clone());
        
        Ok(skill.name)
    }

    pub async fn install_skill_from_github(&mut self, repo: &str) -> Result<String> {
        // Format: username/repo or username/repo/path/to/skill.md
        let parts: Vec<&str> = repo.split('/').collect();
        
        if parts.len() < 2 {
            anyhow::bail!("Invalid GitHub repo format. Use: username/repo or username/repo/path/to/skill.md");
        }
        
        let url = if parts.len() == 2 {
            // Assume skill.md in root
            format!("https://raw.githubusercontent.com/{}/{}/main/skill.md", parts[0], parts[1])
        } else {
            // Full path provided
            format!("https://raw.githubusercontent.com/{}", repo)
        };
        
        self.install_skill_from_url(&url).await
    }

    pub async fn uninstall_skill(&mut self, name: &str) -> Result<()> {
        // Remove from active skills
        self.deactivate_skill(name);
        
        // Remove from skills map
        if let Some(skill) = self.skills.remove(name) {
            // Delete file
            let skill_path = self.skills_dir.join(format!("{}.md", skill.name.to_lowercase().replace(" ", "-")));
            if skill_path.exists() {
                fs::remove_file(skill_path).await?;
            }
            Ok(())
        } else {
            anyhow::bail!("Skill not found: {}", name)
        }
    }

    pub fn clear_active_skills(&mut self) {
        self.active_skills.clear();
    }
}

impl Default for SkillManager {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
