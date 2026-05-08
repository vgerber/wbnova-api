use crate::v2::objects::multi_collision_setup_dictionary::MultiCollisionSetupDictionary;

use crate::v2::objects::rrt_connect_algorithm::RrtConnectAlgorithm;

use crate::v2::objects::motion_group_setup_dictionary::MotionGroupSetupDictionary;

use crate::v2::objects::joint_ptp_motion_dictionary::JointPtpMotionDictionary;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MultiSearchCollisionFreeRequest {
    pub collision_setups: Option<MultiCollisionSetupDictionary>,

    pub algorithm_settings: Option<RrtConnectAlgorithm>,

    pub motion_group_setups_by_motion_group_key: MotionGroupSetupDictionary,

    pub path_definitions_by_motion_group_key: JointPtpMotionDictionary,
}
