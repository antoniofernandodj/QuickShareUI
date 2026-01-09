use yew::prelude::*;
use std::rc::Rc;
use crate::models::file::StoredFile;
use crate::services::storage::StorageService;

#[derive(Clone, PartialEq)]
pub struct FilesStore {
    pub files: Vec<StoredFile>,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for FilesStore {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            loading: false,
            error: None,
        }
    }
}

impl FilesStore {
    pub fn load() -> Self {
        Self {
            files: StorageService::load_files(),
            loading: false,
            error: None,
        }
    }
}

#[allow(dead_code)]
pub enum FilesStoreAction {
    AddFile(StoredFile),
    RemoveFile(String),
    ClearAll,
    SetLoading(bool),
    SetError(Option<String>),
}

impl Reducible for FilesStore {
    type Action = FilesStoreAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_store = (*self).clone();
        
        match action {
            FilesStoreAction::AddFile(file) => {
                new_store.files.insert(0, file);
                StorageService::save_files(&new_store.files);
            }
            FilesStoreAction::RemoveFile(file_id) => {
                new_store.files.retain(|f| f.file_id != file_id);
                StorageService::save_files(&new_store.files);
            }
            FilesStoreAction::ClearAll => {
                new_store.files.clear();
                StorageService::clear_files();
            }
            FilesStoreAction::SetLoading(loading) => {
                new_store.loading = loading;
            }
            FilesStoreAction::SetError(error) => {
                new_store.error = error;
            }
        }
        
        Rc::new(new_store)
    }
}

pub type FilesStoreContext = UseReducerHandle<FilesStore>;