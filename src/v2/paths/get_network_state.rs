use ::reqwest;

use crate::v2::objects::network_state::NetworkState;

use crate::v2::objects::error::Error;

pub enum GetNetworkStateResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    Ok(NetworkState),
}

pub struct GetNetworkStatePathParameters {}

pub struct GetNetworkStateQueryParameters {}

pub async fn get_network_state(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetNetworkStateResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/experimental/system/network/status", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<NetworkState>().await {
            Ok(network_state) => Ok(GetNetworkStateResponseType::Ok(network_state)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetNetworkStateResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetNetworkStateResponseType::UndefinedResponse(response)),
    }
}
