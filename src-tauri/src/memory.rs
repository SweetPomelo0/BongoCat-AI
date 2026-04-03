use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};
use tauri::{AppHandle, Manager};

const MEMORY_DIR_NAME: &str = "memory";
const DAILY_DIR_NAME: &str = "daily";
const PERSONA_FILE_NAME: &str = "PERSONA.md";
const MEMORY_FILE_NAME: &str = "MEMORY.md";

const DEFAULT_PERSONA: &str = r#"# BongoCat Persona

你是一只宝宝。

## 性格
- 简洁、自然、友好
- 优先中文回答
- 不说教，不过度煽情

## 行为原则
- 优先帮助用户完成当前任务
- 记住真正重要、跨天仍有价值的信息
- 对临时事项写入每日笔记，不滥写长期记忆
"#;

const DEFAULT_MEMORY: &str = r#"# 用户偏好

# 持续项目

# 长期背景

# 沟通偏好
"#;

const DAILY_TEMPLATE: &str = r#"## Context

## Decisions

## Temporary Notes
"#;

const RECENT_DAILY_NOTES_LIMIT: usize = 3;
const DAILY_NOTE_CHAR_LIMIT: usize = 2000;
const MEMORY_ENTRY_LIMIT: usize = 6;
const MEMORY_ENTRY_CHAR_LIMIT: usize = 240;
const MEMORY_TEXT_CHAR_LIMIT: usize = 4000;

#[derive(Debug, Clone)]
pub struct MemoryPaths {
    pub root_dir: PathBuf,
    pub persona_path: PathBuf,
    pub memory_path: PathBuf,
    pub today_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct MemoryContext {
    pub persona: String,
    pub long_term: String,
    pub recent_daily_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryExtraction {
    pub daily_note_entries: Vec<String>,
    pub durable_memory_entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MemoryDebugState {
    pub enabled: bool,
    pub persona_preview: String,
    pub long_term_preview: String,
    pub recent_daily_notes_preview: String,
}

#[derive(Debug, Serialize)]
pub struct MemoryStatus {
    pub root_dir: String,
    pub persona_path: String,
    pub memory_path: String,
    pub today_path: String,
}

fn ensure_file(path: &PathBuf, content: &str) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }

    fs::write(path, content).map_err(|err| err.to_string())
}

pub fn ensure_memory_workspace(app_handle: &AppHandle) -> Result<MemoryPaths, String> {
    let app_data = app_handle
        .path()
        .app_data_dir()
        .map_err(|err| err.to_string())?;

    let root_dir = app_data.join(MEMORY_DIR_NAME);
    let daily_dir = root_dir.join(DAILY_DIR_NAME);
    fs::create_dir_all(&daily_dir).map_err(|err| err.to_string())?;

    let persona_path = root_dir.join(PERSONA_FILE_NAME);
    let memory_path = root_dir.join(MEMORY_FILE_NAME);
    let today_file_name = format!("{}.md", Local::now().format("%Y-%m-%d"));
    let today_path = daily_dir.join(today_file_name);

    ensure_file(&persona_path, DEFAULT_PERSONA)?;
    ensure_file(&memory_path, DEFAULT_MEMORY)?;
    ensure_file(&today_path, DAILY_TEMPLATE)?;

    Ok(MemoryPaths {
        root_dir,
        persona_path,
        memory_path,
        today_path,
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenMemoryPathRequest {
    target: String,
}

#[tauri::command]
pub fn get_memory_status(app_handle: AppHandle) -> Result<MemoryStatus, String> {
    let paths = ensure_memory_workspace(&app_handle)?;

    Ok(MemoryStatus {
        root_dir: paths.root_dir.display().to_string(),
        persona_path: paths.persona_path.display().to_string(),
        memory_path: paths.memory_path.display().to_string(),
        today_path: paths.today_path.display().to_string(),
    })
}

#[tauri::command]
pub fn open_memory_path(app_handle: AppHandle, payload: OpenMemoryPathRequest) -> Result<(), String> {
    let paths = ensure_memory_workspace(&app_handle)?;
    let (target_path, is_dir) = match payload.target.as_str() {
        "rootDir" => (paths.root_dir, true),
        "personaPath" => (paths.persona_path, false),
        "memoryPath" => (paths.memory_path, false),
        "todayPath" => (paths.today_path, false),
        _ => return Err("invalid memory path target".to_string()),
    };

    if is_dir {
        Command::new("explorer")
            .arg(target_path)
            .spawn()
            .map(|_| ())
            .map_err(|err| err.to_string())
    } else {
        Command::new("cmd")
            .args(["/C", "start", "", &target_path.display().to_string()])
            .spawn()
            .map(|_| ())
            .map_err(|err| err.to_string())
    }
}

fn read_file_or_empty(path: &PathBuf) -> String {
    fs::read_to_string(path)
        .unwrap_or_default()
        .replace('\u{FFFD}', "")
}

fn truncate_chars(content: String, limit: usize) -> String {
    content.chars().take(limit).collect()
}

fn parse_daily_note_date(path: &PathBuf) -> Option<NaiveDate> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
}

fn load_recent_daily_notes(paths: &MemoryPaths, limit: usize) -> String {
    let daily_dir = paths.root_dir.join(DAILY_DIR_NAME);
    let Ok(entries) = fs::read_dir(daily_dir) else {
        return String::new();
    };

    let mut dated_notes = entries
        .filter_map(|entry| entry.ok().map(|item| item.path()))
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("md"))
        .filter_map(|path| parse_daily_note_date(&path).map(|date| (date, path)))
        .collect::<Vec<_>>();

    dated_notes.sort_by(|a, b| b.0.cmp(&a.0));

    dated_notes
        .into_iter()
        .take(limit)
        .filter_map(|(date, path)| {
            let content = truncate_chars(read_file_or_empty(&path), DAILY_NOTE_CHAR_LIMIT);
            let trimmed = content.trim();
            if trimmed.is_empty() || trimmed.contains('�') {
                None
            } else {
                Some(format!("### {}\n{}", date.format("%Y-%m-%d"), trimmed))
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn build_effective_persona(persona: String) -> String {
    let trimmed = truncate_chars(persona.trim().to_string(), MEMORY_TEXT_CHAR_LIMIT);
    if trimmed.is_empty() {
        DEFAULT_PERSONA.to_string()
    } else {
        trimmed
    }
}

fn build_effective_long_term_memory(long_term: String) -> String {
    let trimmed = truncate_chars(long_term.trim().to_string(), MEMORY_TEXT_CHAR_LIMIT);
    if trimmed.is_empty() {
        DEFAULT_MEMORY.to_string()
    } else {
        trimmed
    }
}

pub fn load_memory_context(app_handle: &AppHandle, memory_enabled: bool) -> Result<Option<MemoryContext>, String> {
    if !memory_enabled {
        return Ok(None);
    }

    let paths = ensure_memory_workspace(app_handle)?;

    let persona = build_effective_persona(read_file_or_empty(&paths.persona_path));
    let long_term = build_effective_long_term_memory(read_file_or_empty(&paths.memory_path));
    let recent_daily_notes = load_recent_daily_notes(&paths, RECENT_DAILY_NOTES_LIMIT);

    Ok(Some(MemoryContext {
        persona,
        long_term,
        recent_daily_notes,
    }))
}

pub fn build_memory_system_prompt(context: Option<&MemoryContext>) -> String {
    let Some(context) = context else {
        return "你是 BongoCat AI，一只住在桌面上的 AI 桌宠。说话简洁、自然、友好，优先中文，回答直接，不要冗长。".to_string();
    };

    format!(
        "你是 BongoCat AI，一只住在桌面上的 AI 桌宠。说话简洁、自然、友好，优先中文，回答直接，不要冗长。\n\n这些记忆来自用户本地可编辑的 Markdown 文件。当前对话中的明确新指令优先于旧记忆。长期记忆只代表稳定事实与偏好；最近日记主要代表最近几天的临时上下文。\n\n[Persona]\n{}\n\n[Long-term Memory]\n{}\n\n[Recent Daily Notes]\n{}",
        context.persona,
        context.long_term,
        if context.recent_daily_notes.trim().is_empty() {
            "暂无最近日记。".to_string()
        } else {
            context.recent_daily_notes.clone()
        }
    )
}

pub fn build_memory_debug_state(context: Option<&MemoryContext>) -> MemoryDebugState {
    let Some(context) = context else {
        return MemoryDebugState {
            enabled: false,
            persona_preview: String::new(),
            long_term_preview: String::new(),
            recent_daily_notes_preview: String::new(),
        };
    };

    MemoryDebugState {
        enabled: true,
        persona_preview: truncate_chars(context.persona.clone(), 200),
        long_term_preview: truncate_chars(context.long_term.clone(), 200),
        recent_daily_notes_preview: truncate_chars(context.recent_daily_notes.clone(), 300),
    }
}

fn normalize_entry(entry: &str) -> String {
    entry.trim().replace("\r\n", "\n")
}

fn sanitize_entries(entries: &[String], limit: usize) -> Vec<String> {
    entries
        .iter()
        .map(|entry| truncate_chars(normalize_entry(entry), MEMORY_ENTRY_CHAR_LIMIT))
        .filter(|entry| !entry.is_empty() && !looks_sensitive(entry) && !looks_noisy(entry))
        .take(limit)
        .collect()
}

fn append_unique_bullets(path: &PathBuf, entries: &[String]) -> Result<(), String> {
    if entries.is_empty() {
        return Ok(());
    }

    let mut content = fs::read_to_string(path).unwrap_or_default();

    for entry in sanitize_entries(entries, MEMORY_ENTRY_LIMIT) {
        if content.contains(&entry) {
            continue;
        }

        if !content.ends_with('\n') {
            content.push('\n');
        }

        content.push_str("- ");
        content.push_str(&entry);
        content.push('\n');
    }

    fs::write(path, content).map_err(|err| err.to_string())
}

fn looks_sensitive(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api key")
        || lower.contains("apikey")
        || lower.contains("token")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("authorization")
        || lower.contains("cookie")
        || lower.contains("sk-")
}

fn looks_noisy(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.contains("http://")
        || trimmed.contains("https://")
        || trimmed.contains("```")
        || trimmed.lines().count() > 3
        || trimmed.matches('/').count() > 4
}

pub fn apply_memory_updates(app_handle: &AppHandle, updates: &MemoryExtraction) -> Result<(), String> {
    let paths = ensure_memory_workspace(app_handle)?;

    let daily_entries = sanitize_entries(&updates.daily_note_entries, MEMORY_ENTRY_LIMIT);
    let durable_entries = sanitize_entries(&updates.durable_memory_entries, MEMORY_ENTRY_LIMIT / 2);

    append_unique_bullets(&paths.today_path, &daily_entries)?;
    append_unique_bullets(&paths.memory_path, &durable_entries)?;

    Ok(())
}
