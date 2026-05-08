use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::arp_scan_response::ArpScanResponse;

pub enum GetArpScanResponseType {
    NotFound(Error),

    BadRequest(Error),

    Ok(ArpScanResponse),

    UndefinedResponse(reqwest::Response),
}

pub struct GetArpScanPathParameters {}

pub struct GetArpScanQueryParameters {
    pub interface: Option<String>,

    pub timeout: Option<i32>,

    pub cidr: Option<String>,
}

pub async fn get_arp_scan(
    client: &reqwest::Client,

    server: &str,

    query_parameters: GetArpScanQueryParameters,
) -> Result<GetArpScanResponseType, reqwest::Error> {
    // Required Query Parameters
    let mut reqwest_query_parameters: Vec<(&str, String)> = vec![];

    // Optional Query Parameters

    if let Some(ref query_parameter) = query_parameters.interface {
        reqwest_query_parameters.push(("interface", query_parameter.to_string()));
    }

    if let Some(ref query_parameter) = query_parameters.timeout {
        reqwest_query_parameters.push(("timeout", query_parameter.to_string()));
    }

    if let Some(ref query_parameter) = query_parameters.cidr {
        reqwest_query_parameters.push(("cidr", query_parameter.to_string()));
    }

    let response = match client
        .get(format!("{}/experimental/system/network/arp-scan", server,))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(GetArpScanResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetArpScanResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ArpScanResponse>().await {
            Ok(arp_scan_response) => Ok(GetArpScanResponseType::Ok(arp_scan_response)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetArpScanResponseType::UndefinedResponse(response)),
    }
}
