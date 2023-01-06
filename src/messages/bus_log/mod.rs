use crate::messages::Message;

#[derive(serde::Serialize)]
pub struct Record {
    pub timestamp: u128,
    pub message: std::borrow::Cow<'static, str>,
    pub group: Option<std::borrow::Cow<'static, str>>,
}

impl Record {
    pub fn new(
        timestamp: u128,
        message: std::borrow::Cow<'static, str>,
        group: Option<std::borrow::Cow<'static, str>>,
    ) -> Self {
        Self {
            timestamp,
            message,
            group,
        }
    }

    pub fn builder(message: std::borrow::Cow<'static, str>) -> RecordBuilder {
        RecordBuilder::new(message)
    }
}

impl Message for Record {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec::<Self>(self).unwrap()
    }

    fn decode(_buffer: &[u8]) -> Self {
        unimplemented!();
    }

    fn routing_key(&self) -> &'static str {
        unimplemented!();
    }
}

pub struct RecordBuilder {
    timestamp: u128,
    message: std::borrow::Cow<'static, str>,
    group: Option<std::borrow::Cow<'static, str>>,
}

impl RecordBuilder {
    pub fn new(message: std::borrow::Cow<'static, str>) -> Self {
        let timestamp: u128 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Self {
            timestamp,
            message,
            group: None,
        }
    }

    pub fn group(mut self, group: std::borrow::Cow<'static, str>) -> Self {
        self.group = Some(group);

        self
    }

    pub fn build(self) -> Record {
        Record::new(self.timestamp, self.message, self.group)
    }
}
