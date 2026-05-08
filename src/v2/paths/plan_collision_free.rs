use ::reqwest;

use crate::v2::objects::plan_422_response::Plan422Response;

use crate::v2::objects::plan_collision_free_response::PlanCollisionFreeResponse;

use crate::v2::objects::plan_collision_free_request::PlanCollisionFreeRequest;

pub enum PlanCollisionFreeResponseType {
    UnprocessableEntity(Plan422Response),

    UndefinedResponse(reqwest::Response),

    Ok(PlanCollisionFreeResponse),
}

pub struct PlanCollisionFreePathParameters {
    pub cell: String,
}

pub struct PlanCollisionFreeQueryParameters {}

pub async fn plan_collision_free(
    client: &reqwest::Client,

    server: &str,

    content: PlanCollisionFreeRequest,

    path_parameters: PlanCollisionFreePathParameters,
) -> Result<PlanCollisionFreeResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/cells/{}/trajectory-planning/plan-collision-free",
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
        422 => match response.json::<Plan422Response>().await {
            Ok(plan_422_response) => Ok(PlanCollisionFreeResponseType::UnprocessableEntity(
                plan_422_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<PlanCollisionFreeResponse>().await {
            Ok(plan_collision_free_response) => Ok(PlanCollisionFreeResponseType::Ok(
                plan_collision_free_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(PlanCollisionFreeResponseType::UndefinedResponse(response)),
    }
}
