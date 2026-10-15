-- UiRect sides
local BorderColor = import_namespace("ui").BorderColor
local BorderRadius = import_namespace("ui").BorderRadius
local Color = import_namespace("color").Color
local ComputedNode = import_namespace("ui").ComputedNode
local Overflow = import_namespace("ui").Overflow
local UiRect = import_namespace("ui").UiRect
local Val = import_namespace("ui").Val
local all = UiRect.all(Val.px(2))
assert(all.left[1] == 2 and all.right[1] == 2 and all.top[1] == 2 and all.bottom[1] == 2, "all")
local sides = UiRect.px(1, 2, 3, 4)
assert(sides.left[1] == 1 and sides.right[1] == 2 and sides.top[1] == 3 and sides.bottom[1] == 4, "px")
assert(UiRect.percent(1, 2, 3, 4).bottom:variant_name() == "Percent", "percent")
local axes = UiRect.axes(Val.px(1), Val.px(2))
assert(axes.left[1] == 1 and axes.right[1] == 1 and axes.top[1] == 2 and axes.bottom[1] == 2, "axes")
assert(UiRect.horizontal(Val.px(3)).right[1] == 3 and UiRect.horizontal(Val.px(3)).top[1] == 0, "horizontal")
assert(UiRect.vertical(Val.px(3)).bottom[1] == 3 and UiRect.vertical(Val.px(3)).left[1] == 0, "vertical")
local new = UiRect.new(Val.px(1), Val.px(2), Val.px(3), Val.px(4))
assert(new.left[1] == 1 and new.right[1] == 2 and new.top[1] == 3 and new.bottom[1] == 4, "new")
assert(UiRect.zero().left[1] == 0 and UiRect.zero().bottom[1] == 0, "zero")

-- BorderRadius corners, each with its radius as `x`
local radius = BorderRadius.all(Val.px(3))
assert(radius.top_left.x[1] == 3 and radius.bottom_right.x[1] == 3, "border radius all")
local corners = BorderRadius.px(1, 2, 3, 4)
assert(corners.top_left.x[1] == 1 and corners.top_right.x[1] == 2, "border radius px top")
assert(corners.bottom_right.x[1] == 3 and corners.bottom_left.x[1] == 4, "border radius px bottom")
assert(BorderRadius.percent(1, 2, 3, 4).bottom_left.x:variant_name() == "Percent", "border radius percent")
assert(BorderRadius.zero().top_left.x[1] == 0, "border radius zero")

-- BorderColor sides
local border = BorderColor.all(Color.white())
assert(border.top:eq(Color.white()) and border.left:eq(Color.white()), "border color all")

-- Overflow on each axis
assert(Overflow.visible().x:variant_name() == "Visible", "visible")
assert(Overflow.clip().x:variant_name() == "Clip" and Overflow.clip().y:variant_name() == "Clip", "clip")
assert(Overflow.clip_x().x:variant_name() == "Clip" and Overflow.clip_x().y:variant_name() == "Visible", "clip_x")
assert(Overflow.clip_y().x:variant_name() == "Visible" and Overflow.clip_y().y:variant_name() == "Clip", "clip_y")
assert(Overflow.hidden().x:variant_name() == "Hidden" and Overflow.hidden().y:variant_name() == "Hidden", "hidden")
assert(Overflow.scroll().x:variant_name() == "Scroll" and Overflow.scroll().y:variant_name() == "Scroll", "scroll")

-- ComputedNode of a node that has not been laid out
local computed = ComputedNode.new({})
assert(computed:is_empty(), "a node with no size is empty")
assert(computed:size().x == 0 and computed:size().y == 0, "size")
assert(computed:content_size().x == 0, "content_size")
