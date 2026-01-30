// OpenCode HTTP Client for OpenCodeMonitor
// Communicates with OpenCode server at localhost:4096

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest::{Client, Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub model: Option<String>,
    pub agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: String,
    pub content: String,
    pub parts: Vec<MessagePart>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePart {
    pub kind: String,
    pub content: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub path: String,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub diff: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub healthy: bool,
    pub version: String,
}

pub struct OpenCodeClient {
    base_url: String,
    http: Client,
    sessions: Arc<Mutex<Vec<Session>>>,
}

impl OpenCodeClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: Client::new(),
            sessions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn health(&self) -> Result<HealthResponse, Error> {
        self.http
            .get(&format!("{}/global/health", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list_sessions(&self) -> Result<Vec<Session>, Error> {
        let response = self.http
            .get(&format!("{}/session", self.base_url))
            .send()
            .await?
            .json::<Vec<Session>>()
            .await?;
        
        let mut sessions = self.sessions.lock().await;
        *sessions = response.clone();
        Ok(response)
    }

    pub async fn create_session(&self, title: Option<&str>) -> Result<Session, Error> {
        let body = json!({
            "title": title.unwrap_or("New Session")
        });
        
        let response = self.http
            .post(&format!("{}/session", self.base_url))
            .json(&body)
            .send()
            .await?
            .json::<Session>()
            .await?;
        
        let mut sessions = self.sessions.lock().await;
        sessions.push(response.clone());
        Ok(response)
    }

    pub async fn send_message(&self, session_id: &str, message: &str, model: Option<&str>) -> Result<Vec<Message>, Error> {
        let body = json!({
            "message": message,
            "model": model
        });
        
        self.http
            .post(&format!("{}/session/{}/message", self.base_url, session_id))
            .json(&body)
            .send()
            .await?
            .json::<Vec<Message>>()
            .await
    }

    pub async fn get_messages(&self, session_id: &str, limit: Option<i32>) -> Result<Vec<Message>, Error> {
        let mut url = format!("{}/session/{}/message", self.base_url, session_id);
        if let Some(l) = limit {
            url.push_str(&format!("?limit={}", l));
        }
        
        self.http
            .get(&url)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_diffs(&self, session_id: &str) -> Result<Vec<FileDiff>, Error> {
        self.http
            .get(&format!("{}/session/{}/diff", self.base_url, session_id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn abort_session(&self, session_id: &str) -> Result<bool, Error> {
        self.http
            .post(&format!("{}/session/{}/abort", self.base_url, session_id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn delete_session(&self, session_id: &str) -> Result<bool, Error> {
        self.http
            .delete(&format!("{}/session/{}", self.base_url, session_id))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn search_files(&self, pattern: &str) -> Result<Vec<String>, Error> {
        self.http
            .get(&format!("{}/find?pattern={}", self.base_url, pattern))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn read_file(&self, path: &str) -> Result<String, Error> {
        self.http
            .get(&format!("{}/file/content?path={}", self.base_url, path))
            .send()
            .await?
            .text()
            .await
    }

    pub async fn list_files(&self, path: &str) -> Result<serde_json::Value, Error> {
        self.http
            .get(&format!("{}/file?path={}", self.base_url, path))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn run_shell(&self, session_id: &str, command: &str, agent: &str) -> Result<Vec<Message>, Error> {
        let body = json!({
            "command": command,
            "agent": agent
        });
        
        self.http
            .post(&format!("{}/session/{}/shell", self.base_url, session_id))
            .json(&body)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn list_agents(&self) -> Result<Vec<Agent>, Error> {
        self.http
            .get(&format!("{}/agent", self.base_url))
            .send()
            .await?
            .json()
            .await
    }

    // events function requires tokio-stream - temporarily disabled
    // pub async fn events(&self) -> impl tokio_stream::Stream<Item = Result<serde_json::Value, reqwest::Error>> {
    //     let response = self
    //         .http
    //         .get(&format!("{}/event", self.base_url))
    //         .send()
    //         .await
    //         .unwrap();
    //     
    //     stream::iter(
    //         response.bytes_stream()
    //             .map(|chunk| {
    //                 let chunk = chunk.unwrap();
    //                 let line = String::from_utf8_lossy(&chunk);
    //                 Ok(serde_json::from_str::<serde_json::Value>(&line).unwrap_or_else(|_| json!({ "raw": line.trim().to_string() })))
    //             })
    //     )
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health() {
        let client = OpenCodeClient::new("http://localhost:4096");
        let health = client.health().await;
        assert!(health.is_ok());
        let h = health.unwrap();
        assert!(h.healthy);
        println!("OpenCode version: {}", h.version);
    }

    #[tokio::test]
    async fn test_list_sessions() {
        let client = OpenCodeClient::new("http://localhost:4096");
        let sessions = client.list_sessions().await;
        println!("Sessions: {:?}", sessions);
        assert!(sessions.is_ok());
    }
}
