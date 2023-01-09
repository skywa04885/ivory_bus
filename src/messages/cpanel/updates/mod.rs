use crate::messages::Message;

#[derive(serde::Serialize)]
pub struct PoseChange {
    pub timestamp: u128,
    pub leg_vertices: [[nalgebra::Vector3<f64>; 5]; 4],
    pub arm_vertices: [nalgebra::Vector3<f64>; 7],
}

impl PoseChange {
    pub fn new(
        timestamp: u128,
        leg_vertices: [[nalgebra::Vector3<f64>; 5]; 4],
        arm_vertices: [nalgebra::Vector3<f64>; 7],
    ) -> Self {
        Self {
            timestamp,
            leg_vertices,
            arm_vertices,
        }
    }

    pub fn builder(
        leg_vertices: [[nalgebra::Vector3<f64>; 5]; 4],
        arm_vertices: [nalgebra::Vector3<f64>; 7],
    ) -> PoseChangeBuilder {
        PoseChangeBuilder::new(leg_vertices, arm_vertices)
    }
}

impl Message for PoseChange {
    fn encode(&self) -> Vec<u8> {
        serde_json::to_vec::<Self>(self).unwrap()
    }

    fn decode(_buffer: &[u8]) -> Self {
        unimplemented!();
    }

    fn routing_key(&self) -> &'static str {
        "cpanel.updates.pose_change"
    }
}

pub struct PoseChangeBuilder {
    timestamp: u128,
    leg_vertices: [[nalgebra::Vector3<f64>; 5]; 4],
    arm_vertices: [nalgebra::Vector3<f64>; 7],
}

impl PoseChangeBuilder {
    pub fn new(
        leg_vertices: [[nalgebra::Vector3<f64>; 5]; 4],
        arm_vertices: [nalgebra::Vector3<f64>; 7],
    ) -> Self {
        let timestamp: u128 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Self {
            timestamp,
            leg_vertices,
            arm_vertices,
        }
    }

    pub fn build(self) -> PoseChange {
        PoseChange::new(self.timestamp, self.leg_vertices, self.arm_vertices)
    }
}
