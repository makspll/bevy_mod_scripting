// UiRect sides
let BorderRadius = import_namespace.call("ui").BorderRadius;
let ComputedNode = import_namespace.call("ui").ComputedNode;
let Overflow = import_namespace.call("ui").Overflow;
let UiRect = import_namespace.call("ui").UiRect;
let Val = import_namespace.call("ui").Val;
let all = UiRect.all.call(Val.px.call(2.0));
let left = all.left;
let bottom = all.bottom;
assert(left[0] == 2.0 && bottom[0] == 2.0, "all");
let sides = UiRect.px.call(1.0, 2.0, 3.0, 4.0);
let top = sides.top;
assert(top[0] == 3.0, "px");
let percent_bottom = UiRect.percent.call(1.0, 2.0, 3.0, 4.0).bottom;
assert(percent_bottom.variant_name.call() == "Percent", "percent");
let axes = UiRect.axes.call(Val.px.call(1.0), Val.px.call(2.0));
let axes_top = axes.top;
assert(axes_top[0] == 2.0, "axes");

// BorderRadius corners, each with its radius as `x`
let radius = BorderRadius.all.call(Val.px.call(3.0));
let corner = radius.top_left.x;
assert(corner[0] == 3.0, "border radius all");

// Overflow on each axis
let clip = Overflow.clip.call();
assert(clip.x.variant_name.call() == "Clip" && clip.y.variant_name.call() == "Clip", "clip");

// ComputedNode of a node that has not been laid out
let computed = ComputedNode.new_.call(#{});
assert(computed.is_empty.call(), "a node with no size is empty");
