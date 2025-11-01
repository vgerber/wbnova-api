use crate::v2::objects::app::App;

use crate::v2::objects::robot_controller::RobotController;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Cell {
    pub apps: Option<Vec<App>>,

    pub name: String,

    pub controllers: Option<Vec<RobotController>>,

    pub description: Option<String>,
}
