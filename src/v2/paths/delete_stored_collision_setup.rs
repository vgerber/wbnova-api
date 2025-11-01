use ::reqwest;

pub enum DeleteStoredCollisionSetupResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteStoredCollisionSetupPathParameters {
    pub setup: String,

    pub cell: String,
}

pub struct DeleteStoredCollisionSetupQueryParameters {}

pub async fn delete_stored_collision_setup(
    client: &reqwest::Client,

    server: &str,

    path_parameters: DeleteStoredCollisionSetupPathParameters,
) -> Result<DeleteStoredCollisionSetupResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
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
        _ => Ok(DeleteStoredCollisionSetupResponseType::UndefinedResponse(
            response,
        )),
    }
}
