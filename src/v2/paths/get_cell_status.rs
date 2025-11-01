use ::reqwest;

use crate::v2::objects::service_status_response::ServiceStatusResponse;

use crate::v2::objects::error::Error;

pub enum GetCellStatusResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(ServiceStatusResponse),

    NotFound(Error),
}

pub struct GetCellStatusPathParameters {
    pub cell: String,
}

pub struct GetCellStatusQueryParameters {}

pub async fn get_cell_status(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetCellStatusPathParameters,
) -> Result<GetCellStatusResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/cells/{}/status", server, path_parameters.cell))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ServiceStatusResponse>().await {
            Ok(service_status_response) => {
                Ok(GetCellStatusResponseType::Ok(service_status_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCellStatusResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetCellStatusResponseType::UndefinedResponse(response)),
    }
}
