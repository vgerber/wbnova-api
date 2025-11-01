use ::reqwest;

use crate::v2::objects::license::License;

use crate::v2::objects::error::Error;

pub enum GetLicenseResponseType {
    Ok(License),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct GetLicensePathParameters {}

pub struct GetLicenseQueryParameters {}

pub async fn get_license(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetLicenseResponseType, reqwest::Error> {
    let response = match client.get(format!("{}/license", server,)).send().await {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<License>().await {
            Ok(license) => Ok(GetLicenseResponseType::Ok(license)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetLicenseResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetLicenseResponseType::UndefinedResponse(response)),
    }
}
