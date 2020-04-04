use serde_json::{json, Map, Value};
use jlog::parser::{LogKeys, LogEntry};

#[test]
fn test_parse_plan_text() {
    let log_keys = LogKeys::default();

    let result = LogEntry::parse(&log_keys, "text");

    assert!(result.is_err());
}

#[test]
fn test_parse_json_empty() {
    let log_keys = LogKeys::default();

    let text = "{}".to_string();

    let result = LogEntry::parse(&log_keys, &text);

    assert_eq!(result.unwrap(), LogEntry::default());
}

#[test]
fn test_parse_json_full() {
    let log_keys = LogKeys::default();

    let text = json!({
        "timestamp":"2020-04-04T00:00:000.000",
        "thread":"main",
        "level":"INFO",
        "logger":"logger",
        "message":"message",
        "stack_trace":"trace"
    }).to_string();

    let expected = LogEntry {
        timestamp: "2020-04-04T00:00:000.000".to_string(),
        thread: "main".to_string(),
        level: "INFO".to_string(),
        logger: "logger".to_string(),
        message: "message".to_string(),
        stack_trace: "trace".to_string(),
    };

    let result = LogEntry::parse(&log_keys, &text);

    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_parse_json_types() {
    let log_keys = LogKeys::default();

    let text = json!({
        "timestamp":"2020-04-04T00:00:000.000",
        "thread":1,
        "level":true,
        "logger":{},
        "message":[]
    }).to_string();

    let expected = LogEntry {
        timestamp: "2020-04-04T00:00:000.000".to_string(),
        thread: 1.to_string(),
        level: true.to_string(),
        logger: "{}".to_string(),
        message: "[]".to_string(),
        stack_trace: String::default(),
    };

    let result = LogEntry::parse(&log_keys, &text);

    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_timestamp_log_keys() {
    let log_keys = LogKeys::default();
    let value = "value";

    let mut expected = LogEntry::default();
    expected.timestamp = value.to_string();

    assert_log_keys(&log_keys, &log_keys.timestamp, &expected, &value);
}

#[test]
fn test_thread_log_keys() {
    let log_keys = LogKeys::default();
    let value = "value";

    let mut expected = LogEntry::default();
    expected.thread = value.to_string();

    assert_log_keys(&log_keys, &log_keys.thread, &expected, &value);
}

#[test]
fn test_level_log_keys() {
    let log_keys = LogKeys::default();
    let value = "value";

    let mut expected = LogEntry::default();
    expected.level = value.to_string();

    assert_log_keys(&log_keys, &log_keys.level, &expected, &value);
}

#[test]
fn test_logger_log_keys() {
    let log_keys = LogKeys::default();
    let value = "value";

    let mut expected = LogEntry::default();
    expected.logger = value.to_string();

    assert_log_keys(&log_keys, &log_keys.logger, &expected, &value);
}

#[test]
fn test_message_log_keys() {
    let log_keys = LogKeys::default();
    let value = "value";

    let mut expected = LogEntry::default();
    expected.message = value.to_string();

    assert_log_keys(&log_keys, &log_keys.message, &expected, &value);
}

#[test]
fn test_stack_trace_log_keys() {
    let log_keys = LogKeys::default();
    let value = "value";

    let mut log_entry = LogEntry::default();
    log_entry.stack_trace = value.to_string();

    assert_log_keys(&log_keys, &log_keys.stack_trace, &log_entry, &value);
}

fn assert_log_keys(log_keys: &LogKeys, keys: &Vec<String>, log_entry: &LogEntry, value: &str) {
    for key in keys {
        let mut map = Map::new();
        map.insert(key.to_string(), Value::String(value.to_string()));
        let text = Value::Object(map).to_string();

        let result = LogEntry::parse(&log_keys, &text);
        assert_eq!(result.unwrap(), *log_entry);
    }

    assert!(!keys.is_empty());
}