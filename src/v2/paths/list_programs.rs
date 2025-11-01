use ::reqwest;

use crate::v2::objects::program::Program;

pub enum ListProgramsResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Vec<Program>),
}

pub struct ListProgramsPathParameters {
    pub cell: String,
}

pub struct ListProgramsQueryParameters {}

pub async fn list_programs(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListProgramsPathParameters,
) -> Result<ListProgramsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/experimental/cells/{}/programs",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Vec<Program>>().await {
            Ok(program_keys) => Ok(ListProgramsResponseType::Ok(program_keys)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListProgramsResponseType::UndefinedResponse(response)),
    }
}
