use ::reqwest;

use crate::v2::objects::http_validation_error::HttpValidationError;

use crate::v2::objects::program_run::ProgramRun;

use crate::v2::objects::program_start_request::ProgramStartRequest;

pub enum StartProgramResponseType {
    UnprocessableEntity(HttpValidationError),

    UndefinedResponse(reqwest::Response),

    Ok(ProgramRun),
}

pub struct StartProgramPathParameters {
    pub cell: String,

    pub program: String,
}

pub struct StartProgramQueryParameters {}

pub async fn start_program(
    client: &reqwest::Client,

    server: &str,

    content: ProgramStartRequest,

    path_parameters: StartProgramPathParameters,
) -> Result<StartProgramResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/experimental/cells/{}/programs/{}/start",
            server, path_parameters.cell, path_parameters.program
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(StartProgramResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ProgramRun>().await {
            Ok(program_run) => Ok(StartProgramResponseType::Ok(program_run)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(StartProgramResponseType::UndefinedResponse(response)),
    }
}
