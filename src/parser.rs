use bytes::Bytes;
use regex::bytes::Regex;
use std::collections::HashMap;

/// Parser splits log by line into events, then parse each event to fields with regex.
pub struct Parser {
    regex: Regex,
}

impl Parser {
    pub fn new(regex: &str) -> Self {
        Self {
            regex: Regex::new(regex).unwrap(),
        }
    }

    pub fn parse(&self, bytes: Bytes) -> Vec<HashMap<String, String>> {
        let mut result = vec![];
        for line in bytes.split(|&char| char == b'\n') {
            if line.is_empty() {
                continue;
            }
            result.push(self.parse_event(line));
        }
        result
    }

    fn parse_event(&self, event: &[u8]) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for caps in self.regex.captures_iter(event) {
            for name in self.regex.capture_names() {
                if let Some(name) = name {
                    let cap = caps.name(name).unwrap();
                    map.insert(
                        name.to_string(),
                        String::from_utf8_lossy(cap.as_bytes()).to_string(),
                    );
                }
            }
        }
        if map.is_empty() {
            panic!(
                "Event does not match regex: {}",
                String::from_utf8_lossy(event)
            );
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    fn it_works() {
        let _ = env_logger::builder().is_test(true).try_init();
        let parser = Parser::new("f=(?P<f>\\w+)?");
        let bytes = Bytes::from("f=1\nf=2\n");
        let events = parser.parse(bytes);
        debug!("{:#?}", events);
        assert_eq!(2, events.len());
        let mut first = HashMap::new();
        first.insert("f".to_string(), "1".to_string());
        let mut second = HashMap::new();
        second.insert("f".to_string(), "2".to_string());
        assert_eq!(vec![first, second], events);
    }

    #[test]
    fn rust_env_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
        let parser = Parser::new(
            r"\[(?P<timestamp>\S+)\s+(?P<level>\S+)\s+(?P<class>\S+)]\s+(?P<content>.*)",
        );
        let bytes = Bytes::from(["[2021-04-18T21:51:25Z TRACE hyper::proto::h1::conn] flushed({role=client}): State { reading: Init, writing: Init, keep_alive: Busy }",
            "[2021-04-18T21:51:25Z TRACE want] poll_want: taker wants!"
        ].join("\n"));
        let events = parser.parse(bytes);
        debug!("{:#?}", events);
    }
}
