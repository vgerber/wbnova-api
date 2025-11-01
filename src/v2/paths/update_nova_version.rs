use ::reqwest;

use crate::v2::objects::update_nova_version_request::UpdateNovaVersionRequest;

pub enum UpdateNovaVersionResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct UpdateNovaVersionPathParameters {}

pub struct UpdateNovaVersionQueryParameters {}

pub async fn update_nova_version(
    client: &reqwest::Client,

    server: &str,

    content: UpdateNovaVersionRequest,
) -> Result<UpdateNovaVersionResponseType, reqwest::Error> {
    let response = match client
        .put(format!("{}/system/update", server,))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(UpdateNovaVersionResponseType::UndefinedResponse(response)),
    }
}
