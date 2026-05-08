use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::network_interfaces::NetworkInterfaces;

pub enum GetNetworkInterfacesResponseType {
    NotFound(Error),

    Ok(NetworkInterfaces),

    UndefinedResponse(reqwest::Response),
}

pub struct GetNetworkInterfacesPathParameters {}

pub struct GetNetworkInterfacesQueryParameters {}

pub async fn get_network_interfaces(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetNetworkInterfacesResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/experimental/system/network/interfaces", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetNetworkInterfacesResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<NetworkInterfaces>().await {
            Ok(network_interfaces) => Ok(GetNetworkInterfacesResponseType::Ok(network_interfaces)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetNetworkInterfacesResponseType::UndefinedResponse(
            response,
        )),
    }
}
