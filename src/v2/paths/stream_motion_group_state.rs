use std::net::TcpStream;

use tungstenite::connect;

use tungstenite::Error;

use tungstenite::WebSocket;

use tungstenite::protocol::CloseFrame;

use tungstenite::stream::MaybeTlsStream;

use tungstenite::http::Uri;

use tungstenite::client::IntoClientRequest;

use tungstenite::http::HeaderName;

use crate::v2::objects::motion_group_state::MotionGroupState;

pub struct StreamMotionGroupStatePathParameters {
    pub cell: String,

    pub controller: String,

    pub motion_group: String,
}

pub struct StreamMotionGroupStateQueryParameters {
    pub response_coordinate_system: Option<String>,

    pub response_rate: Option<i32>,
}

pub struct StreamMotionGroupStateStream {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl StreamMotionGroupStateStream {
    pub fn from(socket: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        StreamMotionGroupStateStream { socket: socket }
    }

    pub fn close(&mut self, code: Option<CloseFrame>) -> Result<(), Error> {
        self.socket.close(code)
    }

    pub fn read(&mut self) -> Result<MotionGroupState, String> {
        let response = match self.socket.read() {
            Ok(response) => response,
            Err(err) => return Err(err.to_string()),
        };

        let response_text = match response.into_text() {
            Ok(response) => response,
            Err(err) => return Err(err.to_string()),
        };

        let result = match serde_json::from_str::<serde_json::Value>(&response_text) {
            Ok(response_json_object) => response_json_object,
            Err(err) => return Err(err.to_string()),
        };

        let response_object = match result.get("result") {
            Some(response_object) => response_object,
            None => return Err(format!("No result in message: {}", response_text)),
        };

        match serde_json::from_value::<MotionGroupState>(response_object.clone()) {
            Ok(response_object) => Ok(response_object),
            Err(err) => return Err(err.to_string()),
        }
    }
}

pub async fn stream_motion_group_state(
    host: &str,

    stream_motion_group_state_path_parameters: StreamMotionGroupStatePathParameters,

    stream_motion_group_state_query_parameters: StreamMotionGroupStateQueryParameters,

    additional_headers: &Option<Vec<(String, String)>>,
) -> Result<StreamMotionGroupStateStream, tungstenite::Error> {
    // Required Query Parameters
    let mut query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) =
        stream_motion_group_state_query_parameters.response_coordinate_system
    {
        query_parameters.push(("response_coordinate_system", query_parameter.to_string()));
    }

    if let Some(ref query_parameter) = stream_motion_group_state_query_parameters.response_rate {
        query_parameters.push(("response_rate", query_parameter.to_string()));
    }

    // Query and Path Parameters Assembly
    let mut query_string = query_parameters
        .iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<String>>()
        .join("&");
    if query_string.len() > 0 {
        query_string.insert_str(0, "?");
    }

    let url = format!(
        "{}/cells/{}/controllers/{}/motion-groups/{}/state-stream{}",
        host,
        stream_motion_group_state_path_parameters.cell,
        stream_motion_group_state_path_parameters.controller,
        stream_motion_group_state_path_parameters.motion_group,
        query_string
    );

    let uri: Uri = match url.parse() {
        Ok(uri) => uri,
        Err(err) => return Err(err.into()),
    };

    let mut request = match uri.into_client_request() {
        Ok(request) => request,
        Err(err) => return Err(err),
    };
    let request_headers = request.headers_mut();

    if let Some(additional_headers) = additional_headers {
        additional_headers.iter().for_each(|(key, value)| {
            let key = match HeaderName::try_from(key) {
                Ok(key) => key,
                Err(_) => return (),
            };
            let value = match value.parse() {
                Ok(value) => value,
                Err(_) => return (),
            };
            request_headers.append(key, value);
        });
    }

    // WebSocket connect
    let (socket, _) = match connect(request) {
        Ok(connection) => connection,
        Err(err) => return Err(err),
    };

    Ok(StreamMotionGroupStateStream::from(socket))
}
