use serde_json::{Map, Value};

pub struct LogKeys {
    pub timestamp: Vec<String>,
    pub thread: Vec<String>,
    pub level: Vec<String>,
    pub logger: Vec<String>,
    pub message: Vec<String>,
    pub stack_trace: Vec<String>,
}

impl Default for LogKeys {
    fn default() -> Self {
        LogKeys {
            timestamp: vec!["time".to_string(), "timestamp".to_string(), "@timestamp".to_string()],
            thread: vec!["thread".to_string(), "thread_name".to_string()],
            level: vec!["level".to_string()],
            logger: vec!["logger".to_string(), "logger_name".to_string()],
            message: vec!["msg".to_string(), "message".to_string()],
            stack_trace: vec!["stack_trace".to_string()],
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct LogEntry {
    pub timestamp: String,
    pub thread: String,
    pub level: String,
    pub logger: String,
    pub message: String,
    pub stack_trace: String,
}


impl LogEntry {
    pub fn parse(log_keys: &LogKeys, raw_string: &str) -> Result<LogEntry, String> {
        match serde_json::from_str::<Value>(&raw_string) {
            Ok(json_line) => parse_json_line(log_keys, json_line, raw_string),
            Err(_) => Err(raw_string.to_string())
        }
    }
}

fn parse_json_line(log_keys: &LogKeys, value: Value, raw_string: &str) -> Result<LogEntry, String> {
    match value.as_object() {
        Some(json_object) => Ok(parse_json_object(log_keys, json_object)),
        None => Err(raw_string.to_string())
    }
}

fn parse_json_object(log_keys: &LogKeys, json_object: &Map<String, Value>) -> LogEntry {
    LogEntry {
        timestamp: get_str(json_object, &log_keys.timestamp),
        thread: get_str(json_object, &log_keys.thread),
        level: get_str(json_object, &log_keys.level),
        logger: get_str(json_object, &log_keys.logger),
        message: get_str(json_object, &log_keys.message),
        stack_trace: get_str(json_object, &log_keys.stack_trace),
    }
}

fn get_str(json_object: &Map<String, Value>, keys: &Vec<String>) -> String {
    keys.iter()
        .find(|key| json_object.contains_key(*key))
        .map(|key| get_str_value(json_object, key))
        .unwrap_or(String::default())
}

fn get_str_value(json_object: &Map<String, Value>, key: &str) -> String {
    json_object.get(key)
        .map(|value| value_to_string(value))
        .unwrap_or(String::default())
}

fn value_to_string(value: &Value) -> String {
    value.as_str()
        .map(|s| String::from(s))
        .unwrap_or_else(|| value.to_string())
}
