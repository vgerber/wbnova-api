use std::net::TcpStream;

use tungstenite::connect;

use tungstenite::Error;

use tungstenite::WebSocket;

use tungstenite::protocol::CloseFrame;

use tungstenite::stream::MaybeTlsStream;

use tungstenite::http::Uri;

use tungstenite::client::IntoClientRequest;

use tungstenite::http::HeaderName;

use crate::v2::objects::stream_io_values_response::StreamIoValuesResponse;

pub struct StreamIoValuesPathParameters {
    pub controller: String,

    pub cell: String,
}

pub struct StreamIoValuesQueryParameters {
    pub ios: Option<Vec<String>>,
}

pub struct StreamIoValuesStream {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl StreamIoValuesStream {
    pub fn from(socket: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        StreamIoValuesStream { socket: socket }
    }

    pub fn close(&mut self, code: Option<CloseFrame>) -> Result<(), Error> {
        self.socket.close(code)
    }

    pub fn read(&mut self) -> Result<StreamIoValuesResponse, String> {
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

        match serde_json::from_value::<StreamIoValuesResponse>(response_object.clone()) {
            Ok(response_object) => Ok(response_object),
            Err(err) => return Err(err.to_string()),
        }
    }
}

pub async fn stream_io_values(
    host: &str,

    stream_io_values_path_parameters: StreamIoValuesPathParameters,

    stream_io_values_query_parameters: StreamIoValuesQueryParameters,

    additional_headers: &Option<Vec<(String, String)>>,
) -> Result<StreamIoValuesStream, tungstenite::Error> {
    // Required Query Parameters
    let mut query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = stream_io_values_query_parameters.ios {
        query_parameter.iter().for_each(|query_parameter_item| {
            query_parameters.push(("ios", query_parameter_item.to_string()))
        });
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
        "{}/cells/{}/controllers/{}/ios/stream{}",
        host,
        stream_io_values_path_parameters.cell,
        stream_io_values_path_parameters.controller,
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

    Ok(StreamIoValuesStream::from(socket))
}
