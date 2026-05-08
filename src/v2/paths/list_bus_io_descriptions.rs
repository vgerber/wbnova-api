use ::reqwest;

use crate::v2::objects::list_bus_io_descriptions_response::ListBusIoDescriptionsResponse;

use crate::v2::objects::error::Error;

pub enum ListBusIoDescriptionsResponseType {
    Ok(ListBusIoDescriptionsResponse),

    NotFound(Error),

    PreconditionFailed(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct ListBusIoDescriptionsPathParameters {
    pub cell: String,
}

pub struct ListBusIoDescriptionsQueryParameters {}

pub async fn list_bus_io_descriptions(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListBusIoDescriptionsPathParameters,
) -> Result<ListBusIoDescriptionsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/bus-ios/ios/description",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ListBusIoDescriptionsResponse>().await {
            Ok(list_bus_io_descriptions_response) => Ok(ListBusIoDescriptionsResponseType::Ok(
                list_bus_io_descriptions_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ListBusIoDescriptionsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        412 => match response.json::<Error>().await {
            Ok(error) => Ok(ListBusIoDescriptionsResponseType::PreconditionFailed(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListBusIoDescriptionsResponseType::UndefinedResponse(
            response,
        )),
    }
}
