use crate::memory::{
    MemoryContext, MemoryExtraction, apply_memory_updates, build_memory_debug_state,
    build_memory_system_prompt, load_memory_context,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

const DEFAULT_SILICONFLOW_API_URL: &str = "https://api.siliconflow.cn/v1/chat/completions";
const DEFAULT_SILICONFLOW_MODEL: &str = "Qwen/Qwen2.5-7B-Instruct";
const CHAT_STREAM_START: &str = "chat-stream-start";
const CHAT_STREAM_CHUNK: &str = "chat-stream-chunk";
const CHAT_STREAM_END: &str = "chat-stream-end";
const CHAT_STREAM_ERROR: &str = "chat-stream-error";
const CHAT_STREAM_CHUNK_SIZE: usize = 12;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatConfig {
    provider: String,
    model: String,
    api_key: String,
    base_url: String,
    memory_enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChatHistoryMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatRequestMessage>,
    temperature: f32,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct ChatRequestMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponseMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct MemoryExtractionResponse {
    daily_note_entries: Vec<String>,
    durable_memory_entries: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
struct ChatStreamPayload {
    content: String,
}

struct ResolvedChatConfig {
    api_key: String,
    model: String,
    base_url: String,
    memory_enabled: bool,
}

struct PreparedChatContext {
    memory_context: Option<MemoryContext>,
    system_prompt: String,
    memory_enabled: bool,
}

fn resolve_config(config: ChatConfig) -> Result<ResolvedChatConfig, String> {
    let api_key = if config.api_key.trim().is_empty() {
        return Err("API Key is empty. Please configure it in Preferences > AI.".to_string());
    } else {
        config.api_key.trim().to_string()
    };
    let model = if config.model.trim().is_empty() {
        DEFAULT_SILICONFLOW_MODEL.to_string()
    } else {
        config.model.trim().to_string()
    };
    let base_url = if config.base_url.trim().is_empty() {
        DEFAULT_SILICONFLOW_API_URL.to_string()
    } else {
        config.base_url.trim().to_string()
    };
    let _provider = config.provider;

    Ok(ResolvedChatConfig {
        api_key,
        model,
        base_url,
        memory_enabled: config.memory_enabled,
    })
}

fn prepare_chat_context(
    app_handle: &AppHandle,
    memory_enabled: bool,
) -> Result<PreparedChatContext, String> {
    let memory_context = load_memory_context(app_handle, memory_enabled)?;
    let memory_debug = build_memory_debug_state(memory_context.as_ref());

    println!(
        "[memory-debug] enabled={} persona_preview={:?} long_term_preview={:?} recent_daily_notes_preview={:?}",
        memory_debug.enabled,
        memory_debug.persona_preview,
        memory_debug.long_term_preview,
        memory_debug.recent_daily_notes_preview
    );

    Ok(PreparedChatContext {
        system_prompt: build_memory_system_prompt(memory_context.as_ref()),
        memory_enabled: memory_context.is_some(),
        memory_context,
    })
}

fn build_messages(
    system_prompt: String,
    message: String,
    history: Vec<ChatHistoryMessage>,
) -> Vec<ChatRequestMessage> {
    let mut messages = vec![ChatRequestMessage {
        role: "system".to_string(),
        content: system_prompt,
    }];

    let recent_history = history.into_iter().rev().take(8).collect::<Vec<_>>();

    for item in recent_history.into_iter().rev() {
        if item.content.trim().is_empty() {
            continue;
        }

        messages.push(ChatRequestMessage {
            role: item.role,
            content: item.content,
        });
    }

    messages.push(ChatRequestMessage {
        role: "user".to_string(),
        content: message,
    });

    messages
}

fn build_memory_extraction_messages(
    message: &str,
    reply: &str,
    history: &[ChatHistoryMessage],
) -> Vec<ChatRequestMessage> {
    let history_block = history
        .iter()
        .rev()
        .take(6)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|item| format!("{}: {}", item.role, item.content))
        .collect::<Vec<_>>()
        .join("\n");

    vec![
        ChatRequestMessage {
            role: "system".to_string(),
            content: "你负责把本轮对话提取成结构化记忆。只输出 JSON，不要输出 markdown、解释或额外文字。JSON 结构必须是 {\"daily_note_entries\": string[], \"durable_memory_entries\": string[]}。durable_memory_entries 只能记录稳定偏好、长期项目、跨天约束、长期背景；daily_note_entries 只记录当天上下文、短期任务、临时决策、当前关注点。若不确定，返回空数组。绝对不要记录 API key、token、password、secret、sk- 开头内容，也不要记录低价值寒暄。每个数组最多 3 条，每条一句话，简短具体。".to_string(),
        },
        ChatRequestMessage {
            role: "user".to_string(),
            content: format!(
                "[Recent History]\n{}\n\n[Current User Message]\n{}\n\n[Assistant Reply]\n{}",
                if history_block.is_empty() {
                    "(empty)".to_string()
                } else {
                    history_block
                },
                message,
                reply
            ),
        },
    ]
}

async fn request_chat_completion(
    client: &Client,
    api_key: &str,
    model: &str,
    base_url: &str,
    messages: Vec<ChatRequestMessage>,
    temperature: f32,
) -> Result<String, String> {
    let payload = ChatRequest {
        model: model.to_string(),
        messages,
        temperature,
        stream: false,
    };

    let response = client
        .post(base_url)
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|err| format!("request failed: {err}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "failed to read error body".to_string());

        return Err(format!("SiliconFlow API error ({status}): {body}"));
    }

    let data: ChatResponse = response
        .json()
        .await
        .map_err(|err| format!("invalid response: {err}"))?;

    data.choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .ok_or_else(|| "empty response from SiliconFlow".to_string())
}

async fn extract_structured_memory_updates(
    client: &Client,
    api_key: &str,
    model: &str,
    base_url: &str,
    message: &str,
    reply: &str,
    history: &[ChatHistoryMessage],
) -> Result<MemoryExtraction, String> {
    let payload = ChatRequest {
        model: model.to_string(),
        messages: build_memory_extraction_messages(message, reply, history),
        temperature: 0.1,
        stream: false,
    };

    let response = client
        .post(base_url)
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|err| format!("memory extraction request failed: {err}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "failed to read extraction error body".to_string());

        return Err(format!("memory extraction API error ({status}): {body}"));
    }

    let data: ChatResponse = response
        .json()
        .await
        .map_err(|err| format!("invalid extraction response: {err}"))?;

    let content = data
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .ok_or_else(|| "empty extraction response".to_string())?;

    let parsed: MemoryExtractionResponse = serde_json::from_str(content.trim())
        .map_err(|err| format!("invalid extraction JSON: {err}"))?;

    Ok(MemoryExtraction {
        daily_note_entries: parsed.daily_note_entries,
        durable_memory_entries: parsed.durable_memory_entries,
    })
}

async fn persist_memory_updates_if_enabled(
    app_handle: &AppHandle,
    client: &Client,
    resolved_config: &ResolvedChatConfig,
    prepared_context: &PreparedChatContext,
    message: &str,
    reply: &str,
    history: &[ChatHistoryMessage],
) -> Result<(), String> {
    if !prepared_context.memory_enabled || prepared_context.memory_context.is_none() {
        return Ok(());
    }

    let updates = match extract_structured_memory_updates(
        client,
        &resolved_config.api_key,
        &resolved_config.model,
        &resolved_config.base_url,
        message,
        reply,
        history,
    )
    .await
    {
        Ok(updates) => updates,
        Err(error) => {
            println!("[memory-extraction-error] {error}");
            return Ok(());
        }
    };

    apply_memory_updates(app_handle, &updates)
}

fn emit_stream_events(app_handle: &AppHandle, reply: &str) -> Result<(), String> {
    app_handle
        .emit(
            CHAT_STREAM_START,
            ChatStreamPayload {
                content: String::new(),
            },
        )
        .map_err(|err| format!("emit start failed: {err}"))?;

    let chars = reply.chars().collect::<Vec<_>>();
    let mut buffer = String::new();

    for chunk in chars.chunks(CHAT_STREAM_CHUNK_SIZE) {
        let piece = chunk.iter().collect::<String>();
        buffer.push_str(&piece);

        app_handle
            .emit(
                CHAT_STREAM_CHUNK,
                ChatStreamPayload {
                    content: piece,
                },
            )
            .map_err(|err| format!("emit chunk failed: {err}"))?;
    }

    app_handle
        .emit(
            CHAT_STREAM_END,
            ChatStreamPayload { content: buffer },
        )
        .map_err(|err| format!("emit end failed: {err}"))?;

    Ok(())
}

fn emit_stream_error(app_handle: &AppHandle, message: &str) {
    let _ = app_handle.emit(
        CHAT_STREAM_ERROR,
        ChatStreamPayload {
            content: message.to_string(),
        },
    );
}

#[tauri::command]
pub async fn send_chat_message(
    app_handle: AppHandle,
    message: String,
    history: Vec<ChatHistoryMessage>,
    config: ChatConfig,
) -> Result<String, String> {
    let client = Client::new();
    let resolved_config = resolve_config(config)?;
    let prepared_context = prepare_chat_context(&app_handle, resolved_config.memory_enabled)?;
    let reply = request_chat_completion(
        &client,
        &resolved_config.api_key,
        &resolved_config.model,
        &resolved_config.base_url,
        build_messages(
            prepared_context.system_prompt.clone(),
            message.clone(),
            history.clone(),
        ),
        0.7,
    )
    .await?;

    persist_memory_updates_if_enabled(
        &app_handle,
        &client,
        &resolved_config,
        &prepared_context,
        &message,
        &reply,
        &history,
    )
    .await?;

    Ok(reply)
}

#[tauri::command]
pub async fn send_chat_message_stream(
    app_handle: AppHandle,
    message: String,
    history: Vec<ChatHistoryMessage>,
    config: ChatConfig,
) -> Result<(), String> {
    let reply = match send_chat_message(app_handle.clone(), message, history, config).await {
        Ok(reply) => reply,
        Err(error) => {
            emit_stream_error(&app_handle, &error);
            return Err(error);
        }
    };

    if let Err(error) = emit_stream_events(&app_handle, &reply) {
        emit_stream_error(&app_handle, &error);
        return Err(error);
    }

    Ok(())
}
