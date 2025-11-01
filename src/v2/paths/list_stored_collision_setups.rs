use ::reqwest;

use crate::v2::objects::list_stored_collision_setups_ok_json::ListStoredCollisionSetupsOkJson;

pub enum ListStoredCollisionSetupsResponseType {
    Ok(ListStoredCollisionSetupsOkJson),

    UndefinedResponse(reqwest::Response),
}

pub struct ListStoredCollisionSetupsPathParameters {
    pub cell: String,
}

pub struct ListStoredCollisionSetupsQueryParameters {}

pub async fn list_stored_collision_setups(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListStoredCollisionSetupsPathParameters,
) -> Result<ListStoredCollisionSetupsResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/setups",
            server, path_parameters.cell
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<ListStoredCollisionSetupsOkJson>().await {
            Ok(list_stored_collision_setups_ok_json) => Ok(
                ListStoredCollisionSetupsResponseType::Ok(list_stored_collision_setups_ok_json),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListStoredCollisionSetupsResponseType::UndefinedResponse(
            response,
        )),
    }
}
