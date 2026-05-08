use ::reqwest;

pub enum StopProgramResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct StopProgramPathParameters {
    pub cell: String,

    pub program: String,
}

pub struct StopProgramQueryParameters {}

pub async fn stop_program(
    client: &reqwest::Client,

    server: &str,

    path_parameters: StopProgramPathParameters,
) -> Result<StopProgramResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/experimental/cells/{}/programs/{}/stop",
            server, path_parameters.cell, path_parameters.program
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(StopProgramResponseType::UndefinedResponse(response)),
    }
}
