use ::reqwest;

use crate::v2::objects::app::App;

use crate::v2::objects::error::Error;

pub enum GetAppResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(App),

    NotFound(Error),
}

pub struct GetAppPathParameters {
    pub cell: String,

    pub app: String,
}

pub struct GetAppQueryParameters {}

pub async fn get_app(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetAppPathParameters,
) -> Result<GetAppResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/apps/{}",
            server, path_parameters.cell, path_parameters.app
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<App>().await {
            Ok(app) => Ok(GetAppResponseType::Ok(app)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetAppResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetAppResponseType::UndefinedResponse(response)),
    }
}
