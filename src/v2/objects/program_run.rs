use crate::v2::objects::input_data::InputData;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProgramRun {
    pub start_time: Option<String>,

    pub program: String,

    pub state: String,

    pub stdout: Option<String>,

    pub input_data: Option<InputData>,

    pub traceback: Option<String>,

    pub error: Option<String>,

    pub stderr: Option<String>,

    pub logs: Option<String>,

    pub app: Option<String>,

    pub run: String,

    pub end_time: Option<String>,
}
