pub mod bridge;
pub mod entity;
pub mod revision_storage;
pub mod types;

pub use bridge::{check_documents_exist, fetch_documents_batch};
pub use revision_storage::RevisionStorageClient;
pub use types::{DocumentExistence, DocumentNamespace};
