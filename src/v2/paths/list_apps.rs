use ::reqwest;

use crate::v2::objects::name_list::NameList;

pub enum ListAppsResponseType {
    UndefinedResponse(reqwest::Response),

    Ok(NameList),
}

pub struct ListAppsPathParameters {
    pub cell: String,
}

pub struct ListAppsQueryParameters {}

pub async fn list_apps(
    client: &reqwest::Client,

    server: &str,

    path_parameters: ListAppsPathParameters,
) -> Result<ListAppsResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/cells/{}/apps", server, path_parameters.cell))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<NameList>().await {
            Ok(name_list) => Ok(ListAppsResponseType::Ok(name_list)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListAppsResponseType::UndefinedResponse(response)),
    }
}
