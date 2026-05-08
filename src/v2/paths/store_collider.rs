use ::reqwest;

use crate::v2::objects::collider::Collider;

pub enum StoreColliderResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(Collider),
}

pub struct StoreColliderPathParameters {
    pub collider: String,

    pub cell: String,
}

pub struct StoreColliderQueryParameters {}

pub async fn store_collider(
    client: &reqwest::Client,

    server: &str,

    content: Collider,

    path_parameters: StoreColliderPathParameters,
) -> Result<StoreColliderResponseType, reqwest::Error> {
    let response = match client
        .put(format!(
            "{}/cells/{}/store/collision/colliders/{}",
            server, path_parameters.cell, path_parameters.collider
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Collider>().await {
            Ok(collider) => Ok(StoreColliderResponseType::Ok(collider)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(StoreColliderResponseType::UndefinedResponse(response)),
    }
}
