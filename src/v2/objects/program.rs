use crate::v2::objects::program_input_json_schema::ProgramInputJsonSchema;

use crate::v2::objects::preconditions_before_the_program_can_be_started::PreconditionsBeforeTheProgramCanBeStarted;

use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct Program {
    pub program: String,

    pub input_schema: Option<ProgramInputJsonSchema>,

    pub preconditions: Option<PreconditionsBeforeTheProgramCanBeStarted>,

    pub description: Option<String>,

    pub app: String,

    pub name: Option<String>,
}
