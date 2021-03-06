use nom::IResult;
use sodiumoxide::crypto::box_::{Nonce, NONCEBYTES};

pub const CONTEXT: [&'static str; 2] = ["dbms", "cache"];
pub const SEPARATOR: &'static str = "\u{fe34}";

#[derive(Debug, PartialEq)]
pub enum Message {
    Dbms {
        context: &'static str,
        action: String,
        application: Option<String>,
    },
    Cache {
        context: &'static str,
        action: String,
        application: Option<String>,
    },
    Invalid,
}

impl Message {
    pub fn new(parts: Vec<&str>) -> Message {
        match parts[0] {
            "dbms" => {
                Message::Dbms {
                    context: CONTEXT[0],
                    action: parts[1].to_string(),
                    application: Some(parts[2].to_string()),
                }
            }
            "cache" => {
                Message::Cache {
                    context: CONTEXT[1],
                    action: parts[1].to_string(),
                    application: Some(parts[2].to_string()),
                }
            }
            _ => Message::Invalid,
        }
    }

    pub fn from(raw_message: &str) -> Message {
        Message::new(raw_message.split(SEPARATOR).collect())
    }

    pub fn to_payload(self) -> String {
        match self {
            Message::Dbms { context, action, application } => {
                match application {
                    Some(app) => {
                        let mut message = context.to_string();

                        message.push_str(SEPARATOR);
                        message.push_str(&action);
                        message.push_str(SEPARATOR);
                        message.push_str(&app);

                        message
                    }
                    None => {
                        let mut message = context.to_string();

                        message.push_str(SEPARATOR);
                        message.push_str(&action);

                        message
                    }
                }
            }
            Message::Cache { context, action, application } => {
                match application {
                    Some(app) => {
                        let mut message = context.to_string();

                        message.push_str(SEPARATOR);
                        message.push_str(&action);
                        message.push_str(SEPARATOR);
                        message.push_str(&app);

                        message
                    }
                    None => {
                        let mut message = context.to_string();

                        message.push_str(SEPARATOR);
                        message.push_str(&action);

                        message
                    }
                }
            }
            Message::Invalid => "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Payload<'a> {
    pub requestor: String,
    pub nonce: Nonce,
    pub the_box: &'a [u8],
}

fn take_until_empty(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take!(input, input.len())
}

named!(parse_raw<&[u8], Payload>,
       chain!(
           requestor: take_until_and_consume!(SEPARATOR) ~
           nonce: take!(NONCEBYTES)                      ~
           the_box: take_until_empty,
           || {
                  let nonce = match Nonce::from_slice(nonce) {
                      Some(nonce) => nonce,
                      _ => panic!("Failed to extract Nonce!")
                  };

                  let requestor = match String::from_utf8(requestor.to_vec()) {
                      Ok(data) => data.trim_matches('\u{0}').to_string(),
                      _ => panic!("Failed to read Sender Information")
                  };

                  Payload {
                      requestor: requestor,
                      nonce: nonce,
                      the_box: the_box,
                  }
              }
       )
);

impl<'a> Payload<'a> {
    pub fn new(raw: &[u8]) -> Option<Payload> {
        match parse_raw(raw) {
            IResult::Done(_, payload) => Some(payload),
            IResult::Error(error) => panic!("Payload Error: {:?}", error),
            _ => None,
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

        assert_eq!(Message::from("dbms\u{fe34}index_status\u{fe34}test_app"), dbms);
    }

    #[test]
    fn cache_parsing() {
        let cache = Message::Cache {
            context: CONTEXT[1],
            action: "purge ^/.*$".to_string(),
            application: Some("atcms".to_string()),
        };

        assert_eq!(Message::from("cache\u{fe34}purge ^/.*$\u{fe34}atcms"), cache);
    }

    #[test]
    fn dbms_to_payload() {
        let dbms = Message::Dbms {
            context: CONTEXT[0],
            action: "index_status".to_string(),
            application: Some("test_app".to_string()),
        };

        assert_eq!(dbms.to_payload(), "dbms\u{fe34}index_status\u{fe34}test_app".to_string());
    }

    #[test]
    fn cache_to_payload() {
        let cache = Message::Cache {
            context: CONTEXT[1],
            action: "purge ^/.*$".to_string(),
            application: Some("atcms".to_string()),
        };

        assert_eq!(cache.to_payload(), "cache\u{fe34}purge ^/.*$\u{fe34}atcms".to_string());
    }
}
