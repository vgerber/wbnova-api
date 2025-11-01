use ::reqwest;

pub enum DeleteStoredColliderResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteStoredColliderPathParameters {
    pub cell: String,

    pub collider: String,
}

pub struct DeleteStoredColliderQueryParameters {}

pub async fn delete_stored_collider(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteStoredColliderPathParameters,
) -> Result<DeleteStoredColliderResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/store/collision/colliders/{}",
            server, path_parameters.cell, path_parameters.collider
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(DeleteStoredColliderResponseType::UndefinedResponse(
            response,
        )),
    }
}
