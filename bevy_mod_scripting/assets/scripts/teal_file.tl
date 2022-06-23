-- this is a teal file!
-- the recommended way of developing with teal is 
-- opening a new vscode window with the top directory 
-- being `scripts`
-- the official teal extension will then automatically pick up
-- on all of the types defined in `types/global_types.d.tl`
-- and provide type hints, errors and maybe inlined docstrings in the future. 

-- teal files will be compiled to lua files when loaded automatically during development time 
-- when you're ready to ship your game, you will need to compile those scripts to pure .lua files
-- with `tl build` in this directory, it's that simple! You don't need to ship any teal files,
-- but you might decide to to provide your modders with a level playing ground!
function on_update()
    local lol : string = my_api.my_function()
    print(lol)
end