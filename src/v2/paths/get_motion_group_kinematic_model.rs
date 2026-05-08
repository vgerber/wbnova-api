use ::reqwest;

use crate::v2::objects::kinematic_model::KinematicModel;

pub enum GetMotionGroupKinematicModelResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(KinematicModel),
}

pub struct GetMotionGroupKinematicModelPathParameters {
    pub motion_group_model: String,
}

pub struct GetMotionGroupKinematicModelQueryParameters {}

pub async fn get_motion_group_kinematic_model(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetMotionGroupKinematicModelPathParameters,
) -> Result<GetMotionGroupKinematicModelResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/motion-group-models/{}/kinematic",
            server, path_parameters.motion_group_model
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<KinematicModel>().await {
            Ok(kinematic_model) => Ok(GetMotionGroupKinematicModelResponseType::Ok(
                kinematic_model,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetMotionGroupKinematicModelResponseType::UndefinedResponse(
            response,
        )),
    }
}
