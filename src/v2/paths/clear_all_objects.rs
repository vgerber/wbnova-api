use ::reqwest;

pub enum ClearAllObjectsResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct ClearAllObjectsPathParameters {
    pub cell: String,
}

pub struct ClearAllObjectsQueryParameters {}

pub async fn clear_all_objects(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ClearAllObjectsPathParameters,
) -> Result<ClearAllObjectsResponseType, reqwest::Error> {
    let response = match client
        .delete(format!(
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
        _ => Ok(ClearAllObjectsResponseType::UndefinedResponse(response)),
    }
}
