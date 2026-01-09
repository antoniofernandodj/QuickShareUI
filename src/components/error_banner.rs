use yew::prelude::*;
use crate::store::files_store::FilesStoreContext;

#[function_component(ErrorBanner)]
pub fn error_banner() -> Html {
    let store = use_context::<FilesStoreContext>().expect("FilesStoreContext not found");

    if let Some(error) = &store.error {
        html! {
            <div class="error-message">
                {"❌ "}{error}
            </div>
        }
    } else {
        html! {}
    }
}