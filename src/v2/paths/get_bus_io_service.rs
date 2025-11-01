use ::reqwest;

use crate::v2::objects::bus_io_type::BusIoType;

use crate::v2::objects::error::Error;

pub enum GetBusIoServiceResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    Ok(BusIoType),
}

pub struct GetBusIoServicePathParameters {
    pub cell: String,
}

pub struct GetBusIoServiceQueryParameters {}

pub async fn get_bus_io_service(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetBusIoServicePathParameters,
) -> Result<GetBusIoServiceResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/cells/{}/bus-ios", server, path_parameters.cell))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<BusIoType>().await {
            Ok(bus_io_type) => Ok(GetBusIoServiceResponseType::Ok(bus_io_type)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetBusIoServiceResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetBusIoServiceResponseType::UndefinedResponse(response)),
    }
}
