use parking_lot::{Mutex, RwLock};
use wasmtime::{Config, Engine, component::Linker};

use crate::WasmtimeStoreData;

pub type WasmtimeRuntime = RwLock<WasmtimeRuntimeInner>;
/// Shared wasmtime runtime.  
pub struct WasmtimeRuntimeInner {
    pub engine: Engine,
    /// Cached linker, populated once from the BMS function registry.  
    pub linker: Linker<WasmtimeStoreData>,
}

impl Default for WasmtimeRuntimeInner {
    fn default() -> Self {
        Self::new()
    }
}
impl WasmtimeRuntimeInner {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config).expect("wasmtime Engine creation failed");
        Self {
            linker: Linker::new(&engine),
            engine,
        }
    }
}
