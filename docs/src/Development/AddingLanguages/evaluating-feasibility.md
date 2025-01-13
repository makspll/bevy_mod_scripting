# Evaluating Feasibility

In order for a language to work well with BMS it's necessary it supports the following features:
- [ ] Interoperability with Rust. If you can't call it from Rust easilly and there is no existing crate that can do it for you, it's a no-go.
- [ ] First class functions. Or at least the ability to call an arbitrary function with an arbitrary number of arguments from a script. Without this feature, you would need to separately generate code for the bevy bindings. This is painful and goes against the grain of the project.

## First Classs Functions

