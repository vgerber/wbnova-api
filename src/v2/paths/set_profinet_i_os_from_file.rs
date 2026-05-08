use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::profinet_input_output_config::ProfinetInputOutputConfig;

pub enum SetProfinetIOsFromFileResponseType {
    NotFound(Error),

    BadRequest(Error),

    PreconditionFailed(Error),

    UndefinedResponse(reqwest::Response),
}

pub struct SetProfinetIOsFromFilePathParameters {
    pub cell: String,
}

pub struct SetProfinetIOsFromFileQueryParameters {}

pub async fn set_profinet_i_os_from_file(
    client: &reqwest::Client,

    server: &str,

    content: ProfinetInputOutputConfig,

    path_parameters: SetProfinetIOsFromFilePathParameters,
) -> Result<SetProfinetIOsFromFileResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/bus-ios/profinet/iofile",
            server, path_parameters.cell
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(SetProfinetIOsFromFileResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(SetProfinetIOsFromFileResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        412 => match response.json::<Error>().await {
            Ok(error) => Ok(SetProfinetIOsFromFileResponseType::PreconditionFailed(
                error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SetProfinetIOsFromFileResponseType::UndefinedResponse(
            response,
        )),
    }
}
