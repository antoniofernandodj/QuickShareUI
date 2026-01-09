use yew::prelude::*;
use crate::components::{error_banner::ErrorBanner, file_list::FileList, upload_form::UploadForm};
use crate::store::files_store::{FilesStore, FilesStoreContext};

#[function_component(App)]
pub fn app() -> Html {
    let store = use_reducer(|| FilesStore::load());

    html! {
        <ContextProvider<FilesStoreContext> context={store}>
            <div class="container">
                <header>
                    <h1>{"📁 Upload de Arquivos"}</h1>
                    <p class="subtitle">{"Compartilhe arquivos temporários (válidos por 24 horas)"}</p>
                </header>

                <ErrorBanner />
                
                <UploadForm />

                <FileList />
            </div>
        </ContextProvider<FilesStoreContext>>
    }
}