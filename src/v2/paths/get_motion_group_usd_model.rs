use ::reqwest;

pub enum GetMotionGroupUsdModelResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionGroupUsdModelPathParameters {
    pub motion_group_model: String,
}

pub struct GetMotionGroupUsdModelQueryParameters {}

pub async fn get_motion_group_usd_model(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetMotionGroupUsdModelPathParameters,
) -> Result<GetMotionGroupUsdModelResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/motion-group-models/{}/usd",
            server, path_parameters.motion_group_model
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(GetMotionGroupUsdModelResponseType::UndefinedResponse(
            response,
        )),
    }
}
