use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::cycle_time::CycleTime;

pub enum GetCycleTimeResponseType {
    UndefinedResponse(reqwest::Response),

    BadRequest(Error),

    Ok(CycleTime),
}

pub struct GetCycleTimePathParameters {
    pub cell: String,

    pub controller: String,
}

pub struct GetCycleTimeQueryParameters {}

pub async fn get_cycle_time(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetCycleTimePathParameters,
) -> Result<GetCycleTimeResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/virtual-controllers/{}/cycle-time",
            server, path_parameters.cell, path_parameters.controller
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCycleTimeResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<CycleTime>().await {
            Ok(cycle_time) => Ok(GetCycleTimeResponseType::Ok(cycle_time)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetCycleTimeResponseType::UndefinedResponse(response)),
    }
}
