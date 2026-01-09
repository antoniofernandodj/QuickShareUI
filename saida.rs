////// Arquivo: ./src/hooks/use_file_upload.rs
use yew::prelude::*;
use web_sys::File;
use wasm_bindgen_futures::spawn_local;
use crate::models::file::StoredFile;
use crate::services::api::ApiClient;
use crate::store::files_store::{FilesStoreAction, FilesStoreContext};

#[hook]
pub fn use_file_upload() -> Callback<File> {
    let store = use_context::<FilesStoreContext>().expect("FilesStoreContext not found");

    Callback::from(move |file: File| {
        let store = store.clone();
        let filename = file.name();

        // Marca loading global para upload
        store.dispatch(FilesStoreAction::SetLoading(true));
        store.dispatch(FilesStoreAction::SetError(None));

        spawn_local(async move {
            match ApiClient::upload_file(file).await {
                Ok(response) => {
                    let stored_file = StoredFile {
                        file_id: response.file_id,
                        filename,
                        download_url: response.download_url,
                        expires_at: response.expires_at,
                        uploaded_at: js_sys::Date::new_0().to_iso_string().into(),
                    };
                    store.dispatch(FilesStoreAction::AddFile(stored_file));
                }
                Err(e) => {
                    store.dispatch(FilesStoreAction::SetError(Some(e.to_string())));
                }
            }

            // Remove loading quando upload termina
            store.dispatch(FilesStoreAction::SetLoading(false));
        });
    })
}


////// Arquivo: ./src/hooks/mod.rs
pub mod use_clipboard;
pub mod use_file_upload;

////// Arquivo: ./src/hooks/use_clipboard.rs
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[hook]
pub fn use_clipboard() -> Callback<String> {
    Callback::from(|text: String| {
        spawn_local(async move {
            if let Some(window) = window() {
                let navigator = window.navigator().clipboard();
                let _ = wasm_bindgen_futures::JsFuture::from(
                    navigator.write_text(&text)
                ).await;
            }
        });
    })
}

////// Arquivo: ./src/main.rs
mod app;
mod components;
mod hooks;
mod models;
mod services;
mod store;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}

////// Arquivo: ./src/services/api.rs
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

////// Arquivo: ./src/services/mod.rs
pub mod api;
pub mod storage;


////// Arquivo: ./src/services/storage.rs
use crate::models::file::StoredFile;
use crate::utils::constants::STORAGE_KEY;
use web_sys::window;

pub struct StorageService;

impl StorageService {
    pub fn load_files() -> Vec<StoredFile> {
        let window = match window() {
            Some(w) => w,
            None => return Vec::new(),
        };

        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return Vec::new(),
        };

        storage
            .get_item(STORAGE_KEY)
            .ok()
            .flatten()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    }

    pub fn save_files(files: &[StoredFile]) {
        let window = match window() {
            Some(w) => w,
            None => return,
        };

        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        if let Ok(json) = serde_json::to_string(files) {
            let _ = storage.set_item(STORAGE_KEY, &json);
        }
    }

    pub fn clear_files() {
        let window = match window() {
            Some(w) => w,
            None => return,
        };

        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return,
        };

        let _ = storage.remove_item(STORAGE_KEY);
    }
}

////// Arquivo: ./src/utils/formatters.rs
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

pub fn format_expires(expires_at: &str) -> String {
    expires_at
        .split('T')
        .next()
        .unwrap_or(expires_at)
        .to_string()
}

#[allow(unused)]
pub fn format_datetime(datetime: &str) -> String {
    if let Some(date_part) = datetime.split('T').next() {
        if let Some(time_part) = datetime.split('T').nth(1) {
            let time = time_part.split('.').next().unwrap_or("");
            return format!("{} às {}", date_part, time);
        }
        return date_part.to_string();
    }
    datetime.to_string()
}


////// Arquivo: ./src/utils/mod.rs
pub mod constants;
pub mod formatters;

////// Arquivo: ./src/utils/constants.rs
// pub const API_URL: &str = "https://quickshare-latest.onrender.com";
pub const STORAGE_KEY: &str = "quickshare_uploaded_files";

#[cfg(feature = "dev")]
pub const API_URL: &str = "http://0.0.0.0:7777";

#[cfg(feature = "release")]
pub const API_URL: &str = "https://quickshareui.pages.dev/";


////// Arquivo: ./src/components/file_list.rs
use yew::prelude::*;
use crate::components::file_card::FileCard;
use crate::store::files_store::FilesStoreContext;

#[function_component(FileList)]
pub fn file_list() -> Html {
    let store = use_context::<FilesStoreContext>().expect("FilesStoreContext not found");

    if store.files.is_empty() {
        return html! {};
    }

    html! {
        <div class="files-list">
            <h2>{"Arquivos Enviados"}</h2>
            {
                for store.files.iter().map(|file| {
                    html! { <FileCard file={file.clone()} /> }
                })
            }
        </div>
    }
}

////// Arquivo: ./src/components/file_card.rs
use yew::prelude::*;
use crate::hooks::use_clipboard::use_clipboard;
use crate::models::file::StoredFile;
use crate::services::api::ApiClient;
use crate::store::files_store::FilesStoreContext;
use crate::utils::constants::API_URL;
use crate::utils::formatters::format_expires;

#[derive(Properties, PartialEq)]
pub struct FileCardProps {
    pub file: StoredFile,
}

#[function_component(FileCard)]
pub fn file_card(props: &FileCardProps) -> Html {
    let store = use_context::<FilesStoreContext>().expect("FilesStoreContext not found");
    let copy_to_clipboard = use_clipboard();
    let download_url = format!("{}/download/{}", API_URL, props.file.file_id);
    let expires = format_expires(&props.file.expires_at);

    let on_copy = {
        let url = download_url.clone();
        let copy = copy_to_clipboard.clone();
        Callback::from(move |_| {
            copy.emit(url.clone());
        })
    };

    let downloading = store.downloading_files.contains(&props.file.file_id);

    let on_download = {
        let store = store.clone();
        let file = props.file.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            ApiClient::download_file(file.clone(), store.clone());
        })
    };

    html! {
        <div class="file-card">
            <div class="file-info">
                <div class="file-name-large">
                    {"📄 "}{&props.file.filename}
                </div>
                <div class="file-meta">
                    <span>{"🆔 ID: "}{&props.file.file_id}</span>
                    <span>{"⏰ Expira em: "}{expires}</span>
                </div>
            </div>

            <div class="file-actions">
                if downloading {
                    <div class="upload-loading">
                        <div class="spinner"></div>
                        <span>{"..."}</span>
                    </div>
                } else {
                    <button
                        onclick={on_download}
                        class="btn-download"
                        disabled={downloading}
                    >
                        { "⬇️ Baixar" }
                    </button>
                }

                <button
                    onclick={on_copy}
                    class="btn-copy"
                    type="button"
                >
                    {"📋 Copiar link"}
                </button>
            </div>
        </div>
    }
}

////// Arquivo: ./src/components/upload_form.rs
use yew::prelude::*;
use web_sys::{Event, File, HtmlInputElement};
use wasm_bindgen::JsCast;
use crate::hooks::use_file_upload::use_file_upload;
use crate::store::files_store::FilesStoreContext;
use crate::utils::formatters::format_bytes;

#[function_component(UploadForm)]
pub fn upload_form() -> Html {
    let selected_file = use_state(|| None::<File>);
    let store = use_context::<FilesStoreContext>().expect("FilesStoreContext not found");
    let loading = store.loading; // usa o loading do store
    let upload = use_file_upload();

    // Quando o usuário seleciona um arquivo
    let on_file_change = {
        let selected_file = selected_file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    selected_file.set(Some(file));
                    return;
                }
            }
            selected_file.set(None);
        })
    };

    // Ao clicar em upload
    let on_upload = {
        let selected_file = selected_file.clone();
        let upload = upload.clone();
        Callback::from(move |_| {
            if let Some(file) = (*selected_file).clone() {
                upload.emit(file); // dispara o hook de upload
                selected_file.set(None); // limpa seleção
            }
        })
    };

    // Limpar seleção de arquivo
    let on_clear = {
        let selected_file = selected_file.clone();
        Callback::from(move |_| {
            selected_file.set(None);
        })
    };

    html! {
        <div class="upload-section">
            <div class="file-input-wrapper">
                <input
                    type="file"
                    id="file-input"
                    onchange={on_file_change}
                    disabled={loading}
                />
                <label for="file-input" class="file-label">
                    {"🔍 Escolher arquivo"}
                </label>
            </div>

            // Spinner e mensagem enquanto estiver carregando
            if loading {
                <div class="upload-loading">
                    <div class="spinner"></div>
                    <span>{"Enviando arquivo..."}</span>
                </div>
            }

            if let Some(file) = (*selected_file).as_ref() {
                <div class="selected-file">
                    <span class="file-name">{file.name()}</span>
                    <span class="file-size">{format_bytes(file.size() as u64)}</span>
                    <button onclick={on_clear} class="btn-clear" disabled={loading}>
                        {"✕"}
                    </button>
                </div>

                if !loading {
                    <button
                        onclick={on_upload}
                        class="btn-upload"
                        disabled={loading}
                    >
                        {"⬆️ Fazer Upload"}
                    </button>
                }
            }
        </div>
    }
}

////// Arquivo: ./src/components/mod.rs
pub mod error_banner;
pub mod file_card;
pub mod file_list;
pub mod upload_form;


////// Arquivo: ./src/components/error_banner.rs
use yew::prelude::*;
use crate::store::files_store::FilesStoreContext;

#[function_component(ErrorBanner)]
pub fn error_banner() -> Html {
    let store = use_context::<FilesStoreContext>().expect("FilesStoreContext not found");

    if let Some(error) = &store.error {
        html! {
            <div class="error-message">
                {"❌ "}{error}
            </div>
        }
    } else {
        html! {}
    }
}

////// Arquivo: ./src/app.rs
use yew::prelude::*;
use crate::components::{error_banner::ErrorBanner, file_list::FileList, upload_form::UploadForm};
use crate::store::files_store::{FilesStore, FilesStoreContext};
use crate::utils::constants::API_URL;
use gloo::console::log;

#[function_component(App)]
pub fn app() -> Html {
    let store = use_reducer(|| FilesStore::load());

    log!(format!("{}", &API_URL));

    html! {
        <ContextProvider<FilesStoreContext> context={store}>
            <div class="container">
                <header>
                    <h1>{"📁 Quickshare"}</h1>
                    <p class="subtitle">{"Compartilhe arquivos temporários (válidos por 24 horas)"}</p>
                </header>

                <ErrorBanner />
                
                <UploadForm />

                <FileList />
            </div>
        </ContextProvider<FilesStoreContext>>
    }
}

////// Arquivo: ./src/models/file.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct StoredFile {
    pub file_id: String,
    pub filename: String,
    pub download_url: String,
    pub expires_at: String,
    pub uploaded_at: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct UploadResponse {
    pub file_id: String,
    pub download_url: String,
    pub expires_at: String,
}

////// Arquivo: ./src/models/error.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    FormDataCreation,
    FormDataAppend,
    RequestBuild(String),
    NetworkError(String),
    JsonParse(String),
    ServerError(u16),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::FormDataCreation => write!(f, "Erro ao criar FormData"),
            ApiError::FormDataAppend => write!(f, "Erro ao adicionar arquivo ao formulário"),
            ApiError::RequestBuild(e) => write!(f, "Erro ao montar requisição: {}", e),
            ApiError::NetworkError(e) => write!(f, "Erro de rede: {}", e),
            ApiError::JsonParse(e) => write!(f, "Erro ao processar resposta: {}", e),
            ApiError::ServerError(code) => write!(f, "Erro no servidor (código {})", code),
        }
    }
}

impl std::error::Error for ApiError {}


////// Arquivo: ./src/models/mod.rs
pub mod error;
pub mod file;


////// Arquivo: ./src/store/files_store.rs
use yew::prelude::*;
use std::rc::Rc;
use crate::models::file::StoredFile;
use crate::services::storage::StorageService;

#[derive(Clone, PartialEq)]
pub struct FilesStore {
    pub files: Vec<StoredFile>,
    pub loading: bool,
    pub error: Option<String>,
    pub downloading_files: Vec<String>, // lista de file_id em download
}

impl Default for FilesStore {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            loading: false,
            error: None,
            downloading_files: Vec::new(),
        }
    }
}

impl FilesStore {
    pub fn load() -> Self {
        Self {
            files: StorageService::load_files(),
            loading: false,
            error: None,
            downloading_files: Vec::new(),
        }
    }
}

#[allow(dead_code)]
pub enum FilesStoreAction {
    AddFile(StoredFile),
    RemoveFile(String),
    ClearAll,
    SetLoading(bool),
    SetError(Option<String>),
    StartDownload(String),   // file_id
    EndDownload(String),     // file_id
}

impl Reducible for FilesStore {
    type Action = FilesStoreAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_store = (*self).clone();
        
        match action {
            FilesStoreAction::AddFile(file) => {
                new_store.files.insert(0, file);
                StorageService::save_files(&new_store.files);
            }
            FilesStoreAction::RemoveFile(file_id) => {
                new_store.files.retain(|f| f.file_id != file_id);
                StorageService::save_files(&new_store.files);
            }
            FilesStoreAction::ClearAll => {
                new_store.files.clear();
                StorageService::clear_files();
            }
            FilesStoreAction::SetLoading(loading) => {
                new_store.loading = loading;
            }
            FilesStoreAction::SetError(error) => {
                new_store.error = error;
            }
            FilesStoreAction::StartDownload(file_id) => {
                if !new_store.downloading_files.contains(&file_id) {
                    new_store.downloading_files.push(file_id);
                }
            }
            FilesStoreAction::EndDownload(file_id) => {
                new_store.downloading_files.retain(|id| id != &file_id);
            }
        }
        
        Rc::new(new_store)
    }
}

pub type FilesStoreContext = UseReducerHandle<FilesStore>;

////// Arquivo: ./src/store/mod.rs
pub mod files_store;

////// Arquivo: ./saida.rs


