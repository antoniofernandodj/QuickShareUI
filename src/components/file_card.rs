use yew::prelude::*;
use crate::hooks::use_clipboard::use_clipboard;
use crate::models::file::StoredFile;
use crate::utils::constants::API_URL;
use crate::utils::formatters::format_expires;

#[derive(Properties, PartialEq)]
pub struct FileCardProps {
    pub file: StoredFile,
}

#[function_component(FileCard)]
pub fn file_card(props: &FileCardProps) -> Html {
    let copy_to_clipboard = use_clipboard();
    let download_url = format!("{}{}", API_URL, props.file.download_url);
    let expires = format_expires(&props.file.expires_at);

    let on_copy = {
        let url = download_url.clone();
        let copy = copy_to_clipboard.clone();
        Callback::from(move |_| {
            copy.emit(url.clone());
        })
    };

    html! {
        <div class="file-card">
            <div class="file-info">
                <div class="file-name-large">
                    {"📄 "}{&props.file.filename}
                </div>
                <div class="file-meta">
                    <span>{"🆔 ID: "}{&props.file.file_id[..8.min(props.file.file_id.len())]}{"..."}</span>
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