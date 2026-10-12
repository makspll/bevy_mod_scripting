# bevy_mod_scripting_core

Language-agnostic script lifecycle, callbacks and script systems.

* Public `event.rs` types are Bevy *messages* (buffered); lifecycle state in `pipeline/` uses *events* (observers). Keep that split.
* Expose to scripts only what scripts can safely construct; keep internal pipeline state out of reflection.
* Script contexts sit behind non-re-entrant locks: never run a callback inline from something a script can trigger. Queue it as a command instead.
* Run callbacks only through `P::handle` (`ScriptingHandler`), which locks the context itself: it turns same-thread re-entry into an error instead of a deadlock and caps nesting depth. Never lock a context to run a callback by hand.
