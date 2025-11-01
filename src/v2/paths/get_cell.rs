use ::reqwest;

use crate::v2::objects::cell::Cell;

use crate::v2::objects::error::Error;

pub enum GetCellResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    Ok(Cell),
}

pub struct GetCellPathParameters {
    pub cell: String,
}

pub struct GetCellQueryParameters {}

pub async fn get_cell(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetCellPathParameters,
) -> Result<GetCellResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/cells/{}", server, path_parameters.cell))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Cell>().await {
            Ok(cell) => Ok(GetCellResponseType::Ok(cell)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCellResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetCellResponseType::UndefinedResponse(response)),
    }
}
