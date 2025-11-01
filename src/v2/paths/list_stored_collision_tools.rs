use ::reqwest;

use crate::v2::objects::list_stored_collision_tools_ok_json::ListStoredCollisionToolsOkJson;

pub enum ListStoredCollisionToolsResponseType {
    Ok(ListStoredCollisionToolsOkJson),

    UndefinedResponse(reqwest::Response),
}

pub struct ListStoredCollisionToolsPathParameters {
    pub cell: String,
}

pub struct ListStoredCollisionToolsQueryParameters {}

pub async fn list_stored_collision_tools(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListStoredCollisionToolsPathParameters,
) -> Result<ListStoredCollisionToolsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/tools",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ListStoredCollisionToolsOkJson>().await {
            Ok(list_stored_collision_tools_ok_json) => Ok(
                ListStoredCollisionToolsResponseType::Ok(list_stored_collision_tools_ok_json),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListStoredCollisionToolsResponseType::UndefinedResponse(
            response,
        )),
    }
}
