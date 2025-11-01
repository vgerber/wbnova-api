use ::reqwest;

use crate::v2::objects::configuration_resource_array::ConfigurationResourceArray;

pub enum ListConfigurationResourcesResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(ConfigurationResourceArray),
}

pub struct ListConfigurationResourcesPathParameters {}

pub struct ListConfigurationResourcesQueryParameters {}

pub async fn list_configuration_resources(
    client: &reqwest::Client,

    server: &str,
) -> Result<ListConfigurationResourcesResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/system/configuration/resources", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ConfigurationResourceArray>().await {
            Ok(configuration_resource_array) => Ok(ListConfigurationResourcesResponseType::Ok(
                configuration_resource_array,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListConfigurationResourcesResponseType::UndefinedResponse(
            response,
        )),
    }
}
