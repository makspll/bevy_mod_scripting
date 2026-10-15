-- AnimationPlayer without any animations playing
local AnimationPlayer = import_namespace("animation").AnimationPlayer
local NodeIndex = import_namespace("animation").NodeIndex
local player = AnimationPlayer.new({})
local node = NodeIndex.new(0)
assert(not player:is_playing_animation(node), "nothing is playing in a new player")
assert(player:all_finished(), "a player with no animations is finished")
assert(player:all_paused(), "a player with no animations is paused")
assert(player:animation_is_finished(node) == nil, "a stopped animation has no finished state")
assert(not pcall(function() player:set_speed(node, 2.0) end), "set_speed errors when not playing")
assert(not pcall(function() player:seek_to(node, 1.0) end), "seek_to errors when not playing")
assert(not pcall(function() player:set_repeat_forever(node) end), "set_repeat_forever errors when not playing")
player:stop(node)
player:stop_all()
player:pause_all()
player:resume_all()
assert(not player:is_playing_animation(node), "still nothing is playing")
