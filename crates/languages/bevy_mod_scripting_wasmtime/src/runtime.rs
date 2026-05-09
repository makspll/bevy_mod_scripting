use parking_lot::Mutex;
use wasmtime::{Config, Engine, component::Linker};

use crate::WasmtimeStoreData;

/// Shared wasmtime runtime.  
pub struct WasmtimeRuntime {
    pub engine: Engine,
    /// Cached linker, populated once from the BMS function registry.  
    pub linker: Mutex<Option<Linker<WasmtimeStoreData>>>,
}

impl Default for WasmtimeRuntime {
    fn default() -> Self {
        Self::new()
    }
}
impl WasmtimeRuntime {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config).expect("wasmtime Engine creation failed");
        Self {
            engine,
            linker: Mutex::new(None),
        }
    }
}
