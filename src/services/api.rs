use crate::models::error::ApiError;
use crate::models::file::UploadResponse;
use crate::utils::constants::API_URL;
use gloo_net::http::Request;
use web_sys::{File, FormData};


use wasm_bindgen_futures::spawn_local;
use web_sys::{Blob, Url};
use wasm_bindgen::JsCast;
use yew::prelude::*;
use crate::models::file::StoredFile;
use crate::store::files_store::{FilesStoreAction};


pub struct ApiClient;

impl ApiClient {

    // Nova função para verificar se um arquivo existe
    pub async fn check_file_exists(file_id: &str) -> bool {
        let url = format!("{}/download/{}", API_URL, file_id);
        
        match Request::get(&url).send().await {
            Ok(response) => response.ok(),
            Err(_) => false,
        }
    }

    // Nova função para validar múltiplos arquivos
    pub async fn validate_files(files: Vec<StoredFile>) -> Vec<StoredFile> {
        let mut valid_files = Vec::new();
        
        for file in files {
            if Self::check_file_exists(&file.file_id).await {
                valid_files.push(file);
            }
        }
        
        valid_files
    }


    pub fn download_file(file: StoredFile, store: UseReducerHandle<crate::store::files_store::FilesStore>) {
        let file_id = file.file_id.clone();
        store.dispatch(FilesStoreAction::StartDownload(file_id.clone()));

        spawn_local(async move {
            let download_url = format!("{}/download/{}", API_URL, file.file_id);
            if let Ok(response) = Request::get(&download_url).send().await {
                if let Ok(bytes) = response.binary().await {
                    let u8_array = js_sys::Uint8Array::from(&bytes[..]);
                    let blob = Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&u8_array)).unwrap();
                    let download_url = Url::create_object_url_with_blob(&blob).unwrap();

                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let a = document.create_element("a").unwrap();
                    a.set_attribute("href", &download_url).unwrap();
                    a.set_attribute("download", &file.filename).unwrap();
                    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
                    Url::revoke_object_url(&download_url).unwrap();
                }
            }

            store.dispatch(FilesStoreAction::EndDownload(file_id));
        });
    }

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