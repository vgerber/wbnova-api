use ::reqwest;

use crate::v2::objects::cloud_disconnection_error::CloudDisconnectionError;

use crate::v2::objects::cloud_disconnection_status_disconnected::CloudDisconnectionStatusDisconnected;

use crate::v2::objects::cloud_disconnection_status_disconnecting::CloudDisconnectionStatusDisconnecting;

pub enum DisconnectFromNovaCloudResponseType {
    FailedDependency(CloudDisconnectionError),

    UndefinedResponse(reqwest::Response),

    Ok(CloudDisconnectionStatusDisconnected),

    Accepted(CloudDisconnectionStatusDisconnecting),
}

pub struct DisconnectFromNovaCloudPathParameters {}

pub struct DisconnectFromNovaCloudQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn disconnect_from_nova_cloud(
    client: &reqwest::Client,

    server: &str,

    query_parameters: DisconnectFromNovaCloudQueryParameters,
) -> Result<DisconnectFromNovaCloudResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .delete(format!("{}/experimental/cloud/config", server,))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        424 => match response.json::<CloudDisconnectionError>().await {
            Ok(cloud_disconnection_error) => Ok(
                DisconnectFromNovaCloudResponseType::FailedDependency(cloud_disconnection_error),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response
            .json::<CloudDisconnectionStatusDisconnected>()
            .await
        {
            Ok(cloud_disconnection_status_disconnected) => Ok(
                DisconnectFromNovaCloudResponseType::Ok(cloud_disconnection_status_disconnected),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        202 => match response
            .json::<CloudDisconnectionStatusDisconnecting>()
            .await
        {
            Ok(cloud_disconnection_status_disconnecting) => {
                Ok(DisconnectFromNovaCloudResponseType::Accepted(
                    cloud_disconnection_status_disconnecting,
                ))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(DisconnectFromNovaCloudResponseType::UndefinedResponse(
            response,
        )),
    }
}
