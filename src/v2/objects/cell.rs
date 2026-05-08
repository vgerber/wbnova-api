use crate::v2::objects::robot_controller::RobotController;

use crate::v2::objects::app::App;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Cell {
    pub version: Option<String>,

    pub name: String,

    pub controllers: Option<Vec<RobotController>>,

    pub apps: Option<Vec<App>>,

    pub description: Option<String>,
}
