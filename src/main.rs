use std::borrow::Cow;

use ivory_bus::{logger::Simple, Bus};

fn main() {
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<(Cow<'static, str>, Vec<u8>)>(100);

    let bus = std::sync::Arc::new(std::sync::Mutex::new(Bus::new(
        "amqp://127.0.0.1:5672".into(),
        "test".into(),
        vec![("test".into(), "log.*.*".into(), std::sync::Arc::new(sender))],
    )));

    let logger = std::boxed::Box::new(Simple::new(bus.clone(), "test".into()));
    log::set_boxed_logger(logger)
        .map(|()| log::set_max_level(log::LevelFilter::Trace))
        .unwrap();

    log::info!("test");

    while let Some((routing_key, buffer)) = receiver.blocking_recv() {
        println!("{} {}", routing_key, String::from_utf8(buffer).unwrap());
    }
}
