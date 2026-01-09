use yew::{html::Scope, prelude::*};
use web_sys::{File, FormData, HtmlInputElement, window};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize)]
struct UploadResponse {
    file_id: String,
    download_url: String,
    expires_at: String,
}

enum Msg {
    FileSelected(File),
    UploadFile,
    UploadSuccess(UploadResponse, String),
    UploadError(String),
    ClearFile,
    CopyLink(String),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct StoredFile {
    file_id: String,
    filename: String,
    download_url: String,
    expires_at: String,
}

const STORAGE_KEY: &str = "quickshare_uploaded_files";


struct App {
    selected_file: Option<File>,
    stored_files: Vec<StoredFile>,
    uploading: bool,
    error_message: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_file: None,
            stored_files: load_files_from_storage(),
            uploading: false,
            error_message: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileSelected(file) => {
                self.selected_file = Some(file);
                self.error_message = None;
                true
            }
            Msg::UploadFile => {
                if let Some(file) = self.selected_file.clone() {
                    self.uploading = true;
                    self.error_message = None;
                    
                    let link = ctx.link().clone();
                    let filename = file.name();
                    
                    spawn_local(async move {
                        match upload_file(file).await {
                            Ok(response) => {
                                link.send_message(Msg::UploadSuccess(response, filename));
                            }
                            Err(e) => {
                                link.send_message(Msg::UploadError(e));
                            }
                        }
                    });
                }
                true
            }
            Msg::UploadSuccess(response, filename) => {
                let stored = StoredFile {
                    file_id: response.file_id.clone(),
                    filename,
                    download_url: response.download_url.clone(),
                    expires_at: response.expires_at.clone(),
                };

                self.stored_files.insert(0, stored);
                save_files_to_storage(&self.stored_files);

                self.uploading = false;
                self.selected_file = None;
                true
            }
            Msg::UploadError(error) => {
                self.error_message = Some(error);
                self.uploading = false;
                true
            }
            Msg::CopyLink(link) => {
                spawn_local(async move {
                    if let Some(window) = web_sys::window() {
                        let navigator = window.navigator().clipboard();
                        let _ = navigator.write_text(&link);
                    }
                });
                false
            }
            Msg::ClearFile => {
                self.selected_file = None;
                self.error_message = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_file_change: Callback<Event> = ctx.link().callback(|e: Event| {
            let input: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    return Msg::FileSelected(file);
                }
            }
            Msg::ClearFile
        });

        let on_upload: Callback<MouseEvent> = ctx.link().callback(|_| Msg::UploadFile);
        let on_clear: Callback<MouseEvent> = ctx.link().callback(|_| Msg::ClearFile);

        html! {
            <div class="container">
                <header>
                    <h1>{"📁 Upload de Arquivos"}</h1>
                    <p class="subtitle">{"Compartilhe arquivos temporários (válidos por 24 horas)"}</p>
                </header>

                <div class="upload-section">
                    <div class="file-input-wrapper">
                        <input
                            type="file"
                            id="file-input"
                            onchange={on_file_change}
                            disabled={self.uploading}
                        />
                        <label for="file-input" class="file-label">
                            {"🔍 Escolher arquivo"}
                        </label>
                    </div>

                    if let Some(file) = &self.selected_file {
                        <div class="selected-file">
                            <span class="file-name">{&file.name()}</span>
                            <span class="file-size">{format_bytes(file.size() as u64)}</span>
                            <button onclick={on_clear} class="btn-clear" disabled={self.uploading}>
                                {"✕"}
                            </button>
                        </div>

                        <button
                            onclick={on_upload}
                            class="btn-upload"
                            disabled={self.uploading}
                        >
                            if self.uploading {
                                {"⏳ Enviando..."}
                            } else {
                                {"⬆️ Fazer Upload"}
                            }
                        </button>
                    }

                    if let Some(error) = &self.error_message {
                        <div class="error-message">
                            {"❌ "}{error}
                        </div>
                    }
                </div>

                if !self.stored_files.is_empty() {
                    <div class="files-list">
                        <h2>{"Arquivos Enviados"}</h2>
                        {
                            for self.stored_files.iter().map(|file| {
                                view_stored_file(file, ctx.link())
                            })
                        }
                    </div>
                }
            </div>
        }
    }
}


fn view_stored_file(file: &StoredFile, link: &Scope<App>) -> Html {
    let download_url = format!("http://127.0.0.1:8000{}", file.download_url);
    let expires = format_expires(&file.expires_at);

    let on_copy = {
        let url = download_url.clone();
        link.callback(move |_| Msg::CopyLink(url.clone()))
    };

    html! {
        <div class="file-card">
            <div class="file-info">
                <div class="file-name-large">
                    {"📄 "}{&file.filename}
                </div>
                <div class="file-meta">
                    <span>{"🆔 ID: "}{&file.file_id[..8]}{"..."}</span>
                    <span>{"⏰ Expira em: "}{expires}</span>
                </div>
            </div>

            <div class="file-actions">
                <a
                    href={download_url.clone()}
                    class="btn-download"
                    target="_blank"
                    rel="noopener noreferrer"
                >
                    {"⬇️ Baixar"}
                </a>

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


async fn upload_file(file: File) -> Result<UploadResponse, String> {
    let form_data = FormData::new().map_err(|_| "Erro ao criar FormData")?;
    form_data
        .append_with_blob("file", &file)
        .map_err(|_| "Erro ao adicionar arquivo ao formulário")?;

    let response = Request::post("http://127.0.0.1:8000/upload")
        .body(form_data)
        .map_err(|e| format!("Erro ao montar requisição: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Erro na requisição: {}", e))?;

    if response.ok() {
        response
            .json::<UploadResponse>()
            .await
            .map_err(|e| format!("Erro ao processar resposta: {}", e))
    } else {
        Err(format!("Erro no servidor: {}", response.status()))
    }
}


fn format_bytes(bytes: u64) -> String {
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


fn load_files_from_storage() -> Vec<StoredFile> {
    let window = window().expect("no window");
    let storage = window.local_storage().unwrap().unwrap();

    storage
        .get_item(STORAGE_KEY)
        .ok()
        .flatten()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

fn save_files_to_storage(files: &[StoredFile]) {
    let window = window().expect("no window");
    let storage = window.local_storage().unwrap().unwrap();

    let _ = storage.set_item(
        STORAGE_KEY,
        &serde_json::to_string(files).unwrap(),
    );
}

fn format_expires(expires_at: &str) -> String {
    // Simplificado - mostra apenas a data/hora
    expires_at.split('T').next().unwrap_or(expires_at).to_string()
}

fn main() {
    yew::Renderer::<App>::new().render();
}