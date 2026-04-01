use super::base::Tool;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct WebFetchTool {
    client: reqwest::Client,
}

impl WebFetchTool {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Tool for WebFetchTool {
    fn name(&self) -> &str {
        "web_fetch"
    }

    fn description(&self) -> &str {
        "Fetch content from a URL. Returns the response body as text."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "URL to fetch"
                }
            },
            "required": ["url"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let url = input["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing url field"))?;

        let response = self.client.get(url).send().await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Ok(format!("HTTP {}: {}", status, text));
        }

        // Limit response size
        if text.len() > 50_000 {
            Ok(format!("{}... (truncated, total {} bytes)", &text[..50_000], text.len()))
        } else {
            Ok(text)
        }
    }
}

pub struct WebSearchTool {
    client: reqwest::Client,
}

impl WebSearchTool {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Search the web using DuckDuckGo. Returns search results with titles and snippets."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                },
                "max_results": {
                    "type": "number",
                    "description": "Maximum number of results (default: 5)"
                }
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let query = input["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query field"))?;
        let max_results = input["max_results"].as_u64().unwrap_or(5) as usize;

        // Use DuckDuckGo HTML search
        let url = format!("https://html.duckduckgo.com/html/?q={}", 
            urlencoding::encode(query));

        let response = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await?;

        let html = response.text().await?;

        // Simple HTML parsing to extract results
        let mut results = Vec::new();
        
        // Look for result divs (simplified parsing)
        for line in html.lines() {
            if line.contains("result__title") && results.len() < max_results {
                // Extract title (very basic)
                if let Some(start) = line.find(">") {
                    if let Some(end) = line[start..].find("</a>") {
                        let title = &line[start+1..start+end];
                        let title = title.replace("<b>", "").replace("</b>", "");
                        results.push(title);
                    }
                }
            }
        }

        if results.is_empty() {
            Ok(format!("No results found for: {}", query))
        } else {
            Ok(format!("Search results for '{}':\n\n{}", 
                query,
                results.iter()
                    .enumerate()
                    .map(|(i, r)| format!("{}. {}", i + 1, r))
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
        }
    }
}

pub struct HttpRequestTool {
    client: reqwest::Client,
}

impl HttpRequestTool {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Tool for HttpRequestTool {
    fn name(&self) -> &str {
        "http_request"
    }

    fn description(&self) -> &str {
        "Make HTTP requests (GET, POST, PUT, DELETE) with custom headers and body."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "enum": ["GET", "POST", "PUT", "DELETE"],
                    "description": "HTTP method"
                },
                "url": {
                    "type": "string",
                    "description": "URL to request"
                },
                "headers": {
                    "type": "object",
                    "description": "Optional headers"
                },
                "body": {
                    "type": "string",
                    "description": "Optional request body"
                }
            },
            "required": ["method", "url"]
        })
    }

    async fn execute(&self, input: Value) -> Result<String> {
        let method = input["method"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing method field"))?;
        let url = input["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing url field"))?;

        let mut request = match method {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            _ => anyhow::bail!("Invalid method: {}", method),
        };

        // Add headers
        if let Some(headers) = input["headers"].as_object() {
            for (key, value) in headers {
                if let Some(v) = value.as_str() {
                    request = request.header(key, v);
                }
            }
        }

        // Add body
        if let Some(body) = input["body"].as_str() {
            request = request.body(body.to_string());
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;

        Ok(format!("Status: {}\n\nBody:\n{}", status, text))
    }

    fn requires_permission(&self) -> bool {
        true
    }
}
