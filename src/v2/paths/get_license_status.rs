use ::reqwest;

use crate::v2::objects::license_status::LicenseStatus;

pub enum GetLicenseStatusResponseType {
    UndefinedResponse(reqwest::Response),

    Forbidden(LicenseStatus),

    Ok(LicenseStatus),
}

pub struct GetLicenseStatusPathParameters {}

pub struct GetLicenseStatusQueryParameters {}

pub async fn get_license_status(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetLicenseStatusResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/license/status", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<LicenseStatus>().await {
            Ok(license_status) => Ok(GetLicenseStatusResponseType::Ok(license_status)),
            Err(parsing_error) => Err(parsing_error),
        },

        403 => match response.json::<LicenseStatus>().await {
            Ok(license_status) => Ok(GetLicenseStatusResponseType::Forbidden(license_status)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetLicenseStatusResponseType::UndefinedResponse(response)),
    }
}
