use ::reqwest;

pub enum GetMotionGroupGlbModelResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionGroupGlbModelPathParameters {
    pub motion_group_model: String,
}

pub struct GetMotionGroupGlbModelQueryParameters {}

pub async fn get_motion_group_glb_model(
    client: &reqwest::Client,

    server: &str,

    path_parameters: GetMotionGroupGlbModelPathParameters,
) -> Result<GetMotionGroupGlbModelResponseType, reqwest::Error> {
    let response = match client
        .get(format!(
            "{}/motion-group-models/{}/glb",
            server, path_parameters.motion_group_model
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(GetMotionGroupGlbModelResponseType::UndefinedResponse(
            response,
        )),
    }
}
