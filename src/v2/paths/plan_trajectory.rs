use ::reqwest;

use crate::v2::objects::plan_422_response::Plan422Response;

use crate::v2::objects::plan_trajectory_response::PlanTrajectoryResponse;

use crate::v2::objects::plan_trajectory_request::PlanTrajectoryRequest;

pub enum PlanTrajectoryResponseType {
    Ok(PlanTrajectoryResponse),

    UnprocessableEntity(Plan422Response),

    UndefinedResponse(reqwest::Response),
}

pub struct PlanTrajectoryPathParameters {
    pub cell: String,
}

pub struct PlanTrajectoryQueryParameters {}

pub async fn plan_trajectory(
    client: &reqwest::Client,

    server: &str,

    content: PlanTrajectoryRequest,

    path_parameters: PlanTrajectoryPathParameters,
) -> Result<PlanTrajectoryResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/cells/{}/trajectory-planning/plan-trajectory",
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
            Ok(plan_422_response) => Ok(PlanTrajectoryResponseType::UnprocessableEntity(
                plan_422_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<PlanTrajectoryResponse>().await {
            Ok(plan_trajectory_response) => {
                Ok(PlanTrajectoryResponseType::Ok(plan_trajectory_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(PlanTrajectoryResponseType::UndefinedResponse(response)),
    }
}
