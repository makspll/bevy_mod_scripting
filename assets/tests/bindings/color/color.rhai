let Color = import_namespace.call("color").Color;
let Hsla = import_namespace.call("color").Hsla;
let Hsva = import_namespace.call("color").Hsva;
let Hwba = import_namespace.call("color").Hwba;
let Laba = import_namespace.call("color").Laba;
let Lcha = import_namespace.call("color").Lcha;
let LinearRgba = import_namespace.call("color").LinearRgba;
let Oklaba = import_namespace.call("color").Oklaba;
let Oklcha = import_namespace.call("color").Oklcha;
let Srgba = import_namespace.call("color").Srgba;
let Xyza = import_namespace.call("color").Xyza;

fn approx(a, b) {
    (a - b).abs() < 0.0001
}

// checks the sRGB channels of a color
fn check_rgba(c, r, g, b, a, msg) {
    let s = c.to_srgba.call();
    assert(approx(s.red, r) && approx(s.green, g) && approx(s.blue, b) && approx(s.alpha, a), msg);
}

// Color constructors
check_rgba(Color.srgb.call(1.0, 0.5, 0.0), 1.0, 0.5, 0.0, 1.0, "srgb");
check_rgba(Color.srgba.call(1.0, 0.5, 0.0, 0.25), 1.0, 0.5, 0.0, 0.25, "srgba");
check_rgba(Color.srgb_u8.call(255, 0, 128), 1.0, 0.0, 128.0 / 255.0, 1.0, "srgb_u8");
check_rgba(Color.srgba_u8.call(255, 0, 0, 128), 1.0, 0.0, 0.0, 128.0 / 255.0, "srgba_u8");
check_rgba(Color.linear_rgb.call(1.0, 0.0, 0.0), 1.0, 0.0, 0.0, 1.0, "linear_rgb");
check_rgba(Color.linear_rgba.call(1.0, 0.0, 0.0, 0.5), 1.0, 0.0, 0.0, 0.5, "linear_rgba");
check_rgba(Color.hsl.call(0.0, 1.0, 0.5), 1.0, 0.0, 0.0, 1.0, "hsl");
check_rgba(Color.hsla.call(240.0, 1.0, 0.5, 0.5), 0.0, 0.0, 1.0, 0.5, "hsla");
check_rgba(Color.hsv.call(120.0, 1.0, 1.0), 0.0, 1.0, 0.0, 1.0, "hsv");
check_rgba(Color.hsva.call(0.0, 1.0, 1.0, 0.5), 1.0, 0.0, 0.0, 0.5, "hsva");
check_rgba(Color.white.call(), 1.0, 1.0, 1.0, 1.0, "white");
check_rgba(Color.black.call(), 0.0, 0.0, 0.0, 1.0, "black");
check_rgba(Color.none.call(), 0.0, 0.0, 0.0, 0.0, "none");

// hex and named colors
check_rgba(Color.hex.call("#ff0000"), 1.0, 0.0, 0.0, 1.0, "hex");
check_rgba(Color.hex.call("#ff000080"), 1.0, 0.0, 0.0, 128.0 / 255.0, "hex with alpha");
check_rgba(Color.named.call("rebecca_purple"), 102.0 / 255.0, 51.0 / 255.0, 153.0 / 255.0, 1.0, "named");
check_rgba(Color.named.call("REBECCA_PURPLE"), 102.0 / 255.0, 51.0 / 255.0, 153.0 / 255.0, 1.0, "named is case-insensitive");
assert(type_of(Color.named.call("not_a_color")) == "()", "named is nil for unknown names");

// alpha
let c = Color.srgb.call(1.0, 0.0, 0.0);
assert(approx(c.alpha.call(), 1.0), "alpha");
check_rgba(c.with_alpha.call(0.5), 1.0, 0.0, 0.0, 0.5, "with_alpha");
assert(approx(c.alpha.call(), 1.0), "with_alpha leaves the original alone");
let m = Color.white.call();
m.set_alpha.call(0.25);
assert(approx(m.alpha.call(), 0.25), "set_alpha");
assert(Color.none.call().is_fully_transparent.call(), "is_fully_transparent");
assert(!Color.white.call().is_fully_transparent.call(), "is_fully_transparent for opaque colors");

// lighter, darker and mix
let grey = Color.srgb.call(0.5, 0.5, 0.5);
assert(grey.lighter.call(0.2).luminance.call() > grey.luminance.call(), "lighter");
assert(grey.darker.call(0.2).luminance.call() < grey.luminance.call(), "darker");
let black = Color.black.call();
let white = Color.white.call();
check_rgba(black.mix.call(white, 0.0), 0.0, 0.0, 0.0, 1.0, "mix at 0");
check_rgba(black.mix.call(white, 1.0), 1.0, 1.0, 1.0, 1.0, "mix at 1");
check_rgba(black.mix.call(white, 0.5), 0.5, 0.5, 0.5, 1.0, "mix at 0.5");
assert(white.luminance.call() > black.luminance.call(), "luminance");

// conversions
let lin = Color.srgb.call(1.0, 0.0, 0.0).to_linear.call();
assert(approx(lin.red, 1.0) && approx(lin.green, 0.0), "to_linear");
assert(Color.srgb_u8.call(255, 0, 0).to_hex.call() == "#FF0000", "to_hex");
assert(Color.srgba_u8.call(255, 0, 0, 128).to_hex.call() == "#FF000080", "to_hex with alpha");
assert(Color.white.call().eq.call(Color.srgb.call(1.0, 1.0, 1.0)), "eq");
assert(!Color.white.call().eq.call(Color.black.call()), "not eq");

// Srgba
let s = Srgba.new_.call(1.0, 0.0, 0.0, 0.5);
assert(approx(s.red, 1.0) && approx(s.alpha, 0.5), "srgba new");
assert(Srgba.rgb.call(0.0, 1.0, 0.0).green == 1.0, "srgba rgb");
assert(Srgba.hex.call("#00ff00").to_hex.call() == "#00FF00", "srgba hex");
assert(Srgba.rgb.call(1.0, 0.0, 0.0).to_hex.call() == "#FF0000", "srgba to_hex");
assert(s.with_alpha.call(1.0).alpha == 1.0, "srgba with_alpha");
check_rgba(s.to_color.call(), 1.0, 0.0, 0.0, 0.5, "srgba to_color");
assert(s.eq.call(Srgba.new_.call(1.0, 0.0, 0.0, 0.5)), "srgba eq");

// LinearRgba
let l = LinearRgba.new_.call(0.25, 0.5, 1.0, 0.5);
assert(approx(l.red, 0.25) && approx(l.alpha, 0.5), "linear_rgba new");
assert(LinearRgba.rgb.call(0.0, 0.0, 1.0).blue == 1.0, "linear_rgba rgb");
let e = l.mul.call(2.0);
assert(approx(e.red, 0.5) && approx(e.green, 1.0) && approx(e.blue, 2.0) && approx(e.alpha, 0.5), "linear_rgba mul");
assert(approx(l.to_color.call().alpha.call(), 0.5), "linear_rgba to_color");
assert(l.eq.call(LinearRgba.new_.call(0.25, 0.5, 1.0, 0.5)), "linear_rgba eq");

// other color spaces convert to Color
assert(Hsla.new_.call(120.0, 1.0, 0.5, 0.5).to_color.call().eq.call(Color.hsla.call(120.0, 1.0, 0.5, 0.5)), "hsla to_color");
assert(Hsva.new_.call(120.0, 1.0, 1.0, 0.5).to_color.call().eq.call(Color.hsva.call(120.0, 1.0, 1.0, 0.5)), "hsva to_color");
assert(approx(Hwba.new_.call(0.0, 0.0, 0.0, 0.5).to_color.call().alpha.call(), 0.5), "hwba to_color");
assert(approx(Laba.new_.call(50.0, 0.0, 0.0, 0.5).to_color.call().alpha.call(), 0.5), "laba to_color");
assert(approx(Lcha.new_.call(50.0, 0.0, 0.0, 0.5).to_color.call().alpha.call(), 0.5), "lcha to_color");
assert(approx(Oklaba.new_.call(0.5, 0.0, 0.0, 0.5).to_color.call().alpha.call(), 0.5), "oklaba to_color");
assert(approx(Oklcha.new_.call(0.5, 0.0, 0.0, 0.5).to_color.call().alpha.call(), 0.5), "oklcha to_color");
assert(approx(Xyza.new_.call(0.5, 0.5, 0.5, 0.5).to_color.call().alpha.call(), 0.5), "xyza to_color");
