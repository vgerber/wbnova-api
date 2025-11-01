use ::reqwest;

use crate::v2::objects::profinet_i_os::ProfinetIOs;

use crate::v2::objects::error::Error;

pub enum ListProfinetIOsResponseType {
    NotFound(Error),

    UndefinedResponse(reqwest::Response),

    Ok(ProfinetIOs),
}

pub struct ListProfinetIOsPathParameters {
    pub cell: String,
}

pub struct ListProfinetIOsQueryParameters {}

pub async fn list_profinet_i_os(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListProfinetIOsPathParameters,
) -> Result<ListProfinetIOsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/bus-ios/profinet/ios",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ProfinetIOs>().await {
            Ok(profinet_i_os) => Ok(ListProfinetIOsResponseType::Ok(profinet_i_os)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ListProfinetIOsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListProfinetIOsResponseType::UndefinedResponse(response)),
    }
}
