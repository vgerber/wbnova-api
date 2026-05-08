use serde::Serialize;

use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct NetworkState {
    pub latency_ms: Option<f64>,

    pub link_quality: Option<f64>,

    pub signal_strength: Option<i32>,

    pub connection_type: Option<String>,

    pub internet_connected: bool,

    pub bandwidth_mbps: Option<f64>,
}
