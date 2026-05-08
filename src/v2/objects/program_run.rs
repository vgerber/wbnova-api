use crate::v2::objects::input_data::InputData;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct ProgramRun {
    pub input_data: Option<InputData>,

    pub run: String,

    pub error: Option<String>,

    pub stderr: Option<String>,

    pub program: String,

    pub logs: Option<String>,

    pub end_time: Option<String>,

    pub stdout: Option<String>,

    pub traceback: Option<String>,

    pub state: String,

    pub start_time: Option<String>,

    pub app: Option<String>,
}
