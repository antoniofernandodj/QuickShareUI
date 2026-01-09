use crate::models::error::ApiError;
use crate::models::file::UploadResponse;
use crate::utils::constants::API_URL;
use gloo_net::http::Request;
use web_sys::{File, FormData};

pub struct ApiClient;

impl ApiClient {
    pub async fn upload_file(file: File) -> Result<UploadResponse, ApiError> {
        let form_data = FormData::new().map_err(|_| ApiError::FormDataCreation)?;

        form_data
            .append_with_blob("file", &file)
            .map_err(|_| ApiError::FormDataAppend)?;

        let url = format!("{}/upload", API_URL);
        
        let response = Request::post(&url)
            .body(form_data)
            .map_err(|e| ApiError::RequestBuild(format!("{:?}", e)))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("{:?}", e)))?;

        if response.ok() {
            response
                .json::<UploadResponse>()
                .await
                .map_err(|e| ApiError::JsonParse(format!("{:?}", e)))
        } else {
            Err(ApiError::ServerError(response.status()))
        }
    }
}