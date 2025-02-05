# Scripting Reference

This part of the book covers the user-facing API of the scripting languages supported by BMS. This will be where you will want to forward your script users to get started with scripting in BMS.

If you are a modder, welcome! 👋, apologies for the rust-centricity of this guide, we are working on it!

## Globals

Scripts will have access to a few global variables in most callbacks:
- `world`: a static reference to the world, with all sorts of functions available
- `entity`: the entity the script is attached to, not available on load/unload callbacks
- `script_id`: the ID of the current script 
