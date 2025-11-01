use ::reqwest;

pub enum ListStoredCollisionSetupsKeysResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Vec<String>),
}

pub struct ListStoredCollisionSetupsKeysPathParameters {
    pub cell: String,
}

pub struct ListStoredCollisionSetupsKeysQueryParameters {}

pub async fn list_stored_collision_setups_keys(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListStoredCollisionSetupsKeysPathParameters,
) -> Result<ListStoredCollisionSetupsKeysResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/setups-keys",
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
            Ok(setup_keys) => Ok(ListStoredCollisionSetupsKeysResponseType::Ok(setup_keys)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListStoredCollisionSetupsKeysResponseType::UndefinedResponse(response)),
    }
}
