use bytes::Bytes;
use log::debug;
use regex::Regex;
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
        let utf8 = String::from_utf8(bytes.to_vec()).unwrap();
        let mut result = vec![];
        for line in utf8.split('\n') {
            if line.is_empty() {
                continue;
            }
            result.push(self.parse_event(line));
        }
        result
    }

    fn parse_event(&self, event: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        debug!("Parsing event: {}", event);
        match self.regex.captures(event) {
            Some(caps) => {
                for name in self.regex.capture_names() {
                    if let Some(name) = name {
                        let cap = caps.name(name).unwrap();
                        map.insert(name.to_string(), cap.as_str().to_string());
                    }
                }
            }
            None => {}
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
