use ::reqwest;

pub enum ListStoredCollidersKeysResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Vec<String>),
}

pub struct ListStoredCollidersKeysPathParameters {
    pub cell: String,
}

pub struct ListStoredCollidersKeysQueryParameters {}

pub async fn list_stored_colliders_keys(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListStoredCollidersKeysPathParameters,
) -> Result<ListStoredCollidersKeysResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/colliders-keys",
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
            Ok(collider_keys) => Ok(ListStoredCollidersKeysResponseType::Ok(collider_keys)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListStoredCollidersKeysResponseType::UndefinedResponse(
            response,
        )),
    }
}
