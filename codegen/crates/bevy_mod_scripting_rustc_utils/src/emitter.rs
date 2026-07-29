use std::sync::Arc;

use rustc_data_structures::sync::Lock;
use rustc_errors::{DiagInner, emitter::Emitter, format_diag_message};
use rustc_span::source_map::SourceMap;

#[derive(Default)]
pub struct Buffer {
    messages: Vec<String>,
    has_errors: bool,
}

impl Buffer {
    pub fn new() -> Arc<Lock<Buffer>> {
        Arc::new(Lock::new(Self::default()))
    }

    pub fn buffer(&self) -> String {
        self.messages.join("\n")
    }
}
 
pub struct BufferEmitter {
    buffer: Arc<Lock<Buffer>>,
}

impl BufferEmitter {
    pub fn new(lock: Arc<Lock<Buffer>>) -> Self {
        Self {
            buffer: lock,
        }
    }
}
 
impl Emitter for BufferEmitter {
    fn emit_diagnostic(&mut self, diag: DiagInner) {
        let mut buffer = self.buffer.borrow_mut();
 
        let translated_main_message = format_diag_message(&diag.messages[0].0, &diag.args);
 
        buffer.messages.push(translated_main_message.to_string());
        if diag.is_error() {
            buffer.has_errors = true;
        }
    }
 
    fn source_map(&self) -> Option<&SourceMap> {
        None
    }
}