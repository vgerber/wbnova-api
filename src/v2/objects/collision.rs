use crate::v2::objects::collision_contact::CollisionContact;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Collision {
    pub id_of_layer: Option<String>,

    pub id_of_a: Option<String>,

    pub id_of_b: Option<String>,

    pub normal_root_on_b: Option<Vec<f64>>,

    pub position_on_a: Option<CollisionContact>,

    pub position_on_b: Option<CollisionContact>,
}
