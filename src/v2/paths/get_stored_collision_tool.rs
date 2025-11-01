use ::reqwest;

use crate::v2::objects::collision_motion_group_tool::CollisionMotionGroupTool;

pub enum GetStoredCollisionToolResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(CollisionMotionGroupTool),
}

pub struct GetStoredCollisionToolPathParameters {
    pub tool: String,

    pub cell: String,
}

pub struct GetStoredCollisionToolQueryParameters {}

pub async fn get_stored_collision_tool(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetStoredCollisionToolPathParameters,
) -> Result<GetStoredCollisionToolResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/tools/{}",
            server, path_parameters.cell, path_parameters.tool
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<CollisionMotionGroupTool>().await {
            Ok(collision_motion_group_tool) => Ok(GetStoredCollisionToolResponseType::Ok(
                collision_motion_group_tool,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetStoredCollisionToolResponseType::UndefinedResponse(
            response,
        )),
    }
}
