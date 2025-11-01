use ::reqwest;

use crate::v2::objects::error::Error;

use crate::v2::objects::configuration_archive_status::ConfigurationArchiveStatus;

pub enum GetConfigurationBackupStatusResponseType {
    UndefinedResponse(reqwest::Response),

    NotFound(Error),

    Ok(ConfigurationArchiveStatus),
}

pub struct GetConfigurationBackupStatusPathParameters {}

pub struct GetConfigurationBackupStatusQueryParameters {
    pub operation_id: String,
}

pub async fn get_configuration_backup_status(
    client: &reqwest::Client,

    server: &str,

    query_parameters: GetConfigurationBackupStatusQueryParameters,
) -> Result<GetConfigurationBackupStatusResponseType, reqwest::Error> {
    // Required Query Parameters
    let reqwest_query_parameters: Vec<(&str, String)> =
        vec![("operation_id", query_parameters.operation_id.to_string())];

    let response = match client
        .get(format!("{}/system/configuration/status", server,))
        .query(&reqwest_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetConfigurationBackupStatusResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },

        200 => match response.json::<ConfigurationArchiveStatus>().await {
            Ok(configuration_archive_status) => Ok(GetConfigurationBackupStatusResponseType::Ok(
                configuration_archive_status,
            )),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(GetConfigurationBackupStatusResponseType::UndefinedResponse(
            response,
        )),
    }
}
