use ::reqwest;

use crate::v2::objects::program::Program;

pub enum GetProgramResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Program),
}

pub struct GetProgramPathParameters {
    pub program: String,

    pub cell: String,
}

pub struct GetProgramQueryParameters {}

pub async fn get_program(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetProgramPathParameters,
) -> Result<GetProgramResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/experimental/cells/{}/programs/{}",
            server, path_parameters.cell, path_parameters.program
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Program>().await {
            Ok(program) => Ok(GetProgramResponseType::Ok(program)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetProgramResponseType::UndefinedResponse(response)),
    }
}
