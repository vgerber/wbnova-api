use ::reqwest;

use crate::v2::objects::project_joint_position_direction_constraint_response::ProjectJointPositionDirectionConstraintResponse;

use crate::v2::objects::project_joint_position_direction_constraint_422_response::ProjectJointPositionDirectionConstraint422Response;

use crate::v2::objects::project_joint_position_direction_constraint_request::ProjectJointPositionDirectionConstraintRequest;

pub enum ProjectJointPositionDirectionConstraintResponseType {
    Ok(ProjectJointPositionDirectionConstraintResponse),

    UnprocessableEntity(ProjectJointPositionDirectionConstraint422Response),

    UndefinedResponse(reqwest::Response),
}

pub struct ProjectJointPositionDirectionConstraintPathParameters {
    pub cell: String,
}

pub struct ProjectJointPositionDirectionConstraintQueryParameters {}

pub async fn project_joint_position_direction_constraint(
    client: &reqwest::Client,

    server: &str,

    content: ProjectJointPositionDirectionConstraintRequest,

    path_parameters: ProjectJointPositionDirectionConstraintPathParameters,
) -> Result<ProjectJointPositionDirectionConstraintResponseType, reqwest::Error> {
    let response = match client
        .post(format!(
            "{}/experimental/cells/{}/kinematic/project-joint-position-direction-constraint",
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
        200 => match response
            .json::<ProjectJointPositionDirectionConstraintResponse>()
            .await
        {
            Ok(project_joint_position_direction_constraint_response) => {
                Ok(ProjectJointPositionDirectionConstraintResponseType::Ok(
                    project_joint_position_direction_constraint_response,
                ))
            }
            Err(parsing_error) => Err(parsing_error),
        },

        422 => match response
            .json::<ProjectJointPositionDirectionConstraint422Response>()
            .await
        {
            Ok(project_joint_position_direction_constraint_422_response) => Ok(
                ProjectJointPositionDirectionConstraintResponseType::UnprocessableEntity(
                    project_joint_position_direction_constraint_422_response,
                ),
            ),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ProjectJointPositionDirectionConstraintResponseType::UndefinedResponse(response)),
    }
}
