use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::bus_i_os_state::BusIOsState;

pub enum GetBusIoStateResponseType {
    Ok(BusIOsState),

    NotFound(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct GetBusIoStatePathParameters {
    pub cell: String,
}

pub struct GetBusIoStateQueryParameters {}

pub async fn get_bus_io_state(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetBusIoStatePathParameters,
) -> Result<GetBusIoStateResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/bus-ios/state",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetBusIoStateResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<BusIOsState>().await {
            Ok(bus_i_os_state) => Ok(GetBusIoStateResponseType::Ok(bus_i_os_state)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetBusIoStateResponseType::UndefinedResponse(response)),
    }
}
