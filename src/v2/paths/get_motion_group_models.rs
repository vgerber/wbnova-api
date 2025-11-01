use ::reqwest;

pub enum GetMotionGroupModelsResponseType {
    Ok(Vec<String>),

    UndefinedResponse(reqwest::Response),
}

pub struct GetMotionGroupModelsPathParameters {}

pub struct GetMotionGroupModelsQueryParameters {}

pub async fn get_motion_group_models(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetMotionGroupModelsResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/motion-group-models", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<Vec<String>>().await {
            Ok(models) => Ok(GetMotionGroupModelsResponseType::Ok(models)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetMotionGroupModelsResponseType::UndefinedResponse(
            response,
        )),
    }
}
