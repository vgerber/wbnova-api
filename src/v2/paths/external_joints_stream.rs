use ::reqwest;

use crate::v2::objects::external_joint_stream_datapoints::ExternalJointStreamDatapoints;

use crate::v2::objects::error::Error;

use crate::v2::objects::external_joint_stream_request::ExternalJointStreamRequest;

pub enum ExternalJointsStreamResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(ExternalJointStreamDatapoints),

    NotFound(Error),

    BadRequest(Error),
}

pub struct ExternalJointsStreamPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct ExternalJointsStreamQueryParameters {}

pub async fn external_joints_stream(
    client: &reqwest::Client,

    server: &str,

    content: ExternalJointStreamRequest,

    path_parameters: ExternalJointsStreamPathParameters,
) -> Result<ExternalJointsStreamResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/external-joints-stream",
            server, path_parameters.cell, path_parameters.controller
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ExternalJointStreamDatapoints>().await {
            Ok(external_joint_stream_datapoints) => Ok(ExternalJointsStreamResponseType::Ok(
                external_joint_stream_datapoints,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        400 => match response.json::<Error>().await {
            Ok(error) => Ok(ExternalJointsStreamResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ExternalJointsStreamResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ExternalJointsStreamResponseType::UndefinedResponse(
            response,
        )),
    }
}
