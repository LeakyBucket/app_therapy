pub const CONTEXT: [&'static str; 2] = ["dbms", "cache"];
pub const SEPARATOR: &'static str = ":";

#[derive(Debug, PartialEq)]
pub enum Message {
    Dbms { context: &'static str, action: String, application: Option<String> },
    Cache { context: &'static str, action: String, application: Option<String> },
    Invalid,
}

impl Message {
    fn new(parts: Vec<&str>) -> Message {
        match parts[0] {
            "dbms" => Message::Dbms {
                context: CONTEXT[0],
                action: parts[1].to_string(),
                application: Some(parts[2].to_string())
            },
            "cache" => Message::Cache {
                context: CONTEXT[1],
                action: parts[1].to_string(),
                application: Some(parts[2].to_string())
            },
            _ => Message::Invalid,
        }
    }

    fn from(raw_message: &str) -> Message {
        Message::new(raw_message.split(SEPARATOR).collect())
    }

    fn to_payload(self) -> String {
        match self {
            Message::Dbms{ context, action, application } => match application {
                Some(app) => {
                    let mut message = context.to_string();

                    message.push_str(SEPARATOR);
                    message.push_str(&action);
                    message.push_str(SEPARATOR);
                    message.push_str(&app);

                    message
                },
                None => {
                    let mut message = context.to_string();

                    message.push_str(SEPARATOR);
                    message.push_str(&action);

                    message
                },
            },
            Message::Cache{ context, action, application  } => match application {
                Some(app) => {
                    let mut message = context.to_string();

                    message.push_str(SEPARATOR);
                    message.push_str(&action);
                    message.push_str(SEPARATOR);
                    message.push_str(&app);

                    message
                },
                None => {
                    let mut message = context.to_string();

                    message.push_str(SEPARATOR);
                    message.push_str(&action);

                    message
                },
            },
            Message::Invalid => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dbms_parsing() {
        let dbms = Message::Dbms {
            context: CONTEXT[0],
            action: "index_status".to_string(),
            application: Some("test_app".to_string()),
        };

        assert_eq!(Message::from("dbms:index_status:test_app"), dbms);
    }

    #[test]
    fn cache_parsing() {
        let cache = Message::Cache {
            context: CONTEXT[1],
            action: "purge ^/.*$".to_string(),
            application: Some("atcms".to_string()),
        };

        assert_eq!(Message::from("cache:purge ^/.*$:atcms"), cache);
    }

    #[test]
    fn dbms_to_payload() {
        let dbms = Message::Dbms {
            context: CONTEXT[0],
            action: "index_status".to_string(),
            application: Some("test_app".to_string()),
        };

        assert_eq!(dbms.to_payload(), "dbms:index_status:test_app".to_string());
    }

    #[test]
    fn cache_to_payload() {
        let cache = Message::Cache {
            context: CONTEXT[1],
            action: "purge ^/.*$".to_string(),
            application: Some("atcms".to_string()),
        };

        assert_eq!(cache.to_payload(), "cache:purge ^/.*$:atcms".to_string());
    }
}
