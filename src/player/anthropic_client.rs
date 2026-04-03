//! Thin HTTP client for the Anthropic Messages API.
//!
//! Speaks the same `/v1/messages` protocol that both the real Anthropic API and
//! llamafile/llama.cpp expose. Includes llamafile-specific extensions (`id_slot`,
//! `cache_prompt`) that are skipped when not set.

use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Anthropic API version header value.
const ANTHROPIC_VERSION: &str = "2023-06-01";

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// A complete Messages API request.
#[derive(Debug, Clone, Serialize)]
pub struct MessagesRequest {
    pub model: String,
    pub max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDef>,
    /// Explicitly disable streaming (llamafile may default to streaming otherwise).
    pub stream: bool,
    // -- llamafile extensions (omitted when None) --
    /// Assign this request to a specific KV cache slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_slot: Option<usize>,
    /// Reuse the KV cache from a previous request in this slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_prompt: Option<bool>,
}

impl MessagesRequest {
    pub fn new(model: impl Into<String>, max_tokens: u32) -> Self {
        Self {
            model: model.into(),
            max_tokens,
            system: None,
            messages: Vec::new(),
            tools: Vec::new(),
            stream: false,
            id_slot: None,
            cache_prompt: None,
        }
    }
}

/// A single message in the conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<ContentBlock>,
}

impl Message {
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: vec![ContentBlock::Text { text: text.into() }],
        }
    }

    pub fn assistant_tool_use(
        id: impl Into<String>,
        name: impl Into<String>,
        input: serde_json::Value,
    ) -> Self {
        Self {
            role: Role::Assistant,
            content: vec![ContentBlock::ToolUse {
                id: id.into(),
                name: name.into(),
                input,
            }],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// A content block within a message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    ToolResult {
        tool_use_id: String,
        content: String,
    },
}

/// A tool definition for the Messages API.
#[derive(Debug, Clone, Serialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

/// The response from a Messages API call.
#[derive(Debug, Clone, Deserialize)]
pub struct MessagesResponse {
    #[serde(default)]
    pub id: String,
    pub content: Vec<ContentBlock>,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub stop_reason: Option<String>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    #[serde(default)]
    pub input_tokens: u32,
    #[serde(default)]
    pub output_tokens: u32,
}

/// An error returned by the API.
#[derive(Debug)]
pub struct ApiError {
    pub status: Option<u16>,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(status) = self.status {
            write!(f, "API error {}: {}", status, self.message)
        } else {
            write!(f, "API error: {}", self.message)
        }
    }
}

impl std::error::Error for ApiError {}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

/// A minimal client for the Anthropic Messages API.
///
/// Works with both the real Anthropic API and local llamafile/llama.cpp servers
/// that expose `/v1/messages`.
pub struct AnthropicClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl AnthropicClient {
    pub fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> Arc<Self> {
        let http = reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("failed to build HTTP client");
        Arc::new(Self {
            http,
            base_url: base_url.into(),
            api_key: api_key.into(),
            model: model.into(),
        })
    }

    /// The model identifier this client uses.
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Send a Messages API request and parse the response.
    pub async fn send_message(
        &self,
        request: &MessagesRequest,
    ) -> Result<MessagesResponse, ApiError> {
        let url = format!("{}/v1/messages", self.base_url);

        log::debug!(
            "AnthropicClient::send_message url={} model={} messages={} tools={}",
            url,
            request.model,
            request.messages.len(),
            request.tools.len(),
        );

        let resp = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .json(request)
            .send()
            .await
            .map_err(|e| ApiError {
                status: None,
                message: format!("HTTP request failed: {}", e),
            })?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| ApiError {
            status: Some(status.as_u16()),
            message: format!("Failed to read response body: {}", e),
        })?;

        log::debug!(
            "AnthropicClient::send_message status={} body_len={}",
            status,
            body.len(),
        );

        if !status.is_success() {
            return Err(ApiError {
                status: Some(status.as_u16()),
                message: body,
            });
        }

        serde_json::from_str(&body).map_err(|e| ApiError {
            status: Some(status.as_u16()),
            message: format!("Failed to parse response: {} -- body: {}", e, body),
        })
    }

    /// Send a streaming Messages API request, forwarding text deltas to `reasoning_tx`
    /// as they arrive. Returns the fully assembled response when the stream completes.
    pub async fn send_message_streaming(
        &self,
        request: &MessagesRequest,
        reasoning_tx: Option<tokio::sync::mpsc::UnboundedSender<String>>,
    ) -> Result<MessagesResponse, ApiError> {
        let url = format!("{}/v1/messages", self.base_url);

        log::debug!(
            "AnthropicClient::send_message_streaming url={} model={} messages={} tools={}",
            url,
            request.model,
            request.messages.len(),
            request.tools.len(),
        );

        let resp = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .json(request)
            .send()
            .await
            .map_err(|e| ApiError {
                status: None,
                message: format!("HTTP request failed: {}", e),
            })?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(ApiError {
                status: Some(status.as_u16()),
                message: body,
            });
        }

        // Parse SSE stream into content blocks.
        let mut content_blocks: Vec<ContentBlock> = Vec::new();
        let mut response_id = String::new();
        let mut response_model = String::new();
        let mut stop_reason: Option<String> = None;
        let mut usage: Option<Usage> = None;

        // Per-block accumulators.
        let mut current_text = String::new();
        let mut current_tool_id = String::new();
        let mut current_tool_name = String::new();
        let mut current_tool_json = String::new();
        let mut current_block_type: Option<&'static str> = None;

        // SSE line buffer (chunks may split across lines).
        let mut buf = String::new();
        let mut current_event_type = String::new();

        let mut resp = resp;
        while let Some(chunk) = resp.chunk().await.map_err(|e| ApiError {
            status: None,
            message: format!("Stream read error: {}", e),
        })? {
            buf.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(line_end) = buf.find('\n') {
                let line = buf[..line_end].trim_end_matches('\r').to_string();
                buf = buf[line_end + 1..].to_string();

                if let Some(event_type) = line.strip_prefix("event: ") {
                    current_event_type = event_type.to_string();
                } else if let Some(data) = line.strip_prefix("data: ") {
                    let Ok(json) = serde_json::from_str::<serde_json::Value>(data) else {
                        continue;
                    };

                    match current_event_type.as_str() {
                        "message_start" => {
                            if let Some(msg) = json.get("message") {
                                response_id = msg
                                    .get("id")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                response_model = msg
                                    .get("model")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();
                                if let Some(u) = msg.get("usage") {
                                    usage = serde_json::from_value(u.clone()).ok();
                                }
                            }
                        }
                        "content_block_start" => {
                            if let Some(cb) = json.get("content_block") {
                                let block_type =
                                    cb.get("type").and_then(|v| v.as_str()).unwrap_or("");
                                match block_type {
                                    "text" => {
                                        current_block_type = Some("text");
                                        current_text.clear();
                                        let initial =
                                            cb.get("text").and_then(|v| v.as_str()).unwrap_or("");
                                        if !initial.is_empty() {
                                            current_text.push_str(initial);
                                            if let Some(tx) = &reasoning_tx {
                                                let _ = tx.send(initial.to_string());
                                            }
                                        }
                                    }
                                    "tool_use" => {
                                        current_block_type = Some("tool_use");
                                        current_tool_id = cb
                                            .get("id")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        current_tool_name = cb
                                            .get("name")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        current_tool_json.clear();
                                    }
                                    _ => {
                                        current_block_type = None;
                                    }
                                }
                            }
                        }
                        "content_block_delta" => {
                            if let Some(delta) = json.get("delta") {
                                let delta_type =
                                    delta.get("type").and_then(|v| v.as_str()).unwrap_or("");
                                match delta_type {
                                    "text_delta" => {
                                        let text = delta
                                            .get("text")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("");
                                        if !text.is_empty() {
                                            current_text.push_str(text);
                                            if let Some(tx) = &reasoning_tx {
                                                let _ = tx.send(text.to_string());
                                            }
                                        }
                                    }
                                    "input_json_delta" => {
                                        let partial = delta
                                            .get("partial_json")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("");
                                        current_tool_json.push_str(partial);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "content_block_stop" => match current_block_type {
                            Some("text") => {
                                content_blocks.push(ContentBlock::Text {
                                    text: std::mem::take(&mut current_text),
                                });
                                current_block_type = None;
                            }
                            Some("tool_use") => {
                                let input: serde_json::Value =
                                    serde_json::from_str(&current_tool_json)
                                        .unwrap_or(serde_json::Value::Object(Default::default()));
                                content_blocks.push(ContentBlock::ToolUse {
                                    id: std::mem::take(&mut current_tool_id),
                                    name: std::mem::take(&mut current_tool_name),
                                    input,
                                });
                                current_tool_json.clear();
                                current_block_type = None;
                            }
                            _ => {}
                        },
                        "message_delta" => {
                            if let Some(delta) = json.get("delta") {
                                stop_reason = delta
                                    .get("stop_reason")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());
                            }
                            if let Some(u) = json.get("usage") {
                                if let Ok(u) = serde_json::from_value::<Usage>(u.clone()) {
                                    // Merge output_tokens from message_delta.
                                    if let Some(ref mut existing) = usage {
                                        existing.output_tokens = u.output_tokens;
                                    } else {
                                        usage = Some(u);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        log::debug!(
            "AnthropicClient::send_message_streaming done: id={} blocks={} stop={:?}",
            response_id,
            content_blocks.len(),
            stop_reason,
        );

        Ok(MessagesResponse {
            id: response_id,
            content: content_blocks,
            model: response_model,
            stop_reason,
            usage,
        })
    }

    /// Extract the first tool call matching `name` from a response.
    ///
    /// Returns `(tool_input_args, reasoning_text)` where reasoning is the text
    /// content preceding the tool call (if any).
    pub fn extract_tool_call(
        response: &MessagesResponse,
        name: &str,
    ) -> Option<(serde_json::Value, String)> {
        let mut reasoning = String::new();

        for block in &response.content {
            match block {
                ContentBlock::Text { text } => {
                    if !reasoning.is_empty() {
                        reasoning.push('\n');
                    }
                    reasoning.push_str(text);
                }
                ContentBlock::ToolUse {
                    name: tool_name,
                    input,
                    ..
                } => {
                    if tool_name == name {
                        return Some((input.clone(), reasoning));
                    }
                }
                _ => {}
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn request_serialization_omits_none_fields() {
        let req = MessagesRequest::new("test-model", 1024);
        let json = serde_json::to_value(&req).unwrap();
        assert!(!json.as_object().unwrap().contains_key("id_slot"));
        assert!(!json.as_object().unwrap().contains_key("cache_prompt"));
        assert!(!json.as_object().unwrap().contains_key("system"));
        assert!(!json.as_object().unwrap().contains_key("tools"));
        assert_eq!(json["stream"], false);
    }

    #[test]
    fn request_serialization_includes_llamafile_fields() {
        let mut req = MessagesRequest::new("test-model", 1024);
        req.id_slot = Some(2);
        req.cache_prompt = Some(true);
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["id_slot"], 2);
        assert_eq!(json["cache_prompt"], true);
    }

    #[test]
    fn message_user_creates_text_block() {
        let msg = Message::user("hello");
        assert_eq!(msg.role, Role::User);
        assert_eq!(msg.content.len(), 1);
        match &msg.content[0] {
            ContentBlock::Text { text } => assert_eq!(text, "hello"),
            _ => panic!("expected Text block"),
        }
    }

    #[test]
    fn message_assistant_tool_use_creates_correct_block() {
        let msg = Message::assistant_tool_use("id-1", "choose_index", json!({"index": 3}));
        assert_eq!(msg.role, Role::Assistant);
        match &msg.content[0] {
            ContentBlock::ToolUse { id, name, input } => {
                assert_eq!(id, "id-1");
                assert_eq!(name, "choose_index");
                assert_eq!(input["index"], 3);
            }
            _ => panic!("expected ToolUse block"),
        }
    }

    #[test]
    fn extract_tool_call_finds_matching_tool() {
        let response = MessagesResponse {
            id: "msg-1".into(),
            content: vec![
                ContentBlock::Text {
                    text: "I think option 2 is best because it has good resources.".into(),
                },
                ContentBlock::ToolUse {
                    id: "tu-1".into(),
                    name: "choose_index".into(),
                    input: json!({"index": 2}),
                },
            ],
            model: "test".into(),
            stop_reason: Some("tool_use".into()),
            usage: None,
        };

        let (args, reasoning) = AnthropicClient::extract_tool_call(&response, "choose_index")
            .expect("should find tool call");
        assert_eq!(args["index"], 2);
        assert!(reasoning.contains("I think option 2 is best"));
    }

    #[test]
    fn extract_tool_call_returns_none_for_missing_tool() {
        let response = MessagesResponse {
            id: "msg-1".into(),
            content: vec![ContentBlock::Text {
                text: "No tool here".into(),
            }],
            model: "test".into(),
            stop_reason: Some("end_turn".into()),
            usage: None,
        };

        assert!(AnthropicClient::extract_tool_call(&response, "choose_index").is_none());
    }

    #[test]
    fn response_deserialization() {
        let json = json!({
            "id": "msg-123",
            "content": [
                {"type": "text", "text": "reasoning here"},
                {"type": "tool_use", "id": "tu-1", "name": "choose_index", "input": {"index": 0}}
            ],
            "model": "bonsai",
            "stop_reason": "tool_use",
            "usage": {"input_tokens": 100, "output_tokens": 50}
        });

        let resp: MessagesResponse = serde_json::from_value(json).unwrap();
        assert_eq!(resp.id, "msg-123");
        assert_eq!(resp.content.len(), 2);
        assert_eq!(resp.usage.unwrap().input_tokens, 100);
    }

    #[test]
    fn content_block_text_round_trip() {
        let block = ContentBlock::Text {
            text: "hello".into(),
        };
        let json = serde_json::to_value(&block).unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["text"], "hello");

        let parsed: ContentBlock = serde_json::from_value(json).unwrap();
        match parsed {
            ContentBlock::Text { text } => assert_eq!(text, "hello"),
            _ => panic!("expected Text"),
        }
    }

    #[test]
    fn content_block_tool_use_round_trip() {
        let block = ContentBlock::ToolUse {
            id: "tu-1".into(),
            name: "test_tool".into(),
            input: json!({"key": "value"}),
        };
        let json = serde_json::to_value(&block).unwrap();
        assert_eq!(json["type"], "tool_use");
        assert_eq!(json["name"], "test_tool");

        let parsed: ContentBlock = serde_json::from_value(json).unwrap();
        match parsed {
            ContentBlock::ToolUse { id, name, input } => {
                assert_eq!(id, "tu-1");
                assert_eq!(name, "test_tool");
                assert_eq!(input["key"], "value");
            }
            _ => panic!("expected ToolUse"),
        }
    }

    #[test]
    fn tool_def_serialization() {
        let tool = ToolDef {
            name: "choose_index".into(),
            description: "Pick an option".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "index": {"type": "integer"}
                },
                "required": ["index"]
            }),
        };
        let json = serde_json::to_value(&tool).unwrap();
        assert_eq!(json["name"], "choose_index");
        assert!(json["input_schema"]["properties"]["index"].is_object());
    }

    #[test]
    fn client_constructor() {
        let client = AnthropicClient::new("http://localhost:8080", "test-key", "test-model");
        assert_eq!(client.model(), "test-model");
    }
}
