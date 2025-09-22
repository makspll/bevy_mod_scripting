# Script Pipeline

Processing script atachments and detachments is all done by the script pipeline.

In versions prior to `16.0` the pipeline was represented by load unload commands, which atomically processed scripts. Since then the pipeline has been maassively overhauled to allow:
- Customizing callbacks during the process
- Budgeting frame time for the pipeline
- Allowing async steps in the process

The new pipeline works like this:
- A set of systems in `PostUpdate` "filter" incoming `ScriptAttachedEvent` and `ScriptDetachedEvent`'s as well as `ScriptAssetModifiedEvent`'s (some generated from asset events, others from component hooks). The filtered events are wrapped in `ForPlugin<P>(inner)` events based on the language of the underlying event's.
- Then inside the `ScriptProcessingSchedule<P>`