use crate::v2::objects::rrt_connect_algorithm::RrtConnectAlgorithm;

use crate::v2::objects::midpoint_insertion_algorithm::MidpointInsertionAlgorithm;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub enum CollisionFreeAlgorithm {
    RrtConnectAlgorithmValue(RrtConnectAlgorithm),

    MidpointInsertionAlgorithmValue(MidpointInsertionAlgorithm),
}
