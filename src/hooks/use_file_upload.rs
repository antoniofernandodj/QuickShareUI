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
