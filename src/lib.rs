use messages::Message;

pub mod logger;
pub mod messages;

pub struct Bus {
    out_sender: tokio::sync::mpsc::Sender<(std::borrow::Cow<'static, str>, Vec<u8>)>,
    handle: std::thread::JoinHandle<()>,
}

impl Bus {
    pub fn new(
        uri: std::borrow::Cow<'static, str>,
        exchange: std::borrow::Cow<'static, str>,
        listeners: Vec<(
            std::borrow::Cow<'static, str>,
            std::borrow::Cow<'static, str>,
            std::sync::Arc<tokio::sync::mpsc::Sender<(std::borrow::Cow<'static, str>, Vec<u8>)>>,
        )>,
    ) -> Self {
        let (out_sender, mut out_receiver) =
            tokio::sync::mpsc::channel::<(std::borrow::Cow<'static, str>, Vec<u8>)>(128);

        let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            let runtime: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async move {
                let connection_properties: lapin::ConnectionProperties =
                    lapin::ConnectionProperties::default()
                        .with_reactor(tokio_reactor_trait::Tokio)
                        .with_executor(tokio_executor_trait::Tokio::current());

                let connection: lapin::Connection =
                    lapin::Connection::connect(uri.as_ref(), connection_properties)
                        .await
                        .unwrap();

                let channel: lapin::Channel = connection.create_channel().await.unwrap();

                let exchange_kind: lapin::ExchangeKind = lapin::ExchangeKind::Topic;
                let exchange_options: lapin::options::ExchangeDeclareOptions =
                    lapin::options::ExchangeDeclareOptions::default();
                let exchange_arguments: lapin::types::FieldTable =
                    lapin::types::FieldTable::default();

                channel
                    .exchange_declare(
                        &exchange,
                        exchange_kind,
                        exchange_options,
                        exchange_arguments,
                    )
                    .await
                    .unwrap();

                for (consumer_tag, routing_key, sender) in listeners {
                    let mut queue_options: lapin::options::QueueDeclareOptions =
                        lapin::options::QueueDeclareOptions::default();
                    queue_options.auto_delete = true;
                    let queue_arguments: lapin::types::FieldTable =
                        lapin::types::FieldTable::default();

                    let queue: lapin::Queue = channel
                        .queue_declare("", queue_options, queue_arguments)
                        .await
                        .unwrap();

                    let queue_bind_options: lapin::options::QueueBindOptions =
                        lapin::options::QueueBindOptions::default();
                    let queue_bind_arguments: lapin::types::FieldTable =
                        lapin::types::FieldTable::default();

                    channel
                        .queue_bind(
                            queue.name().as_str(),
                            &exchange,
                            &routing_key,
                            queue_bind_options,
                            queue_bind_arguments,
                        )
                        .await
                        .unwrap();

                    let queue_consumer_options: lapin::options::BasicConsumeOptions =
                        lapin::options::BasicConsumeOptions::default();
                    let queue_consumer_arguments: lapin::types::FieldTable =
                        lapin::types::FieldTable::default();

                    let consumer: lapin::Consumer = channel
                        .basic_consume(
                            queue.name().as_str(),
                            &consumer_tag,
                            queue_consumer_options,
                            queue_consumer_arguments,
                        )
                        .await
                        .unwrap();

                    consumer.set_delegate(move |delivery_result| {
                        let sender = sender.clone();

                        async move {
                            let delivery: lapin::message::Delivery = match delivery_result {
                                Ok(Some(delivery)) => delivery,
                                Ok(None) => return,
                                Err(error) => {
                                    println!("Failed to receive message: {}", error);
                                    return;
                                }
                            };

                            sender
                                .send((
                                    delivery.routing_key.to_string().into(),
                                    delivery.data.clone(),
                                ))
                                .await
                                .unwrap();

                            let ack_options: lapin::options::BasicAckOptions =
                                lapin::options::BasicAckOptions::default();
                            delivery.ack(ack_options).await.unwrap();
                        }
                    });
                }

                while let Some((routing_key, buffer)) = out_receiver.recv().await {
                    let options: lapin::options::BasicPublishOptions =
                        lapin::options::BasicPublishOptions::default();
                    let properties: lapin::BasicProperties = lapin::BasicProperties::default();

                    channel
                        .basic_publish(&exchange, &routing_key, options, &buffer, properties)
                        .await
                        .unwrap();
                }
            });
        });

        Self { out_sender, handle }
    }

    pub fn publish(&mut self, routing_key: std::borrow::Cow<'static, str>, buffer: Vec<u8>) {
        self.out_sender
            .blocking_send((routing_key, buffer))
            .unwrap();
    }

    pub fn publish_message<T: Message>(&mut self, message: T) {
        self.publish(message.routing_key().into(), message.encode())
    }
}
