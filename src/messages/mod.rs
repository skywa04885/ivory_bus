pub mod kinematics;
pub mod motion;
pub mod bus_log;

pub trait Message {
    fn encode(&self) -> Vec<u8>;
    fn decode(buffer: &[u8]) -> Self;
    fn routing_key(&self) -> &'static str;
}
