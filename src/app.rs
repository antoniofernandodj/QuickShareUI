use yew::prelude::*;
use crate::components::{error_banner::ErrorBanner, file_list::FileList, upload_form::UploadForm};
use crate::store::files_store::{FilesStore, FilesStoreContext};
use crate::utils::constants::API_URL;
use gloo::console;

#[function_component(App)]
pub fn app() -> Html {
    let store = use_reducer(|| FilesStore::load());

    console::log!(format!("{}", &API_URL));

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