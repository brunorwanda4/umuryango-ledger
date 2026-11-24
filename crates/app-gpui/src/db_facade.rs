//! Optional SQLite local storage facade for native desktop & mobile targets.

#[cfg(not(target_arch = "wasm32"))]
pub use umuryango_storage_local::{
    CacheRepository, LocalDatabase, SyncMutation, SyncQueueRepository,
};

#[cfg(target_arch = "wasm32")]
pub struct LocalDatabase;

#[cfg(target_arch = "wasm32")]
impl LocalDatabase {
    pub fn open_in_memory() -> Result<Self, String> {
        Ok(Self)
    }
}
