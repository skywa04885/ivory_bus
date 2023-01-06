use std::iter::empty;

use crate::messages::Message;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChangeLeg {
    pub leg: u8,
    pub thetas: nalgebra::Vector3<f64>,
    pub velocities: nalgebra::Vector3<f64>,
}

impl ChangeLeg {
    pub fn new(
        leg: u8,
        thetas: nalgebra::Vector3<f64>,
        velocities: nalgebra::Vector3<f64>,
    ) -> Self {
        Self {
            leg,
            thetas,
            velocities,
        }
    }

    pub fn builder() -> ChangeLegBuilder {
        ChangeLegBuilder::new()
    }
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

pub struct ChangeLegBuilder {
    pub leg: u8,
    pub thetas: nalgebra::Vector3<f64>,
    pub velocities: nalgebra::Vector3<f64>,
}

impl ChangeLegBuilder {
    pub fn new() -> Self {
        Self {
            leg: 0,
            thetas: nalgebra::Vector3::<f64>::zeros(),
            velocities: nalgebra::Vector3::<f64>::zeros(),
        }
    }

    pub fn leg(mut self, leg: u8) -> Self {
        self.leg = leg;

        self
    }

    pub fn thetas(mut self, thetas: nalgebra::Vector3<f64>) -> Self {
        self.thetas = thetas;

        self
    }

    pub fn velocities(mut self, velocities: nalgebra::Vector3<f64>) -> Self {
        self.velocities = velocities;

        self
    }

    pub fn build(self) -> ChangeLeg {
        ChangeLeg::new(self.leg, self.thetas, self.velocities)
    }
}
