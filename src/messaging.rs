#[derive(Debug, PartialEq)]
pub enum ReceivedMessage {
    Exec { command: String, application: Option<String> },
    Log { action: Option<String>, application: Option<String>},
    Dbms { action: String, application: Option<String> },
    None,
}

impl ReceivedMessage {
    fn new(parts: Vec<&str>) -> ReceivedMessage {
        match parts[0] {
            "exec" => ReceivedMessage::Exec {
                command: parts[1].to_string(),
                application: Some(parts[2].to_string())
            },
            "log" => ReceivedMessage::Log {
                action: Some(parts[1].to_string()),
                application: Some(parts[2].to_string())
            },
            "dbms" => ReceivedMessage::Dbms {
                action: parts[1].to_string(),
                application: Some(parts[2].to_string())
            },
            _ => ReceivedMessage::None,
        }
    }
}

pub fn parse_message(raw_message: &str) -> ReceivedMessage {
    ReceivedMessage::new(raw_message.split(':').collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_parsing() {
        let exec = ReceivedMessage::Exec {
            command: "bundle exec rake".to_string(),
            application: Some("test_app".to_string()),
        };

        assert_eq!(parse_message("exec:bundle exec rake:test_app"), exec);
    }

    #[test]
    fn log_parsing() {
        let log = ReceivedMessage::Log {
            action: Some("-f".to_string()),
            application: Some("test_app".to_string()),
        };

        assert_eq!(parse_message("log:-f:test_app"), log);
    }

    #[test]
    fn dbms_parsing() {
        let dbms = ReceivedMessage::Dbms {
            action: "index_status".to_string(),
            application: Some("test_app".to_string()),
        };

        assert_eq!(parse_message("dbms:index_status:test_app"), dbms);
    }
}
