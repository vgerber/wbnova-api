use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::profinet_description::ProfinetDescription;

pub enum GetProfinetDescriptionResponseType {
    BadRequest(Error),

    NotFound(Error),

    Ok(ProfinetDescription),

    UndefinedResponse(reqwest::Response),
}

pub struct GetProfinetDescriptionPathParameters {
    pub cell: String,
}

pub struct GetProfinetDescriptionQueryParameters {}

pub async fn get_profinet_description(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetProfinetDescriptionPathParameters,
) -> Result<GetProfinetDescriptionResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/bus-ios/profinet/description",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetProfinetDescriptionResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ProfinetDescription>().await {
            Ok(profinet_description) => {
                Ok(GetProfinetDescriptionResponseType::Ok(profinet_description))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetProfinetDescriptionResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetProfinetDescriptionResponseType::UndefinedResponse(
            response,
        )),
    }
}
