use crate::v2::objects::merge_trajectories_segment::MergeTrajectoriesSegment;

use crate::v2::objects::motion_group_setup::MotionGroupSetup;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct MergeTrajectoriesRequest {
    pub trajectory_segments: Vec<MergeTrajectoriesSegment>,

    pub motion_group_setup: MotionGroupSetup,
}
