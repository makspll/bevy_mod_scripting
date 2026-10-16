// AnimationPlayer without any animations playing
let AnimationPlayer = import_namespace.call("animation").AnimationPlayer;
let NodeIndex = import_namespace.call("animation").NodeIndex;
let player = AnimationPlayer.new_.call(#{});
let node = NodeIndex.new_.call(0);
assert(!player.is_playing_animation.call(node), "nothing is playing in a new player");
assert(player.all_finished.call(), "a player with no animations is finished");
assert(player.all_paused.call(), "a player with no animations is paused");
assert(type_of(player.animation_is_finished.call(node)) == "()", "a stopped animation has no finished state");
player.stop_all.call();
player.pause_all.call();
player.resume_all.call();
assert(!player.is_playing_animation.call(node), "still nothing is playing");
