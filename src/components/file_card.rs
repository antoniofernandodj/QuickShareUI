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