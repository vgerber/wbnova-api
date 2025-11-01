use ::reqwest;

pub enum DeleteStoredCollisionToolResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteStoredCollisionToolPathParameters {
    pub tool: String,

    pub cell: String,
}

pub struct DeleteStoredCollisionToolQueryParameters {}

pub async fn delete_stored_collision_tool(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteStoredCollisionToolPathParameters,
) -> Result<DeleteStoredCollisionToolResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
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
        _ => Ok(DeleteStoredCollisionToolResponseType::UndefinedResponse(
            response,
        )),
    }
}
