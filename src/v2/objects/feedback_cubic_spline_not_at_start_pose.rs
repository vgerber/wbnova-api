use crate::v2::objects::pose::Pose;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct FeedbackCubicSplineNotAtStartPose {
    pub start_pose: Option<Pose>,

    pub first_spline_pose: Option<Pose>,

    pub error_feedback_name: String,
}
