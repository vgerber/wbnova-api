use ::reqwest;

use crate::v2::objects::link_chain::LinkChain;

pub enum GetMotionGroupCollisionModelResponseType {
    Ok(LinkChain),

    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionGroupCollisionModelPathParameters {
    pub motion_group_model: String,
}

pub struct GetMotionGroupCollisionModelQueryParameters {}

pub async fn get_motion_group_collision_model(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetMotionGroupCollisionModelPathParameters,
) -> Result<GetMotionGroupCollisionModelResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/motion-group-models/{}/collision",
            server, path_parameters.motion_group_model
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<LinkChain>().await {
            Ok(link_chain) => Ok(GetMotionGroupCollisionModelResponseType::Ok(link_chain)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetMotionGroupCollisionModelResponseType::UndefinedResponse(
            response,
        )),
    }
}
