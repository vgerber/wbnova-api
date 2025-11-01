use ::reqwest;

pub enum GetDiagnosePackageResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct GetDiagnosePackagePathParameters {}

pub struct GetDiagnosePackageQueryParameters {}

pub async fn get_diagnose_package(
    client: &reqwest::Client,

    server: &str,
) -> Result<GetDiagnosePackageResponseType, reqwest::Error> {
    let response = match client
        .get(format!("{}/system/diagnosis-package/zip", server,))
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };

    match response.status().as_u16() {
        _ => Ok(GetDiagnosePackageResponseType::UndefinedResponse(response)),
    }
}
