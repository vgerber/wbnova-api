use ::reqwest;

pub enum ListStoredCollisionToolsKeysResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Vec<String>),
}

pub struct ListStoredCollisionToolsKeysPathParameters {
    pub cell: String,
}

pub struct ListStoredCollisionToolsKeysQueryParameters {}

pub async fn list_stored_collision_tools_keys(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListStoredCollisionToolsKeysPathParameters,
) -> Result<ListStoredCollisionToolsKeysResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/tools-keys",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Vec<String>>().await {
            Ok(tool_keys) => Ok(ListStoredCollisionToolsKeysResponseType::Ok(tool_keys)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListStoredCollisionToolsKeysResponseType::UndefinedResponse(
            response,
        )),
    }
}
