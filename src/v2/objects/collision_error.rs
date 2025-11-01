use crate::v2::objects::feedback_collision::FeedbackCollision;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct CollisionError {
    pub collision: Option<FeedbackCollision>,
}
