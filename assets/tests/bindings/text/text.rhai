// Text, Text2d and TextSpan all hold a string
let Text = import_namespace.call("text").Text;
let Text2d = import_namespace.call("text").Text2d;
let TextSpan = import_namespace.call("text").TextSpan;
let text = Text.new_.call("hi");
assert(text.as_str.call() == "hi", "Text new");
text.set.call("bye");
assert(text.as_str.call() == "bye", "Text set");

let text2d = Text2d.new_.call("hi");
assert(text2d.as_str.call() == "hi", "Text2d new");
text2d.set.call("bye");
assert(text2d.as_str.call() == "bye", "Text2d set");

let span = TextSpan.new_.call("hi");
assert(span.as_str.call() == "hi", "TextSpan new");
span.set.call("bye");
assert(span.as_str.call() == "bye", "TextSpan set");
