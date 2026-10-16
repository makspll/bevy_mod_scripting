local Color = import_namespace("color").Color
local Hsla = import_namespace("color").Hsla
local Hsva = import_namespace("color").Hsva
local Hwba = import_namespace("color").Hwba
local Laba = import_namespace("color").Laba
local Lcha = import_namespace("color").Lcha
local LinearRgba = import_namespace("color").LinearRgba
local Oklaba = import_namespace("color").Oklaba
local Oklcha = import_namespace("color").Oklcha
local Srgba = import_namespace("color").Srgba
local Xyza = import_namespace("color").Xyza
local function approx(a, b)
    return math.abs(a - b) < 0.0001
end

-- checks the sRGB channels of a color
local function check_rgba(c, r, g, b, a, msg)
    local s = c:to_srgba()
    assert(approx(s.red, r) and approx(s.green, g) and approx(s.blue, b) and approx(s.alpha, a), msg)
end

-- Color constructors
check_rgba(Color.srgb(1.0, 0.5, 0.0), 1.0, 0.5, 0.0, 1.0, "srgb")
check_rgba(Color.srgba(1.0, 0.5, 0.0, 0.25), 1.0, 0.5, 0.0, 0.25, "srgba")
check_rgba(Color.srgb_u8(255, 0, 128), 1.0, 0.0, 128 / 255, 1.0, "srgb_u8")
check_rgba(Color.srgba_u8(255, 0, 0, 128), 1.0, 0.0, 0.0, 128 / 255, "srgba_u8")
check_rgba(Color.linear_rgb(1.0, 0.0, 0.0), 1.0, 0.0, 0.0, 1.0, "linear_rgb")
check_rgba(Color.linear_rgba(1.0, 0.0, 0.0, 0.5), 1.0, 0.0, 0.0, 0.5, "linear_rgba")
check_rgba(Color.hsl(0.0, 1.0, 0.5), 1.0, 0.0, 0.0, 1.0, "hsl")
check_rgba(Color.hsla(240.0, 1.0, 0.5, 0.5), 0.0, 0.0, 1.0, 0.5, "hsla")
check_rgba(Color.hsv(120.0, 1.0, 1.0), 0.0, 1.0, 0.0, 1.0, "hsv")
check_rgba(Color.hsva(0.0, 1.0, 1.0, 0.5), 1.0, 0.0, 0.0, 0.5, "hsva")
check_rgba(Color.white(), 1.0, 1.0, 1.0, 1.0, "white")
check_rgba(Color.black(), 0.0, 0.0, 0.0, 1.0, "black")
check_rgba(Color.none(), 0.0, 0.0, 0.0, 0.0, "none")

-- hex and named colors
check_rgba(Color.hex("#ff0000"), 1.0, 0.0, 0.0, 1.0, "hex")
check_rgba(Color.hex("#ff000080"), 1.0, 0.0, 0.0, 128 / 255, "hex with alpha")
assert(not pcall(function() return Color.hex("not a color") end), "hex errors on invalid input")
check_rgba(Color.named("rebecca_purple"), 102 / 255, 51 / 255, 153 / 255, 1.0, "named")
check_rgba(Color.named("REBECCA_PURPLE"), 102 / 255, 51 / 255, 153 / 255, 1.0, "named is case-insensitive")
assert(Color.named("not_a_color") == nil, "named is nil for unknown names")

-- alpha
local c = Color.srgb(1.0, 0.0, 0.0)
assert(approx(c:alpha(), 1.0), "alpha")
check_rgba(c:with_alpha(0.5), 1.0, 0.0, 0.0, 0.5, "with_alpha")
assert(approx(c:alpha(), 1.0), "with_alpha leaves the original alone")
local m = Color.white()
m:set_alpha(0.25)
assert(approx(m:alpha(), 0.25), "set_alpha")
assert(Color.none():is_fully_transparent(), "is_fully_transparent")
assert(not Color.white():is_fully_transparent(), "is_fully_transparent for opaque colors")

-- lighter, darker and mix
local grey = Color.srgb(0.5, 0.5, 0.5)
assert(grey:lighter(0.2):luminance() > grey:luminance(), "lighter")
assert(grey:darker(0.2):luminance() < grey:luminance(), "darker")
local black, white = Color.black(), Color.white()
check_rgba(black:mix(white, 0.0), 0.0, 0.0, 0.0, 1.0, "mix at 0")
check_rgba(black:mix(white, 1.0), 1.0, 1.0, 1.0, 1.0, "mix at 1")
check_rgba(black:mix(white, 0.5), 0.5, 0.5, 0.5, 1.0, "mix at 0.5")
assert(white:luminance() > black:luminance(), "luminance")

-- conversions
local lin = Color.srgb(1.0, 0.0, 0.0):to_linear()
assert(approx(lin.red, 1.0) and approx(lin.green, 0.0), "to_linear")
assert(Color.srgb_u8(255, 0, 0):to_hex() == "#FF0000", "to_hex")
assert(Color.srgba_u8(255, 0, 0, 128):to_hex() == "#FF000080", "to_hex with alpha")
assert(Color.white():eq(Color.srgb(1.0, 1.0, 1.0)), "eq")
assert(not Color.white():eq(Color.black()), "not eq")

-- Srgba
local s = Srgba.new(1.0, 0.0, 0.0, 0.5)
assert(approx(s.red, 1.0) and approx(s.alpha, 0.5), "srgba new")
assert(Srgba.rgb(0.0, 1.0, 0.0).green == 1.0, "srgba rgb")
assert(Srgba.hex("#00ff00"):to_hex() == "#00FF00", "srgba hex")
assert(not pcall(function() return Srgba.hex("zz") end), "srgba hex errors on invalid input")
assert(Srgba.rgb(1.0, 0.0, 0.0):to_hex() == "#FF0000", "srgba to_hex")
assert(s:with_alpha(1.0).alpha == 1.0, "srgba with_alpha")
check_rgba(s:to_color(), 1.0, 0.0, 0.0, 0.5, "srgba to_color")
assert(s:eq(Srgba.new(1.0, 0.0, 0.0, 0.5)), "srgba eq")

-- LinearRgba
local l = LinearRgba.new(0.25, 0.5, 1.0, 0.5)
assert(approx(l.red, 0.25) and approx(l.alpha, 0.5), "linear_rgba new")
assert(LinearRgba.rgb(0.0, 0.0, 1.0).blue == 1.0, "linear_rgba rgb")
local e = l:mul(2.0)
assert(approx(e.red, 0.5) and approx(e.green, 1.0) and approx(e.blue, 2.0) and approx(e.alpha, 0.5), "linear_rgba mul")
assert(approx(l:to_color():alpha(), 0.5), "linear_rgba to_color")
assert(l:eq(LinearRgba.new(0.25, 0.5, 1.0, 0.5)), "linear_rgba eq")

-- other color spaces convert to Color
assert(Hsla.new(120.0, 1.0, 0.5, 0.5):to_color():eq(Color.hsla(120.0, 1.0, 0.5, 0.5)), "hsla to_color")
assert(Hsva.new(120.0, 1.0, 1.0, 0.5):to_color():eq(Color.hsva(120.0, 1.0, 1.0, 0.5)), "hsva to_color")
assert(approx(Hwba.new(0.0, 0.0, 0.0, 0.5):to_color():alpha(), 0.5), "hwba to_color")
assert(approx(Laba.new(50.0, 0.0, 0.0, 0.5):to_color():alpha(), 0.5), "laba to_color")
assert(approx(Lcha.new(50.0, 0.0, 0.0, 0.5):to_color():alpha(), 0.5), "lcha to_color")
assert(approx(Oklaba.new(0.5, 0.0, 0.0, 0.5):to_color():alpha(), 0.5), "oklaba to_color")
assert(approx(Oklcha.new(0.5, 0.0, 0.0, 0.5):to_color():alpha(), 0.5), "oklcha to_color")
assert(approx(Xyza.new(0.5, 0.5, 0.5, 0.5):to_color():alpha(), 0.5), "xyza to_color")
