use ::reqwest;

use crate::v2::objects::collision_motion_group_tool::CollisionMotionGroupTool;

pub enum StoreCollisionToolResponseType {
    Ok(CollisionMotionGroupTool),

    UndefinedResponse(reqwest::Response),
}

pub struct StoreCollisionToolPathParameters {
    pub cell: String,

    pub tool: String,
}

pub struct StoreCollisionToolQueryParameters {}

pub async fn store_collision_tool(
    client: &reqwest::Client,

    server: &str,

    content: CollisionMotionGroupTool,

    path_parameters: StoreCollisionToolPathParameters,
) -> Result<StoreCollisionToolResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/store/collision/tools/{}",
            server, path_parameters.cell, path_parameters.tool
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<CollisionMotionGroupTool>().await {
            Ok(collision_motion_group_tool) => Ok(StoreCollisionToolResponseType::Ok(
                collision_motion_group_tool,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(StoreCollisionToolResponseType::UndefinedResponse(response)),
    }
}
