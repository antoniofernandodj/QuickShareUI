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