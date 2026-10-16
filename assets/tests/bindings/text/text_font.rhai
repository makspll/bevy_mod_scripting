// TextFont constructors and builders
let font = TextFont.from_font_size.call(20.0);
assert(type_of(font) != "()", "from_font_size");
assert(type_of(font.with_font_size.call(30.0)) != "()", "with_font_size");

// TextLayout constructors
assert(type_of(TextLayout.new_with_no_wrap.call()) != "()", "new_with_no_wrap");
assert(type_of(TextLayout.new_with_justify.call(Justify.Center)) != "()", "new_with_justify");
