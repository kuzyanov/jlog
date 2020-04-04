use clap::{App, ArgMatches};

pub fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("jlog")
        .version(env!("CARGO_PKG_VERSION"))
        .about("JSON log parser")
        .after_help("Example usage:\n\
        $ cat json_logs.log | jlog > parsed_logs.log\n\n\
        \
        Parses JSON object logs line by line, if line cannot be parsed it will be printed as is.\n\
        By default parse following JSON keys:\n\
        \t* time, timestamp, @timestamp\n\
        \t* thread, thread_name\n\
        \t* level\n\
        \t* logger, logger_name\n\
        \t* msg, message\n\
        \t* stack_trace\n\n\
        \
        Value conversions:\n\
        \t* Thread name wraps by []: main > [main]\n\
        \t* Logger name with dots are truncated: com.example.Logger > c.e.Logger\n\
        \t* Stack trace printed on new line\n\n\
        \
        Example for command: \n\
        $ cat <<EOF | jlog\n\
        {\"timestamp\": \"01:02:03\", \"level\": \"INFO\", \"message\": \"message\"}\n\
        plain text\n\
        EOF\n\n\
        \
        output:\n\
        01:02:03 INFO - message\n\
        plain text")
        .get_matches()
}