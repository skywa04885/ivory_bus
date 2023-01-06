use crate::{messages::Message, Bus};

pub struct Simple {
    origin: std::borrow::Cow<'static, str>,
    log_record_sender: std::sync::Mutex<std::sync::mpsc::Sender<(String, Vec<u8>)>>,
    handle: std::thread::JoinHandle<()>,
}

impl Simple {
    pub fn new(
        bus: std::sync::Arc<std::sync::Mutex<Bus>>,
        origin: std::borrow::Cow<'static, str>,
    ) -> Self {
        let (log_record_sender, log_record_receiver) =
            std::sync::mpsc::channel::<(String, Vec<u8>)>();

        let log_record_sender = std::sync::Mutex::new(log_record_sender);

        let handle: std::thread::JoinHandle<()> = {
            let bus = bus.clone();
            std::thread::spawn(move || {
                while let Ok((routing_key, buffer)) = log_record_receiver.recv() {
                    bus.lock().unwrap().publish(routing_key.into(), buffer);
                }
            })
        };

        Self {
            origin,
            log_record_sender,
            handle,
        }
    }
}

impl log::Log for Simple {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message: std::borrow::Cow<'static, str> =
            std::borrow::Cow::Owned(format!("{}", record.args()));

        let message: super::messages::bus_log::Record =
            super::messages::bus_log::Record::builder(message)
                .group("Global".into())
                .build();

        let routing_key: String =
            format!("log.{}.{}", self.origin, record.metadata().level().as_str());

        self.log_record_sender
            .lock()
            .unwrap()
            .send((routing_key, message.encode()))
            .unwrap();
    }

    fn flush(&self) {}
}
