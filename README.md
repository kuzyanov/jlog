# jlog
JSON log parser

# Flags:
```shell script
-h, --help       Prints help information
-V, --version    Prints version information
```

# Binaries
Binaries built for x86_x64 linux and windows - [release page](https://github.com/kuzyanov/jlog/releases).

# Setup
Add jlog executable to PATH variable.

# Example usage:
```shell script
cat json_logs.log | jlog > parsed_logs.log
```

# Description
Parses JSON object logs line by line, if line cannot be parsed it will be printed as is.
#### By default parse following JSON keys:
* time, timestamp, @timestamp
* thread, thread_name
* level
* logger, logger_name
* msg, message
* stack_trace

#### Value conversions:
* Thread name wraps by []: main > [main]
* Logger name with dots are truncated: com.example.Logger > c.e.Logger
* Stack trace printed on new line

# Output example:
For command:
```shell script
$ cat <<EOF | jlog
{"timestamp": "01:02:03", "level": "INFO", "message": "message"}
plain text
EOF
```
output:
```shell script
01:02:03 INFO - message
plain text
```