use ::reqwest;

use crate::v2::objects::merge_trajectories_response::MergeTrajectoriesResponse;

use crate::v2::objects::merge_trajectories_422_response::MergeTrajectories422Response;

use crate::v2::objects::merge_trajectories_request::MergeTrajectoriesRequest;

pub enum MergeTrajectoriesResponseType {
    UnprocessableEntity(MergeTrajectories422Response),

    UndefinedResponse(reqwest::Response),

    Ok(MergeTrajectoriesResponse),
}

pub struct MergeTrajectoriesPathParameters {
    pub cell: String,
}

pub struct MergeTrajectoriesQueryParameters {}

pub async fn merge_trajectories(
    client: &reqwest::Client,

    server: &str,

    content: MergeTrajectoriesRequest,

    path_parameters: MergeTrajectoriesPathParameters,
) -> Result<MergeTrajectoriesResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/experimental/cells/{}/trajectory-planning/merge-trajectories",
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
        200 => match response.json::<MergeTrajectoriesResponse>().await {
            Ok(merge_trajectories_response) => Ok(MergeTrajectoriesResponseType::Ok(
                merge_trajectories_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        422 => match response.json::<MergeTrajectories422Response>().await {
            Ok(merge_trajectories_422_response) => Ok(
                MergeTrajectoriesResponseType::UnprocessableEntity(merge_trajectories_422_response),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(MergeTrajectoriesResponseType::UndefinedResponse(response)),
    }
}
