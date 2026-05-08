use ::reqwest;

pub enum CheckCellVersionUpdateResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(String),
}

pub struct CheckCellVersionUpdatePathParameters {
    pub cell: String,
}

pub struct CheckCellVersionUpdateQueryParameters {
    pub channel: String,
}

pub async fn check_cell_version_update(
    client: &reqwest::Client,

    server: &str,

    path_parameters: CheckCellVersionUpdatePathParameters,

    query_parameters: CheckCellVersionUpdateQueryParameters,
) -> Result<CheckCellVersionUpdateResponseType, reqwest::Error> {
    // Required Query Parameters
    let reqwest_query_parameters: Vec<(&str, String)> =
        vec![("channel", query_parameters.channel.to_string())];

    let response = match client
        .get(format!("{}/cells/{}/update", server, path_parameters.cell))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.text().await {
            Ok(response_text) => Ok(CheckCellVersionUpdateResponseType::Ok(response_text)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(CheckCellVersionUpdateResponseType::UndefinedResponse(
            response,
        )),
    }
}
