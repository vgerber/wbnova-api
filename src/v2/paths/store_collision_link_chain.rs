use ::reqwest;

use crate::v2::objects::link_chain::LinkChain;

pub enum StoreCollisionLinkChainResponseType {
    Ok(LinkChain),

    UndefinedResponse(reqwest::Response),
}

pub struct StoreCollisionLinkChainPathParameters {
    pub cell: String,

    pub link_chain: String,
}

pub struct StoreCollisionLinkChainQueryParameters {}

pub async fn store_collision_link_chain(
    client: &reqwest::Client,

    server: &str,

    content: LinkChain,

    path_parameters: StoreCollisionLinkChainPathParameters,
) -> Result<StoreCollisionLinkChainResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/store/collision/link-chains/{}",
            server, path_parameters.cell, path_parameters.link_chain
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<LinkChain>().await {
            Ok(link_chain) => Ok(StoreCollisionLinkChainResponseType::Ok(link_chain)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(StoreCollisionLinkChainResponseType::UndefinedResponse(
            response,
        )),
    }
}
