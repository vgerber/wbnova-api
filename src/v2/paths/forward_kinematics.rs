use ::reqwest;

use crate::v2::objects::forward_kinematics_422_response::ForwardKinematics422Response;

use crate::v2::objects::forward_kinematics_response::ForwardKinematicsResponse;

use crate::v2::objects::forward_kinematics_request::ForwardKinematicsRequest;

pub enum ForwardKinematicsResponseType {
    UndefinedResponse(reqwest::Response),

    UnprocessableEntity(ForwardKinematics422Response),

    Ok(ForwardKinematicsResponse),
}

pub struct ForwardKinematicsPathParameters {
    pub cell: String,
}

pub struct ForwardKinematicsQueryParameters {}

pub async fn forward_kinematics(
    client: &reqwest::Client,

    server: &str,

    content: ForwardKinematicsRequest,

    path_parameters: ForwardKinematicsPathParameters,
) -> Result<ForwardKinematicsResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/cells/{}/kinematic/forward",
            server, path_parameters.cell
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        422 => match response.json::<ForwardKinematics422Response>().await {
            Ok(forward_kinematics_422_response) => Ok(
                ForwardKinematicsResponseType::UnprocessableEntity(forward_kinematics_422_response),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ForwardKinematicsResponse>().await {
            Ok(forward_kinematics_response) => Ok(ForwardKinematicsResponseType::Ok(
                forward_kinematics_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ForwardKinematicsResponseType::UndefinedResponse(response)),
    }
}
