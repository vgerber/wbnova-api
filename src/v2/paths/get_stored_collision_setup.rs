use ::reqwest;

use crate::v2::objects::collision_setup::CollisionSetup;

pub enum GetStoredCollisionSetupResponseType {
    Ok(CollisionSetup),

    UndefinedResponse(reqwest::Response),
}

pub struct GetStoredCollisionSetupPathParameters {
    pub setup: String,

    pub cell: String,
}

pub struct GetStoredCollisionSetupQueryParameters {}

pub async fn get_stored_collision_setup(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetStoredCollisionSetupPathParameters,
) -> Result<GetStoredCollisionSetupResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/setups/{}",
            server, path_parameters.cell, path_parameters.setup
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<CollisionSetup>().await {
            Ok(collision_setup) => Ok(GetStoredCollisionSetupResponseType::Ok(collision_setup)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetStoredCollisionSetupResponseType::UndefinedResponse(
            response,
        )),
    }
}
