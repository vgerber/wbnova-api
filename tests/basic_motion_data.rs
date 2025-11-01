use std::{env, str::FromStr};

use reqwest::Url;
use wbnova_api::v2::paths::stream_motion_group_state::{
    stream_motion_group_state, StreamMotionGroupStatePathParameters,
    StreamMotionGroupStateQueryParameters,
};

#[tokio::test]
async fn test_basic_motion_data() {
    dotenv::from_filename(".env.test").ok();
    let instance_uri = Url::from_str(&env::var("INSTANCE_URL").unwrap()).unwrap();

    let mut stream = match stream_motion_group_state(
        &format!("wss://{}/api/v2", instance_uri.host().unwrap()),
        &StreamMotionGroupStatePathParameters {
            motion_group: env::var("MOTION_GROUP").unwrap(),
            cell: env::var("CELL").unwrap(),
            controller: env::var("CONTROLLER").unwrap(),
        },
        &StreamMotionGroupStateQueryParameters {
            response_coordinate_system: None,
            response_rate: None,
        },
        &Some(vec![(
            "Authorization".to_owned(),
            format!("Bearer {}", env::var("INSTANCE_ACCESS_TOKEN").unwrap()),
        )]),
    )
    .await
    {
        Ok(s) => s,
        Err(e) => panic!("Failed to create stream: {:?}", e),
    };

    let mut message_counter = 0;

    while message_counter < 5 {
        match stream.read() {
            Ok(message) => {
                println!("Received message: {:?}", message);
                message_counter += 1;
            }
            Err(e) => {
                panic!("Error receiving message: {:?}", e);
            }
        }
    }
}
