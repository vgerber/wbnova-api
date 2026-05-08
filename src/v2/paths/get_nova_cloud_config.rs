use ::reqwest;

use crate::v2::objects::cloud_config_status::CloudConfigStatus;

pub enum GetNovaCloudConfigResponseType {
    Ok(CloudConfigStatus),

    UndefinedResponse(reqwest::Response),
}

pub struct GetNovaCloudConfigPathParameters {}

pub struct GetNovaCloudConfigQueryParameters {}

pub async fn get_nova_cloud_config(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetNovaCloudConfigResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/experimental/cloud/config", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<CloudConfigStatus>().await {
            Ok(cloud_config_status) => Ok(GetNovaCloudConfigResponseType::Ok(cloud_config_status)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetNovaCloudConfigResponseType::UndefinedResponse(response)),
    }
}
