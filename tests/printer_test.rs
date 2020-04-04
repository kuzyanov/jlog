use jlog::parser::LogEntry;

#[test]
fn test_print_empty_entry() {
    let entry = LogEntry::default();

    assert_eq!(entry.to_string(), String::default());
}

#[test]
fn test_print_empty_full_entry() {
    let entry = LogEntry {
        timestamp: "2020-04-04T00:00:000.000".to_string(),
        thread: "main".to_string(),
        level: "INFO".to_string(),
        logger: "logger".to_string(),
        message: "message".to_string(),
        stack_trace: "trace".to_string(),
    };

    let expected = "2020-04-04T00:00:000.000 [main] INFO logger - message\ntrace";

    assert_eq!(entry.to_string(), expected);
}

#[test]
fn test_print_empty_dot_separated_logger_name() {
    let entry = LogEntry {
        timestamp: "2020-04-04T00:00:000.000".to_string(),
        thread: "main".to_string(),
        level: "INFO".to_string(),
        logger: "com.example.Logger".to_string(),
        message: "message".to_string(),
        stack_trace: "trace".to_string(),
    };

    let expected = "2020-04-04T00:00:000.000 [main] INFO c.e.Logger - message\ntrace";

    assert_eq!(entry.to_string(), expected);
}