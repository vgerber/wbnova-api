use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::license::License;

use crate::v2::objects::activate_license_request::ActivateLicenseRequest;

pub enum ActivateLicenseResponseType {
    Forbidden(Error),

    Ok(License),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct ActivateLicensePathParameters {}

pub struct ActivateLicenseQueryParameters {}

pub async fn activate_license(
    client: &reqwest::Client,

    server: &str,

    content: ActivateLicenseRequest,
) -> Result<ActivateLicenseResponseType, reqwest::Error> {
    let response = match client
        .post(format!("{}/license", server,))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ActivateLicenseResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        403 => match response.json::<Error>().await {
            Ok(error) => Ok(ActivateLicenseResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<License>().await {
            Ok(license) => Ok(ActivateLicenseResponseType::Ok(license)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ActivateLicenseResponseType::UndefinedResponse(response)),
    }
}
