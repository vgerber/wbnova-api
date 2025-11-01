use ::reqwest;

pub enum ListCollisionLinkChainsKeysResponseType {
    Ok(Vec<String>),

    UndefinedResponse(reqwest::Response),
}

pub struct ListCollisionLinkChainsKeysPathParameters {
    pub cell: String,
}

pub struct ListCollisionLinkChainsKeysQueryParameters {}

pub async fn list_collision_link_chains_keys(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListCollisionLinkChainsKeysPathParameters,
) -> Result<ListCollisionLinkChainsKeysResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/link-chains-keys",
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
            Ok(link_chain_keys) => Ok(ListCollisionLinkChainsKeysResponseType::Ok(link_chain_keys)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListCollisionLinkChainsKeysResponseType::UndefinedResponse(
            response,
        )),
    }
}
