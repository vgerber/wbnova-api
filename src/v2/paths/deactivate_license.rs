use ::reqwest;

pub enum DeactivateLicenseResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeactivateLicensePathParameters {}

pub struct DeactivateLicenseQueryParameters {}

pub async fn deactivate_license(
    client: &reqwest::Client,

    server: &str,
) -> Result<DeactivateLicenseResponseType, reqwest::Error> {
    let response = match client.delete(format!("{}/license", server,)).send().await {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(DeactivateLicenseResponseType::UndefinedResponse(response)),
    }
}
