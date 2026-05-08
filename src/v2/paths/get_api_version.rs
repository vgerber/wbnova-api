use ::reqwest;

use crate::v2::objects::api_version::ApiVersion;

pub enum GetApiVersionResponseType {
    Ok(ApiVersion),

    UndefinedResponse(reqwest::Response),
}

pub struct GetApiVersionPathParameters {}

pub struct GetApiVersionQueryParameters {}

pub async fn get_api_version(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetApiVersionResponseType, reqwest::Error> {
    let response = match client.get(format!("{}/version", server,)).send().await {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ApiVersion>().await {
            Ok(api_version) => Ok(GetApiVersionResponseType::Ok(api_version)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetApiVersionResponseType::UndefinedResponse(response)),
    }
}
