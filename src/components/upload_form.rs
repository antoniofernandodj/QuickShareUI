use yew::prelude::*;
use web_sys::{Event, File, HtmlInputElement};
use wasm_bindgen::JsCast;
use crate::hooks::use_file_upload::use_file_upload;
use crate::utils::formatters::format_bytes;

#[function_component(UploadForm)]
pub fn upload_form() -> Html {
    let selected_file = use_state(|| None::<File>);
    let (upload, loading, _error) = use_file_upload();

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

    let on_upload = {
        let selected_file = selected_file.clone();
        let upload = upload.clone();
        Callback::from(move |_| {
            if let Some(file) = (*selected_file).clone() {
                upload.emit(file);
                selected_file.set(None);
            }
        })
    };

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

            if let Some(file) = (*selected_file).as_ref() {
                <div class="selected-file">
                    <span class="file-name">{file.name()}</span>
                    <span class="file-size">{format_bytes(file.size() as u64)}</span>
                    <button onclick={on_clear} class="btn-clear" disabled={loading}>
                        {"✕"}
                    </button>
                </div>

                <button
                    onclick={on_upload}
                    class="btn-upload"
                    disabled={loading}
                >
                    if loading {
                        {"⏳ Enviando..."}
                    } else {
                        {"⬆️ Fazer Upload"}
                    }
                </button>
            }
        </div>
    }
}