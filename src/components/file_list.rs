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