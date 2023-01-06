use crate::messages::Message;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangePawPosition {
    leg: u8,
    relative: bool,
    position: nalgebra::Vector3<f64>,
}

impl Message for ChangePawPosition {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec::<Self>(self).unwrap()
    }

    fn decode(buffer: &[u8]) -> Self {
        serde_json::from_slice::<Self>(buffer).unwrap()
    }

    fn routing_key(&self) -> &'static str {
        "kinematics.instructions.change_paw_position"
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeTorsoPosition {
    leg: u8,
    relative: bool,
    position: nalgebra::Vector3<f64>,
}

impl Message for ChangeTorsoPosition {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec::<Self>(self).unwrap()
    }

    fn decode(buffer: &[u8]) -> Self {
        serde_json::from_slice::<Self>(buffer).unwrap()
    }

    fn routing_key(&self) -> &'static str {
        "kinematics.instructions.change_torso_position"
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeTorsoOrientation {
    leg: u8,
    relative: bool,
    orientation: nalgebra::Vector3<f64>,
}

impl Message for ChangeTorsoOrientation {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec::<Self>(self).unwrap()
    }

    fn decode(buffer: &[u8]) -> Self {
        serde_json::from_slice::<Self>(buffer).unwrap()
    }

    fn routing_key(&self) -> &'static str {
        "kinematics.instructions.change_torso_orientation"
    }
}
