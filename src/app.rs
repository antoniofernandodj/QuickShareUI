use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::components::{error_banner::ErrorBanner, file_list::FileList, upload_form::UploadForm};
use crate::services::api::ApiClient;
use crate::store::files_store::{FilesStore, FilesStoreAction, FilesStoreContext};
use crate::utils::constants::API_URL;
use gloo::console;

#[function_component(App)]
pub fn app() -> Html {
    let store = use_reducer(|| FilesStore::load());

    console::log!(format!("{}", &API_URL));

    // Efeito para validar arquivos ao montar o componente
    {
        let store = store.clone();
        use_effect_with((), move |_| {
            let store = store.clone();
            spawn_local(async move {
                let current_files = store.files.clone();
                
                if !current_files.is_empty() {
                    console::log!("Validando arquivos existentes...");
                    let valid_files = ApiClient::validate_files(current_files).await;
                    
                    console::log!(format!("Arquivos válidos: {}", valid_files.len()));
                    store.dispatch(FilesStoreAction::SetValidatedFiles(valid_files));
                }
                
                store.dispatch(FilesStoreAction::SetValidating(false));
            });
            || ()
        });
    }

    html! {
        <ContextProvider<FilesStoreContext> context={store.clone()}>
            <div class="container">
                <header>
                    <h1>{"📁 Quickshare"}</h1>
                    <p class="subtitle">{"Compartilhe arquivos temporários (válidos por 24 horas)"}</p>
                </header>
                <ErrorBanner />
                if store.validating {
                    <div class="upload-loading">
                        <div class="spinner"></div>
                        <span>{"Verificando arquivos..."}</span>
                    </div>
                } else {
                    <>
                        <UploadForm />
                        <FileList />
                    </>
                }
            </div>
        </ContextProvider<FilesStoreContext>>
    }
}