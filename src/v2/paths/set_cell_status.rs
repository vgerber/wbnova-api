use ::reqwest;

use crate::v2::objects::error::Error;

pub enum SetCellStatusResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),
}

pub struct SetCellStatusPathParameters {
    pub cell: String,
}

pub struct SetCellStatusQueryParameters {
    pub operating_state: String,
}

pub async fn set_cell_status(
    client: &reqwest::Client,

    server: &str,

    path_parameters: SetCellStatusPathParameters,

    query_parameters: SetCellStatusQueryParameters,
) -> Result<SetCellStatusResponseType, reqwest::Error> {
    // Required Query Parameters
    let reqwest_query_parameters: Vec<(&str, String)> = vec![(
        "operating_state",
        query_parameters.operating_state.to_string(),
    )];

    let response = match client
        .put(format!("{}/cells/{}/status", server, path_parameters.cell))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetCellStatusResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetCellStatusResponseType::UndefinedResponse(response)),
    }
}
