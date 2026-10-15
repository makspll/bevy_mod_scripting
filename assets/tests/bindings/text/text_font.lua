-- TextFont constructors and builders
local Justify = import_namespace("text").Justify
local TextFont = import_namespace("text").TextFont
local TextLayout = import_namespace("text").TextLayout
local font = TextFont.from_font_size(20.0)
assert(font ~= nil, "from_font_size")
assert(font:with_font_size(30.0) ~= nil, "with_font_size")

-- TextLayout constructors
assert(TextLayout.new_with_no_wrap() ~= nil, "new_with_no_wrap")
assert(TextLayout.new_with_justify(Justify.Center) ~= nil, "new_with_justify")
