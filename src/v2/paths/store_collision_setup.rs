use ::reqwest;

use crate::v2::objects::collision_setup::CollisionSetup;

pub enum StoreCollisionSetupResponseType {
    Ok(CollisionSetup),

    UndefinedResponse(reqwest::Response),
}

pub struct StoreCollisionSetupPathParameters {
    pub cell: String,

    pub setup: String,
}

pub struct StoreCollisionSetupQueryParameters {}

pub async fn store_collision_setup(
    client: &reqwest::Client,

    server: &str,

    content: CollisionSetup,

    path_parameters: StoreCollisionSetupPathParameters,
) -> Result<StoreCollisionSetupResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/store/collision/setups/{}",
            server, path_parameters.cell, path_parameters.setup
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<CollisionSetup>().await {
            Ok(collision_setup) => Ok(StoreCollisionSetupResponseType::Ok(collision_setup)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(StoreCollisionSetupResponseType::UndefinedResponse(response)),
    }
}
