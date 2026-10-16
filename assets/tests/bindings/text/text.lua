-- Text, Text2d and TextSpan all hold a string
local text = Text.new("hi")
assert(text:as_str() == "hi", "Text new")
text:set("bye")
assert(text:as_str() == "bye", "Text set")

local text2d = Text2d.new("hi")
assert(text2d:as_str() == "hi", "Text2d new")
text2d:set("bye")
assert(text2d:as_str() == "bye", "Text2d set")

local span = TextSpan.new("hi")
assert(span:as_str() == "hi", "TextSpan new")
span:set("bye")
assert(span:as_str() == "bye", "TextSpan set")
