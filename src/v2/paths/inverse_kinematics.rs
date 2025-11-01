use ::reqwest;

use crate::v2::objects::inverse_kinematics_422_response::InverseKinematics422Response;

use crate::v2::objects::inverse_kinematics_response::InverseKinematicsResponse;

use crate::v2::objects::inverse_kinematics_request::InverseKinematicsRequest;

pub enum InverseKinematicsResponseType {
    Ok(InverseKinematicsResponse),

    UnprocessableEntity(InverseKinematics422Response),

    UndefinedResponse(reqwest::Response),
}

pub struct InverseKinematicsPathParameters {
    pub cell: String,
}

pub struct InverseKinematicsQueryParameters {}

pub async fn inverse_kinematics(
    client: &reqwest::Client,

    server: &str,

    content: InverseKinematicsRequest,

    path_parameters: InverseKinematicsPathParameters,
) -> Result<InverseKinematicsResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/cells/{}/kinematic/inverse",
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
        422 => match response.json::<InverseKinematics422Response>().await {
            Ok(inverse_kinematics_422_response) => Ok(
                InverseKinematicsResponseType::UnprocessableEntity(inverse_kinematics_422_response),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<InverseKinematicsResponse>().await {
            Ok(inverse_kinematics_response) => Ok(InverseKinematicsResponseType::Ok(
                inverse_kinematics_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(InverseKinematicsResponseType::UndefinedResponse(response)),
    }
}
