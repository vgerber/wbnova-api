use ::reqwest;

pub enum ListAllObjectKeysResponseType {
    Ok(Vec<String>),

    UndefinedResponse(reqwest::Response),
}

pub struct ListAllObjectKeysPathParameters {
    pub cell: String,
}

pub struct ListAllObjectKeysQueryParameters {}

pub async fn list_all_object_keys(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListAllObjectKeysPathParameters,
) -> Result<ListAllObjectKeysResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/objects",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Vec<String>>().await {
            Ok(strings) => Ok(ListAllObjectKeysResponseType::Ok(strings)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListAllObjectKeysResponseType::UndefinedResponse(response)),
    }
}
