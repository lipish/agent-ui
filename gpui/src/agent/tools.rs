// Agent tools

use anyhow::Result;
use serde_json::json;
use super::types::ToolDefinition;

/// Tool trait
pub trait Tool: Send + Sync {
    /// Tool name
    fn name(&self) -> &str;
    
    /// Tool description
    fn description(&self) -> &str;
    
    /// Tool parameters schema
    fn parameters(&self) -> serde_json::Value;
    
    /// Execute the tool
    fn execute(&self, arguments: serde_json::Value) -> Result<String>;
}

/// Get all available tools
pub fn get_available_tools() -> Vec<ToolDefinition> {
    vec![
        // File system tools
        ToolDefinition {
            name: "read_file".to_string(),
            description: "Read the contents of a file".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The path to the file to read"
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "write_file".to_string(),
            description: "Write content to a file".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The path to the file to write"
                    },
                    "content": {
                        "type": "string",
                        "description": "The content to write to the file"
                    }
                },
                "required": ["path", "content"]
            }),
        },
        ToolDefinition {
            name: "list_files".to_string(),
            description: "List files in a directory".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The directory path to list"
                    }
                },
                "required": ["path"]
            }),
        },
        
        // Code analysis tools
        ToolDefinition {
            name: "search_code".to_string(),
            description: "Search for code patterns in the codebase".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query"
                    },
                    "path": {
                        "type": "string",
                        "description": "Optional path to search in"
                    }
                },
                "required": ["query"]
            }),
        },
        
        // Shell execution
        ToolDefinition {
            name: "run_command".to_string(),
            description: "Run a shell command".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The command to run"
                    },
                    "cwd": {
                        "type": "string",
                        "description": "Working directory (optional)"
                    }
                },
                "required": ["command"]
            }),
        },
    ]
}

/// Execute a tool by name
pub fn execute_tool(name: &str, arguments: serde_json::Value) -> Result<String> {
    match name {
        "read_file" => execute_read_file(arguments),
        "write_file" => execute_write_file(arguments),
        "list_files" => execute_list_files(arguments),
        "search_code" => execute_search_code(arguments),
        "run_command" => execute_run_command(arguments),
        _ => Err(anyhow::anyhow!("Unknown tool: {}", name)),
    }
}

fn execute_read_file(args: serde_json::Value) -> Result<String> {
    let path = args["path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing path"))?;
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

fn execute_write_file(args: serde_json::Value) -> Result<String> {
    let path = args["path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing path"))?;
    let content = args["content"].as_str().ok_or_else(|| anyhow::anyhow!("Missing content"))?;
    std::fs::write(path, content)?;
    Ok(format!("Successfully wrote to {}", path))
}

fn execute_list_files(args: serde_json::Value) -> Result<String> {
    let path = args["path"].as_str().ok_or_else(|| anyhow::anyhow!("Missing path"))?;
    let entries = std::fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    Ok(entries.join("\n"))
}

fn execute_search_code(args: serde_json::Value) -> Result<String> {
    let query = args["query"].as_str().ok_or_else(|| anyhow::anyhow!("Missing query"))?;
    // TODO: Implement actual code search
    Ok(format!("Searching for: {}", query))
}

fn execute_run_command(args: serde_json::Value) -> Result<String> {
    let command = args["command"].as_str().ok_or_else(|| anyhow::anyhow!("Missing command"))?;
    let cwd = args["cwd"].as_str();
    
    let mut cmd = std::process::Command::new("sh");
    cmd.arg("-c").arg(command);
    
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    
    let output = cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if output.status.success() {
        Ok(stdout.to_string())
    } else {
        Ok(format!("Error:\n{}\n{}", stdout, stderr))
    }
}

