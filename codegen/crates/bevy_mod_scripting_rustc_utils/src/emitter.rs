use std::{io, sync::{Arc, Mutex, OnceLock, atomic::{AtomicBool, Ordering}}};

use rustc_data_structures::sync::DynSend;
use rustc_errors::{DiagInner, annotate_snippet_emitter_writer::AnnotateSnippetEmitter, emitter::{DynEmitter, Emitter, HumanReadableErrorType, OutputTheme, stderr_destination}, json::JsonEmitter};
use rustc_middle::ty::{TyCtxt};
use rustc_session::{Session, config::ErrorOutputType};
use rustc_span::source_map::SourceMap;

#[derive(Default)]
pub struct Buffer {
    messages: Vec<String>,
    has_errors: bool,
}

impl Buffer {
    // #[allow(clippy::arc_with_non_send_sync)]
    // pub fn new() -> Arc<Mutex<Buffer>> {
    //     Arc::new(Mutex::new(Self::default()))
    // }

    pub fn capture_and_reset(&mut self) -> (String, bool) {
        let ret = (self.messages.join("\n"), self.has_errors);
        *self = Default::default();
        ret
    }
}

pub struct CaptureState {  
    pub capture: AtomicBool,  
    pub buffer: Mutex<Buffer>,  
    /// Set to `true` by `DelegatingBufferEmitter::new`, so we can tell  
    /// whether the consumer actually installed our emitter on the dcx.  
    pub installed: AtomicBool,  
}  
  
/// Global, process-wide handle to the capture state. Populated once when  
/// `DelegatingBufferEmitter` is constructed and wrapped into the real `DiagCtxt`.  
static CAPTURE_STATE: OnceLock<Arc<CaptureState>> = OnceLock::new();  
  
impl CaptureState {  
    fn get_or_init() -> &'static Arc<CaptureState> {  
        CAPTURE_STATE.get_or_init(|| {  
            Arc::new(CaptureState {  
                capture: AtomicBool::new(false),  
                buffer: Mutex::new(Buffer::default()),  
                installed: AtomicBool::new(false),  
            })  
        })  
    }  
  
    /// Call once, right after building the emitter, so callers can detect  
    /// a missing `set_emitter` at startup.  
    pub fn install(session: &Session) {  
        session.dcx().set_emitter(Box::new(DelegatingBufferEmitter::new(build_real_emitter(session))));
        Self::get_or_init().installed.store(true, Ordering::SeqCst);  
    }  
  
    pub fn is_installed() -> bool {  
        Self::get_or_init().installed.load(Ordering::SeqCst)  
    }  
  
    /// Turn capturing on, run `f`, turn it back off, and return the  
    /// captured messages. Panics loudly if the delegating emitter was  
    /// never installed, instead of silently emitting to stderr.  
    pub fn capture<T>(ctxt: TyCtxt<'_>, f: impl FnOnce() -> T) -> (T, String, bool) {  
        assert!(  
            Self::is_installed(),  
            "DelegatingBufferEmitter was never installed on the DiagCtxt; 
             call CaptureState::install(&session) from your callbacks"  
        );  
  
        let state = Self::get_or_init();  
        state.capture.store(true, Ordering::SeqCst);  
        let had_errors_before = ctxt.dcx().has_errors().is_some();
        let result = f();  
        
        if !had_errors_before {
            ctxt.dcx().reset_err_count();
        }
        state.capture.store(false, Ordering::SeqCst);  
  
        let mut buffer = state.buffer.lock().unwrap();  
        let (messages, has_errors) = buffer.capture_and_reset();
        (result, messages, has_errors)  
    }  
}

 pub struct DelegatingBufferEmitter {  
    inner: Box<DynEmitter>,  
}  
  
impl DelegatingBufferEmitter {  
    pub fn new(inner: Box<DynEmitter>) -> Self {  
        Self { inner }  
    }  
}  
  
impl Emitter for DelegatingBufferEmitter {  
    fn emit_diagnostic(&mut self, diag: DiagInner) {  
        let state = CaptureState::get_or_init();  
        if state.capture.load(Ordering::SeqCst) {  
            let mut buffer = state.buffer.lock().unwrap();  
            let msg = rustc_errors::format_diag_message(&diag.messages[0].0, &diag.args);  
            buffer.messages.push(msg.into_owned());  
            if diag.is_error() {  
                buffer.has_errors = true;  
            }  
        } else {  
            self.inner.emit_diagnostic(diag);  
        }  
    }  
  
    fn source_map(&self) -> Option<&SourceMap> {  
        self.inner.source_map()  
    }  
}

fn build_real_emitter(sess: &Session) -> Box<dyn Emitter + DynSend> {  
    // same shape as librustdoc::core::new_dcx / rustc_session::session::mk_emitter  
    match sess.opts.error_format {  
        ErrorOutputType::HumanReadable { kind: HumanReadableErrorType { short, unicode }, color_config } => {  
            Box::new(  
                AnnotateSnippetEmitter::new(stderr_destination(color_config))  
                    .sm(Some(sess.psess.clone_source_map()))  
                    .short_message(short)  
                    .diagnostic_width(sess.opts.diagnostic_width)  
                    .track_diagnostics(sess.opts.unstable_opts.track_diagnostics)  
                    .theme(if unicode { OutputTheme::Unicode } else { OutputTheme::Ascii }),  
            )  
        }  
        ErrorOutputType::Json { pretty, json_rendered, color_config } => {  
            Box::new(  
                JsonEmitter::new(  
                    Box::new(io::BufWriter::new(io::stderr())),  
                    Some(sess.psess.clone_source_map()),  
                    pretty,  
                    json_rendered,  
                    color_config,  
                )  
                .diagnostic_width(sess.opts.diagnostic_width)  
                .track_diagnostics(sess.opts.unstable_opts.track_diagnostics),  
            )  
        }  
    }  
}