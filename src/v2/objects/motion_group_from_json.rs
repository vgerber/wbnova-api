use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MotionGroupFromJson {
    pub motion_group: String,

    pub json_data: String,

    pub initial_joint_position: Option<String>,

    pub extracted_motion_group_id: String,
}
