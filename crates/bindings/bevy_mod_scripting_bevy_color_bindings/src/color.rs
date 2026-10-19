//! [`Color`] and the named CSS colors.

// `script_bindings` expects these in scope
use bevy_color::{
    Alpha, Color, LinearRgba, Luminance, Mix, Srgba,
    palettes::{basic, css},
};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{M, R, V},
};
use bevy_mod_scripting_derive::script_bindings;

/// Declares the named CSS colors: each [`css`] constant with its lowercase name.
macro_rules! css_colors {
    ($($c:ident => $name:literal),* $(,)?) => {
        /// Every named CSS color, by lowercase name.
        const NAMED_COLORS: &[(&str, Srgba)] = &[$(($name, css::$c)),*];
    };
}

css_colors!(
    ALICE_BLUE => "alice_blue",
    ANTIQUE_WHITE => "antique_white",
    AQUA => "aqua",
    AQUAMARINE => "aquamarine",
    AZURE => "azure",
    BEIGE => "beige",
    BISQUE => "bisque",
    BLANCHED_ALMOND => "blanched_almond",
    BLUE_VIOLET => "blue_violet",
    BROWN => "brown",
    BURLYWOOD => "burlywood",
    CADET_BLUE => "cadet_blue",
    CHARTREUSE => "chartreuse",
    CHOCOLATE => "chocolate",
    CORAL => "coral",
    CORNFLOWER_BLUE => "cornflower_blue",
    CORNSILK => "cornsilk",
    CRIMSON => "crimson",
    DARK_BLUE => "dark_blue",
    DARK_CYAN => "dark_cyan",
    DARK_GOLDENROD => "dark_goldenrod",
    DARK_GRAY => "dark_gray",
    DARK_GREEN => "dark_green",
    DARK_GREY => "dark_grey",
    DARK_KHAKI => "dark_khaki",
    DARK_MAGENTA => "dark_magenta",
    DARK_OLIVEGREEN => "dark_olivegreen",
    DARK_ORANGE => "dark_orange",
    DARK_ORCHID => "dark_orchid",
    DARK_RED => "dark_red",
    DARK_SALMON => "dark_salmon",
    DARK_SEA_GREEN => "dark_sea_green",
    DARK_SLATE_BLUE => "dark_slate_blue",
    DARK_SLATE_GRAY => "dark_slate_gray",
    DARK_SLATE_GREY => "dark_slate_grey",
    DARK_TURQUOISE => "dark_turquoise",
    DARK_VIOLET => "dark_violet",
    DEEP_PINK => "deep_pink",
    DEEP_SKY_BLUE => "deep_sky_blue",
    DIM_GRAY => "dim_gray",
    DIM_GREY => "dim_grey",
    DODGER_BLUE => "dodger_blue",
    FIRE_BRICK => "fire_brick",
    FLORAL_WHITE => "floral_white",
    FOREST_GREEN => "forest_green",
    GAINSBORO => "gainsboro",
    GHOST_WHITE => "ghost_white",
    GOLD => "gold",
    GOLDENROD => "goldenrod",
    GREEN_YELLOW => "green_yellow",
    GREY => "grey",
    HONEYDEW => "honeydew",
    HOT_PINK => "hot_pink",
    INDIAN_RED => "indian_red",
    INDIGO => "indigo",
    IVORY => "ivory",
    KHAKI => "khaki",
    LAVENDER => "lavender",
    LAVENDER_BLUSH => "lavender_blush",
    LAWN_GREEN => "lawn_green",
    LEMON_CHIFFON => "lemon_chiffon",
    LIGHT_BLUE => "light_blue",
    LIGHT_CORAL => "light_coral",
    LIGHT_CYAN => "light_cyan",
    LIGHT_GOLDENROD_YELLOW => "light_goldenrod_yellow",
    LIGHT_GRAY => "light_gray",
    LIGHT_GREEN => "light_green",
    LIGHT_GREY => "light_grey",
    LIGHT_PINK => "light_pink",
    LIGHT_SALMON => "light_salmon",
    LIGHT_SEA_GREEN => "light_sea_green",
    LIGHT_SKY_BLUE => "light_sky_blue",
    LIGHT_SLATE_GRAY => "light_slate_gray",
    LIGHT_SLATE_GREY => "light_slate_grey",
    LIGHT_STEEL_BLUE => "light_steel_blue",
    LIGHT_YELLOW => "light_yellow",
    LIMEGREEN => "limegreen",
    LINEN => "linen",
    MAGENTA => "magenta",
    MEDIUM_AQUAMARINE => "medium_aquamarine",
    MEDIUM_BLUE => "medium_blue",
    MEDIUM_ORCHID => "medium_orchid",
    MEDIUM_PURPLE => "medium_purple",
    MEDIUM_SEA_GREEN => "medium_sea_green",
    MEDIUM_SLATE_BLUE => "medium_slate_blue",
    MEDIUM_SPRING_GREEN => "medium_spring_green",
    MEDIUM_TURQUOISE => "medium_turquoise",
    MEDIUM_VIOLET_RED => "medium_violet_red",
    MIDNIGHT_BLUE => "midnight_blue",
    MINT_CREAM => "mint_cream",
    MISTY_ROSE => "misty_rose",
    MOCCASIN => "moccasin",
    NAVAJO_WHITE => "navajo_white",
    OLD_LACE => "old_lace",
    OLIVE_DRAB => "olive_drab",
    ORANGE => "orange",
    ORANGE_RED => "orange_red",
    ORCHID => "orchid",
    PALE_GOLDENROD => "pale_goldenrod",
    PALE_GREEN => "pale_green",
    PALE_TURQUOISE => "pale_turquoise",
    PALE_VIOLETRED => "pale_violetred",
    PAPAYA_WHIP => "papaya_whip",
    PEACHPUFF => "peachpuff",
    PERU => "peru",
    PINK => "pink",
    PLUM => "plum",
    POWDER_BLUE => "powder_blue",
    REBECCA_PURPLE => "rebecca_purple",
    ROSY_BROWN => "rosy_brown",
    ROYAL_BLUE => "royal_blue",
    SADDLE_BROWN => "saddle_brown",
    SALMON => "salmon",
    SANDY_BROWN => "sandy_brown",
    SEA_GREEN => "sea_green",
    SEASHELL => "seashell",
    SIENNA => "sienna",
    SKY_BLUE => "sky_blue",
    SLATE_BLUE => "slate_blue",
    SLATE_GRAY => "slate_gray",
    SLATE_GREY => "slate_grey",
    SNOW => "snow",
    SPRING_GREEN => "spring_green",
    STEEL_BLUE => "steel_blue",
    TAN => "tan",
    THISTLE => "thistle",
    TOMATO => "tomato",
    TURQUOISE => "turquoise",
    VIOLET => "violet",
    WHEAT => "wheat",
    WHITE_SMOKE => "white_smoke",
    YELLOW_GREEN => "yellow_green",
);

#[script_bindings(
    remote,
    name = "color_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "color",
    core
)]
impl Color {
    /// A color from red, green and blue in the sRGB space, each from `0` to `1`, fully opaque.
    ///
    /// ```multi-lang-example,lua
    /// local Color = import_namespace("color").Color
    /// local red = Color.srgb(1.0, 0.0, 0.0)
    /// assert(red:to_srgba().red == 1.0)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let Color = import_namespace("color").Color;
    /// let red = Color.srgb(1.0, 0.0, 0.0);
    /// assert!(red.to_srgba().red == 1.0);
    /// ```
    fn srgb(red: f32, green: f32, blue: f32) -> V<Color> {
        V(Color::srgb(red, green, blue))
    }

    /// A color from red, green, blue and alpha in the sRGB space, each from `0` to `1`.
    fn srgba(red: f32, green: f32, blue: f32, alpha: f32) -> V<Color> {
        V(Color::srgba(red, green, blue, alpha))
    }

    /// A color from red, green and blue in the sRGB space, each from `0` to `255`, fully opaque.
    fn srgb_u8(red: u8, green: u8, blue: u8) -> V<Color> {
        V(Color::srgb_u8(red, green, blue))
    }

    /// A color from red, green, blue and alpha in the sRGB space, each from `0` to `255`.
    fn srgba_u8(red: u8, green: u8, blue: u8, alpha: u8) -> V<Color> {
        V(Color::srgba_u8(red, green, blue, alpha))
    }

    /// A color from red, green and blue in the linear RGB space, each from `0` to `1`, fully opaque.
    fn linear_rgb(red: f32, green: f32, blue: f32) -> V<Color> {
        V(Color::linear_rgb(red, green, blue))
    }

    /// A color from red, green, blue and alpha in the linear RGB space, each from `0` to `1`.
    fn linear_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> V<Color> {
        V(Color::linear_rgba(red, green, blue, alpha))
    }

    /// A color from hue (in degrees), saturation and lightness, fully opaque.
    fn hsl(hue: f32, saturation: f32, lightness: f32) -> V<Color> {
        V(Color::hsl(hue, saturation, lightness))
    }

    /// A color from hue (in degrees), saturation, lightness and alpha.
    fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> V<Color> {
        V(Color::hsla(hue, saturation, lightness, alpha))
    }

    /// A color from hue (in degrees), saturation and value, fully opaque.
    fn hsv(hue: f32, saturation: f32, value: f32) -> V<Color> {
        V(Color::hsv(hue, saturation, value))
    }

    /// A color from hue (in degrees), saturation, value and alpha.
    fn hsva(hue: f32, saturation: f32, value: f32, alpha: f32) -> V<Color> {
        V(Color::hsva(hue, saturation, value, alpha))
    }

    /// Opaque white.
    fn white() -> V<Color> {
        V(Color::Srgba(basic::WHITE))
    }

    /// Opaque black.
    fn black() -> V<Color> {
        V(Color::Srgba(basic::BLACK))
    }

    /// A fully transparent color.
    fn none() -> V<Color> {
        V(Color::srgba(0.0, 0.0, 0.0, 0.0))
    }

    /// A color from a hex string such as `"#ff8800"` or `"#ff880080"`, or an error if the string is not valid hex.
    fn hex(hex: String) -> Result<V<Color>, InteropError> {
        Srgba::hex(&hex)
            .map(|c| V(Color::Srgba(c)))
            .map_err(|e| InteropError::string(format!("invalid hex color `{hex}`: {e}")))
    }

    /// A named CSS color such as `"rebecca_purple"` (case-insensitive), or nil if there is no color with that name.
    fn named(name: String) -> Option<V<Color>> {
        NAMED_COLORS
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(&name))
            .map(|(_, c)| V(Color::Srgba(*c)))
    }

    /// The alpha (opacity) of the color, from `0` (transparent) to `1` (opaque).
    fn alpha(_self: R<Color>) -> f32 {
        _self.alpha()
    }

    /// A copy of the color with its alpha set to `alpha`.
    fn with_alpha(_self: R<Color>, alpha: f32) -> V<Color> {
        V(_self.with_alpha(alpha))
    }

    /// Sets the alpha (opacity) of the color in place.
    fn set_alpha(mut _self: M<Color>, alpha: f32) {
        _self.0.set_alpha(alpha);
    }

    /// A lighter copy of the color, `amount` from `0` to `1`.
    fn lighter(_self: R<Color>, amount: f32) -> V<Color> {
        V(_self.lighter(amount))
    }

    /// A darker copy of the color, `amount` from `0` to `1`.
    fn darker(_self: R<Color>, amount: f32) -> V<Color> {
        V(_self.darker(amount))
    }

    /// A color between this one and `other`, `t = 0` gives this color and `t = 1` gives `other`.
    fn mix(_self: R<Color>, other: R<Color>, t: f32) -> V<Color> {
        V(_self.mix(&*other, t))
    }

    /// The color converted to the sRGB space.
    fn to_srgba(_self: R<Color>) -> V<Srgba> {
        V(_self.to_srgba())
    }

    /// The color converted to the linear RGB space.
    fn to_linear(_self: R<Color>) -> V<LinearRgba> {
        V(_self.to_linear())
    }

    /// The color as an uppercase hex string such as `"#FF8800"`, with alpha appended unless fully opaque.
    fn to_hex(_self: R<Color>) -> String {
        _self.to_srgba().to_hex()
    }

    /// The perceived brightness of the color, from `0` (black) to `1` (white).
    fn luminance(_self: R<Color>) -> f32 {
        _self.luminance()
    }

    /// Whether the color is completely transparent.
    fn is_fully_transparent(_self: R<Color>) -> bool {
        _self.is_fully_transparent()
    }

    /// Whether both colors are the same color in the same color space.
    fn eq(_self: R<Color>, other: R<Color>) -> bool {
        *_self == *other
    }
}

/// Registers the color functions.
pub(crate) fn register(world: &mut World) {
    register_color_functions(world);
}
