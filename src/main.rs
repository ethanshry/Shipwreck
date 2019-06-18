use lapin_async as lapin;

use crate::lapin::{
  BasicProperties, Channel, Connection, ConnectionProperties, ConsumerSubscriber,
  message::Delivery,
  options::*,
  types::FieldTable,
};

#[derive(Clone,Debug)]
struct Subscriber {
  channel: Channel,
}

impl ConsumerSubscriber for Subscriber {
    fn new_delivery(&self, delivery: Delivery) {
      print!("received message: {:?}", String::from_utf8_lossy(&delivery.data));
      self.channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).into_result().expect("basic_ack");
      println!(" - acked");
    }
    fn drop_prefetched_messages(&self) {}
    fn cancel(&self) {}
}

fn main() {

    // establish connection
    let addr = "amqp://127.0.0.1:5672/";
    let conn = Connection::connect(&addr, ConnectionProperties::default()).wait().expect("connection error");

    //receive channel
    let channel_b = conn.create_channel().wait().expect("create_channel");

    let queue = channel_b.queue_declare("deployment_trigger", QueueDeclareOptions::default(), FieldTable::default()).wait().expect("queue_declare");
  
    channel_b.basic_consume(&queue, "shipwreck_1", BasicConsumeOptions::default(), FieldTable::default(), Box::new(Subscriber { channel: channel_b.clone() })).wait().expect("basic_consume");

    loop {

    }
}