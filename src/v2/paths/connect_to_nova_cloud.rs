use ::reqwest;

use crate::v2::objects::cloud_registration_success_response::CloudRegistrationSuccessResponse;

use crate::v2::objects::cloud_connection_error::CloudConnectionError;

use crate::v2::objects::zod_validation_error::ZodValidationError;

use crate::v2::objects::cloud_connection_request::CloudConnectionRequest;

pub enum ConnectToNovaCloudResponseType {
    Accepted(CloudRegistrationSuccessResponse),

    BadRequest(ZodValidationError),

    FailedDependency(CloudConnectionError),

    UndefinedResponse(reqwest::Response),

    Ok(CloudRegistrationSuccessResponse),
}

pub struct ConnectToNovaCloudPathParameters {}

pub struct ConnectToNovaCloudQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn connect_to_nova_cloud(
    client: &reqwest::Client,

    server: &str,

    content: CloudConnectionRequest,

    query_parameters: ConnectToNovaCloudQueryParameters,
) -> Result<ConnectToNovaCloudResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        reqwest_query_parameters.push(("completion_timeout", query_parameter.to_string()));
    }

    let response = match client
        .post(format!("{}/experimental/cloud/connect", server,))
        .query(&reqwest_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<CloudRegistrationSuccessResponse>().await {
            Ok(cloud_registration_success_response) => Ok(ConnectToNovaCloudResponseType::Ok(
                cloud_registration_success_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        424 => match response.json::<CloudConnectionError>().await {
            Ok(cloud_connection_error) => Ok(ConnectToNovaCloudResponseType::FailedDependency(
                cloud_connection_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<ZodValidationError>().await {
            Ok(zod_validation_error) => Ok(ConnectToNovaCloudResponseType::BadRequest(
                zod_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        202 => match response.json::<CloudRegistrationSuccessResponse>().await {
            Ok(cloud_registration_success_response) => Ok(
                ConnectToNovaCloudResponseType::Accepted(cloud_registration_success_response),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ConnectToNovaCloudResponseType::UndefinedResponse(response)),
    }
}
