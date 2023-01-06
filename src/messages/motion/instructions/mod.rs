use crate::messages::Message;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeLeg {
    leg: u8,
    thetas: nalgebra::Vector3<f64>,
}

impl Message for ChangeLeg {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec::<Self>(self).unwrap()
    }

    fn decode(buffer: &[u8]) -> Self {
        serde_json::from_slice::<Self>(buffer).unwrap()
    }

    fn routing_key(&self) -> &'static str {
        "motion.instructions.change_leg"
    }
}
