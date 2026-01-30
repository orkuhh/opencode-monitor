// Pi Coding Agent Integration for OpenCodeMonitor
// Runs Pi CLI with gpt-5.2-codex and custom Copilot prompt

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::process::{Command, Child, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiConfig {
    pub model: String,
    pub thinking: String,
    pub system_prompt: String,
    pub provider: String,
}

impl Default for PiConfig {
    fn default() -> Self {
        Self {
            model: "gpt-5.2-codex".to_string(),
            thinking: "xhigh".to_string(),
            system_prompt: PiConfig::default_system_prompt(),
            provider: "github-copilot".to_string(),
        }
    }
}

impl PiConfig {
    pub fn default_system_prompt() -> String {
        r#"You are a coding agent based on GPT-5-Codex.

## Editing constraints
- Default to ASCII when editing or creating files. Only introduce non-ASCII or other Unicode characters when there is a clear justification and the file already uses them.
- Add succinct code comments that explain what is going on if code is not self-explanatory.
- You may be in a dirty git worktree.
  * NEVER revert existing changes you did not make unless explicitly requested.
  * If asked to make a commit or code edits and there are unrelated changes, don't revert them.
- Do not amend a commit unless explicitly requested to do so.
- **NEVER** use destructive commands like `git reset --hard` unless specifically requested.

## Exploration and reading files
- **Think first.** Before any tool call, decide ALL files/resources you will need.
- **Batch everything.** If you need multiple files, read them together.
- Use parallel tool calls when possible.
- Only make sequential calls if you truly cannot know the next file without seeing a result first.
- Always maximize parallelism.

## Tool use
- You have access to tools. If a tool exists to perform a specific task, you MUST use that tool instead of running a terminal command.
- Use the `bash` tool to run terminal commands.
- Use the `read` tool to read files.
- Use the `edit` tool to make edits to files.
- Use `grep` to search for strings in files.
- Use `find` or `ls` to list files and directories.

## Presenting your work
- Default: be very concise; friendly coding teammate tone.
- For substantial work, summarize clearly.
- For code changes: Lead with a quick explanation, then details on where and why.
- Use proper Markdown formatting.

## Final answer structure
- Markdown text. Use structure only when it helps scanability.
- Bullets: use - ; merge related points; keep to one line when possible.
- Code samples wrapped in fenced code blocks with language hints.
- Tone: collaborative, concise, factual; present tense, active voice.
"#.to_string()
    }
}

pub struct PiSession {
    pub id: String,
    pub config: PiConfig,
    pub process: Option<Child>,
    pub output: Arc<Mutex<Vec<String>>>,
}

impl PiSession {
    pub fn new(id: &str, config: Option<PiConfig>) -> Self {
        Self {
            id: id.to_string(),
            config: config.unwrap_or_default(),
            process: None,
            output: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn spawn(&mut self, prompt: &str, workdir: &str) -> Result<(), std::io::Error> {
        let mut cmd = Command::new("pi");
        
        // Configure Pi with our settings
        cmd.arg("--provider").arg(&self.config.provider);
        cmd.arg("--model").arg(&self.config.model);
        cmd.arg("--thinking").arg(&self.config.thinking);
        cmd.arg("-p").arg(prompt);
        
        // Set GitHub token if available
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            cmd.env("GITHUB_TOKEN", token);
        }
        
        // Set working directory
        cmd.current_dir(workdir);
        
        // Pipe output
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let mut child = cmd.spawn()?;
        self.process = Some(child);
        
        // Read output in background
        let output = self.output.clone();
        if let Some(stdout) = self.process.as_mut().unwrap().stdout.take() {
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                let mut lines = reader.lines();
                while let Some(line) = lines.next_line().await.ok().flatten() {
                    output.lock().await.push(line);
                }
            });
        }
        
        Ok(())
    }

    pub async fn wait(&mut self) -> Result<(), std::io::Error> {
        if let Some(proc) = self.process.as_mut() {
            proc.wait().await?;
        }
        Ok(())
    }

    pub async fn kill(&mut self) {
        if let Some(proc) = self.process.as_mut() {
            proc.kill().await.ok();
        }
    }

    pub fn is_running(&self) -> bool {
        match self.process {
            Some(ref proc) => proc.try_wait().ok().flatten().is_none(),
            None => false,
        }
    }

    pub async fn get_output(&self) -> Vec<String> {
        self.output.lock().await.clone()
    }
}

pub struct PiManager {
    sessions: Arc<Mutex<HashMap<String, PiSession>>>,
    default_config: Arc<Mutex<PiConfig>>,
}

impl PiManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            default_config: Arc::new(Mutex::new(PiConfig::default())),
        }
    }

    pub async fn create_session(&self, id: &str, config: Option<PiConfig>) {
        let mut sessions = self.sessions.lock().await;
        sessions.insert(id.to_string(), PiSession::new(id, config));
    }

    pub async fn run(&self, session_id: &str, prompt: &str, workdir: &str) -> Result<(), std::io::Error> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.spawn(prompt, workdir).await
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Session not found"))
        }
    }

    pub async fn wait(&self, session_id: &str) -> Result<(), std::io::Error> {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.wait().await
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Session not found"))
        }
    }

    pub async fn kill(&self, session_id: &str) {
        let mut sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.kill().await;
        }
    }

    pub async fn output(&self, session_id: &str) -> Vec<String> {
        let sessions = self.sessions.lock().await;
        if let Some(session) = sessions.get(session_id) {
            session.get_output().await
        } else {
            Vec::new()
        }
    }

    pub async fn list_models(&self) -> Result<Vec<String>, std::io::Error> {
        let output = Command::new("pi")
            .arg("--list-models")
            .output()
            .await?;
        
        let models = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect();
        
        Ok(models)
    }

    pub fn update_config(&self, config: PiConfig) {
        let mut default = self.default_config.lock().await;
        *default = config;
    }

    pub fn get_config(&self) -> PiConfig {
        let default = self.default_config.lock().await;
        default.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_models() {
        let manager = PiManager::new();
        let models = manager.list_models().await;
        assert!(models.is_ok());
        let m = models.unwrap();
        println!("Available models: {:?}", m.iter().take(5).collect::<Vec<_>>());
    }

    #[tokio::test]
    async fn test_default_config() {
        let config = PiConfig::default();
        println!("Default model: {}", config.model);
        println!("Default thinking: {}", config.thinking);
        assert_eq!(config.model, "gpt-5.2-codex");
        assert_eq!(config.thinking, "xhigh");
    }
}
