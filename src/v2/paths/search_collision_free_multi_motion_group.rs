use ::reqwest;

use crate::v2::objects::multi_search_collision_free_422_response::MultiSearchCollisionFree422Response;

use crate::v2::objects::multi_search_collision_free_response::MultiSearchCollisionFreeResponse;

use crate::v2::objects::multi_search_collision_free_request::MultiSearchCollisionFreeRequest;

pub enum SearchCollisionFreeMultiMotionGroupResponseType {
    UndefinedResponse(reqwest::Response),

    UnprocessableEntity(MultiSearchCollisionFree422Response),

    Ok(MultiSearchCollisionFreeResponse),
}

pub struct SearchCollisionFreeMultiMotionGroupPathParameters {
    pub cell: String,
}

pub struct SearchCollisionFreeMultiMotionGroupQueryParameters {}

pub async fn search_collision_free_multi_motion_group(
    client: &reqwest::Client,

    server: &str,

    content: MultiSearchCollisionFreeRequest,

    path_parameters: SearchCollisionFreeMultiMotionGroupPathParameters,
) -> Result<SearchCollisionFreeMultiMotionGroupResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/experimental/cells/{}/trajectory-planning/search-collision-free-multi-motion-group",
            server, path_parameters.cell
        ))
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        422 => match response.json::<MultiSearchCollisionFree422Response>().await {
            Ok(multi_search_collision_free_422_response) => Ok(
                SearchCollisionFreeMultiMotionGroupResponseType::UnprocessableEntity(
                    multi_search_collision_free_422_response,
                ),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<MultiSearchCollisionFreeResponse>().await {
            Ok(multi_search_collision_free_response) => {
                Ok(SearchCollisionFreeMultiMotionGroupResponseType::Ok(
                    multi_search_collision_free_response,
                ))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(SearchCollisionFreeMultiMotionGroupResponseType::UndefinedResponse(response)),
    }
}
