# bevy_mod_scripting_core

Language-agnostic script lifecycle, callbacks and script systems.

* Public `event.rs` types are Bevy *messages* (buffered); lifecycle state in `pipeline/` uses *events* (observers). Keep that split.
* Expose to scripts only what scripts can safely construct; keep internal pipeline state out of reflection.
* Script contexts sit behind non-re-entrant locks: never run a callback inline from something a script can trigger. Queue it with `AppScriptCallbackRunners::queue`, or, if it should run right away when possible, run it with `AppScriptCallbackRunners::run_or_queue`, which queues it for contexts that are busy. Only these may run callbacks from a Bevy command (as script observers do): the world's command queue can be flushed in the middle of a script (e.g. by `world.spawn`).
* Errors of callbacks run outside the handler systems (i.e. script observers) go through `AppScriptCallbackRunners::report_error`, which this crate points at the script error messages.
* Run callbacks only through `P::handle` (`ScriptingHandler`), which locks the context itself: it turns same-thread re-entry into an error instead of a deadlock and caps nesting depth. Never lock a context to run a callback by hand.
