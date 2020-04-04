use crate::parser::LogEntry;

impl ToString for LogEntry {
    fn to_string(&self) -> String {
        print_to_string(self)
    }
}

fn print_to_string(log_entry: &LogEntry) -> String {
    let mut log_record: String = String::new();

    append(&mut log_record, &log_entry.timestamp);
    append_with_space(&mut log_record, &format_thread_name(log_entry));
    append_with_space(&mut log_record, &log_entry.level);
    append_with_space(&mut log_record, &format_logger_name(log_entry));
    append_with_space(&mut log_record, &format_message(log_entry));
    append_with_new_line(&mut log_record, &log_entry.stack_trace);

    log_record
}

fn format_thread_name(log_entry: &LogEntry) -> String {
    Option::Some(&log_entry.thread)
        .filter(|v| !v.is_empty())
        .map(|v| format!("[{}]", v))
        .unwrap_or(String::default())
}

fn format_logger_name(log_entry: &LogEntry) -> String {
    let split_result: Vec<&str> = (&log_entry.logger).split(".").collect();
    let len = split_result.len();

    if len == 1 {
        split_result.get(0).unwrap().to_string()
    } else {
        let mut logger_name = String::new();
        for (i, item) in split_result.iter().enumerate() {
            if i < len - 1 {
                let first_char = item.chars().next().unwrap().to_string();
                logger_name.push_str(&first_char);
                logger_name.push_str(".");
            } else {
                logger_name.push_str(item);
            }
        }
        logger_name
    }
}

fn format_message(log_entry: &LogEntry) -> String {
    if log_entry.message.is_empty() {
        String::default()
    } else {
        format!("- {}", log_entry.message)
    }
}

fn append(row: &mut String, value: &str) {
    if value.is_empty() {
        return;
    }
    row.push_str(value);
}

fn append_with_space(row: &mut String, value: &str) {
    if value.is_empty() {
        return;
    }
    row.push_str(" ");
    row.push_str(value);
}

fn append_with_new_line(row: &mut String, value: &str) {
    if value.is_empty() {
        return;
    }
    row.push_str("\n");
    row.push_str(value);
}