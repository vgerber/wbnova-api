use std::{env, str::FromStr};

use reqwest::Url;
use wbnova_api::v2::paths::{
    get_current_robot_controller_state::{
        get_current_robot_controller_state, GetCurrentRobotControllerStatePathParameters,
        GetCurrentRobotControllerStateResponseType,
    },
    get_motion_group_description::{
        get_motion_group_description, GetMotionGroupDescriptionPathParameters,
        GetMotionGroupDescriptionResponseType,
    },
    list_cells::list_cells,
    list_robot_controllers::{list_robot_controllers, ListRobotControllersPathParameters},
};

#[tokio::test]
async fn test_basic_motion_data() {
    dotenv::from_filename(".env.test").ok();
    let instance_uri = Url::from_str(&env::var("INSTANCE_URL").unwrap()).unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", env::var("INSTANCE_ACCESS_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );
    let server = format!("https://{}/api/v2", instance_uri.host().unwrap());
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let cells = match list_cells(&client, &server).await.unwrap() {
        wbnova_api::v2::paths::list_cells::ListCellsResponseType::Ok(name_list) => name_list,
        _ => panic!("Failed to list cells"),
    };
    println!("Cells: {:#?}", cells);

    let cell = &cells[0];

    let controllers = match list_robot_controllers(
        &client,
        &server,
        ListRobotControllersPathParameters { cell: cell.clone() },
    )
    .await
    .unwrap()
    {
        wbnova_api::v2::paths::list_robot_controllers::ListRobotControllersResponseType::Ok(
            name_list,
        ) => name_list,
        _ => panic!("Failed to list controllers"),
    };
    println!("Controllers: {:#?}", controllers);

    for controller in controllers {
        let controller_state = match get_current_robot_controller_state(
            &client,
            &server,
            GetCurrentRobotControllerStatePathParameters {
                controller: controller.clone(),
                cell: cell.clone(),
            },
        )
        .await
        .unwrap()
        {
            GetCurrentRobotControllerStateResponseType::Ok(state) => state,
            _ => panic!("Failed to get controller state"),
        };

        println!(
            "Controller State for {}: {:#?}",
            controller, controller_state
        );

        for motion_group_state in controller_state.motion_groups {
            let motion_group_description = match get_motion_group_description(
                &client,
                &server,
                GetMotionGroupDescriptionPathParameters {
                    controller: controller.clone(),
                    cell: cell.clone(),
                    motion_group: motion_group_state.motion_group.clone(),
                },
            )
            .await
            .unwrap()
            {
                GetMotionGroupDescriptionResponseType::Ok(description) => description,
                GetMotionGroupDescriptionResponseType::BadRequest(err) => {
                    panic!(
                        "Bad request when getting motion group description: {:?}",
                        err
                    )
                }
                GetMotionGroupDescriptionResponseType::NotFound(err) => {
                    panic!("Motion group description not found: {:?}", err)
                }
                GetMotionGroupDescriptionResponseType::UndefinedResponse(response) => {
                    panic!("Failed to get motion group description: {:?}", response)
                }
            };

            println!(
                "Motion Group Description for {}: {:#?}",
                motion_group_state.motion_group, motion_group_description
            );
        }
    }
}
