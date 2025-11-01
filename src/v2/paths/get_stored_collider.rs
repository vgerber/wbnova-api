use ::reqwest;

use crate::v2::objects::collider::Collider;

pub enum GetStoredColliderResponseType {
    Ok(Collider),

    UndefinedResponse(reqwest::Response),
}

pub struct GetStoredColliderPathParameters {
    pub cell: String,

    pub collider: String,
}

pub struct GetStoredColliderQueryParameters {}

pub async fn get_stored_collider(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetStoredColliderPathParameters,
) -> Result<GetStoredColliderResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/cells/{}/store/collision/colliders/{}",
            server, path_parameters.cell, path_parameters.collider
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Collider>().await {
            Ok(collider) => Ok(GetStoredColliderResponseType::Ok(collider)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetStoredColliderResponseType::UndefinedResponse(response)),
    }
}
