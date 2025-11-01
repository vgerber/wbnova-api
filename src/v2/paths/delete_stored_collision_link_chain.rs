use ::reqwest;

pub enum DeleteStoredCollisionLinkChainResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteStoredCollisionLinkChainPathParameters {
    pub link_chain: String,

    pub cell: String,
}

pub struct DeleteStoredCollisionLinkChainQueryParameters {}

pub async fn delete_stored_collision_link_chain(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteStoredCollisionLinkChainPathParameters,
) -> Result<DeleteStoredCollisionLinkChainResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
            "{}/cells/{}/store/collision/link-chains/{}",
            server, path_parameters.cell, path_parameters.link_chain
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(DeleteStoredCollisionLinkChainResponseType::UndefinedResponse(response)),
    }
}
