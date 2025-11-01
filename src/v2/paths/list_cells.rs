use ::reqwest;

use crate::v2::objects::name_list::NameList;

pub enum ListCellsResponseType {
    Ok(NameList),

    UndefinedResponse(reqwest::Response),
}

pub struct ListCellsPathParameters {}

pub struct ListCellsQueryParameters {}

pub async fn list_cells(
    client: &reqwest::Client,

    server: &str,
) -> Result<ListCellsResponseType, reqwest::Error> {
    let response = match client.get(format!("{}/cells", server,)).send().await {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        200 => match response.json::<NameList>().await {
            Ok(name_list) => Ok(ListCellsResponseType::Ok(name_list)),
            Err(parsing_error) => Err(parsing_error),
        },

        _ => Ok(ListCellsResponseType::UndefinedResponse(response)),
    }
}
