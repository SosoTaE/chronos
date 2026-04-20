use serde::{Deserialize, Serialize};

const OLLAMA_URL: &str = "http://localhost:11434";
const AI_MODEL: &str = "gemma3:4b";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: Option<OllamaMessageResponse>,
}

#[derive(Deserialize)]
struct OllamaMessageResponse {
    content: String,
}

pub async fn chat(history: Vec<ChatMessage>) -> Result<ChatMessage, String> {
    let client = reqwest::Client::new();

    let request = OllamaRequest {
        model: AI_MODEL.to_string(),
        messages: history,
        stream: false,
    };

    let response = client
        .post(format!("{}/api/chat", OLLAMA_URL))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;

    let body: OllamaResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    let content = body
        .message
        .map(|m| m.content)
        .unwrap_or_else(|| "No response from AI".to_string());

    Ok(ChatMessage {
        role: "assistant".to_string(),
        content,
    })
}

pub async fn check_health() -> Result<bool, String> {
    let client = reqwest::Client::new();
    match client.get(format!("{}/api/tags", OLLAMA_URL)).send().await {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => Ok(false),
    }
}

pub async fn analyze_task(title: &str, category: &str, estimated_mins: i32, actual_secs: i64) -> Result<String, String> {
    let actual_mins = actual_secs / 60;
    let prompt = format!(
        "Analyze this completed task briefly (3-4 sentences max):\n\
         Task: {}\n\
         Category: {}\n\
         Estimated: {} minutes\n\
         Actual: {} minutes\n\
         Give insights on time management and a short encouragement. No markdown or formatting.",
        title, category, estimated_mins, actual_mins
    );

    let result = chat(vec![ChatMessage {
        role: "user".to_string(),
        content: prompt,
    }])
    .await?;

    Ok(result.content)
}

pub async fn analyze_achievements(tasks_summary: &str) -> Result<String, String> {
    let prompt = format!(
        "You are a productivity AI assistant. Analyze these completed tasks and provide insights:\n\n\
         {}\n\n\
         Provide: 1) Overall productivity score 2) Time estimation accuracy 3) Category distribution insights \
         4) A motivational closing. Keep it concise, no markdown.",
        tasks_summary
    );

    let result = chat(vec![ChatMessage {
        role: "user".to_string(),
        content: prompt,
    }])
    .await?;

    Ok(result.content)
}

pub async fn generate_nudge_message(processes: &[String]) -> Result<String, String> {
    let process_list = processes.join(", ");
    let prompt = format!(
        "I detected you're actively using these tools: [{}].\n\
         However, no timer is running to track your work.\n\
         Generate a brief, friendly, and encouraging message (max 25 words)\n\
         to nudge me to start tracking time. Make it sound helpful, not pushy.\n\
         Focus on the benefits of time tracking.",
        process_list
    );

    let result = chat(vec![ChatMessage {
        role: "user".to_string(),
        content: prompt,
    }])
    .await?;

    Ok(result.content)
}
