use ::reqwest;

pub enum CheckNovaVersionUpdateResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(String),
}

pub struct CheckNovaVersionUpdatePathParameters {}

pub struct CheckNovaVersionUpdateQueryParameters {
    pub channel: String,
}

pub async fn check_nova_version_update(
    client: &reqwest::Client,

    server: &str,

    query_parameters: CheckNovaVersionUpdateQueryParameters,
) -> Result<CheckNovaVersionUpdateResponseType, reqwest::Error> {
    // Required Query Parameters
    let reqwest_query_parameters: Vec<(&str, String)> =
        vec![("channel", query_parameters.channel.to_string())];

    let response = match client
        .get(format!("{}/system/update", server,))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.text().await {
            Ok(response_text) => Ok(CheckNovaVersionUpdateResponseType::Ok(response_text)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(CheckNovaVersionUpdateResponseType::UndefinedResponse(
            response,
        )),
    }
}
