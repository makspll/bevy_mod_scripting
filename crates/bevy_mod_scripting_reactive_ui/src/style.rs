//! The properties scripts set on elements, and how plain script values become `bevy_ui` values.
//!
//! Layout properties are the fields of [`Node`] under their own names, each taking a plain value:
//! a number of pixels or a string such as `"50%"` for sizes, the name of the variant for enums (`"space_between"`).

use std::sync::LazyLock;

use bevy_asset::AssetServer;
use bevy_color::{Color, Srgba};
use bevy_ecs::{bundle::Bundle, name::Name, world::EntityWorldMut};
use bevy_mod_scripting_bindings::{InteropError, ScriptValue};
use bevy_platform::collections::HashMap;
use bevy_reflect::{
    FromReflect, TypeInfo, Typed,
    enums::{DynamicEnum, DynamicVariant},
};
use bevy_text::{FontSize, TextColor, TextFont};
use bevy_ui::{
    AlignContent, AlignItems, AlignSelf, BackgroundColor, BorderColor, BorderRadius, BoxSizing,
    Display, FlexDirection, FlexWrap, GridAutoFlow, InlineDirection, JustifyContent, JustifyItems,
    JustifySelf, Node, Overflow, OverflowAxis, PositionType, UiRect, Val, ZIndex,
    widget::{ImageNode, Text},
};

/// A property of an element.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Prop {
    /// A layout property, by its index in [`STYLE`]
    Style(u8),
    BackgroundColor,
    BorderColor,
    ZIndex,
    Text,
    FontSize,
    Color,
    Image,
    Name,
    /// The handler of an event
    On(UiEvent),
}

/// The events elements can handle, from `bevy_picking`'s pointer events.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UiEvent {
    Click,
    Press,
    Release,
    PointerEnter,
    PointerLeave,
    /// Both of the above, the handler is given whether the pointer is over the element
    Hover,
    Drag,
    Scroll,
}

impl UiEvent {
    pub(crate) const ALL: [(&'static str, UiEvent); 8] = [
        ("click", UiEvent::Click),
        ("press", UiEvent::Press),
        ("release", UiEvent::Release),
        ("pointer_enter", UiEvent::PointerEnter),
        ("pointer_leave", UiEvent::PointerLeave),
        ("hover", UiEvent::Hover),
        ("drag", UiEvent::Drag),
        ("scroll", UiEvent::Scroll),
    ];
}

/// A layout property: a field of [`Node`] and how it is set from a script value, `Unit` resets it.
struct StyleField {
    name: &'static str,
    set: fn(&mut Node, &ScriptValue) -> Result<(), InteropError>,
    /// Reads it back as a script would set it
    #[cfg(feature = "testing")]
    get: fn(&Node) -> ScriptValue,
}

/// The fields of [`Node`] scripts can set, by their name.
macro_rules! style_fields {
    ($($field:ident),* $(,)?) => {
        &[$(StyleField {
            name: stringify!($field),
            set: |node, value| {
                node.$field = match value {
                    ScriptValue::Unit => Node::DEFAULT.$field,
                    value => FromProp::from_prop(value)?,
                };
                Ok(())
            },
            #[cfg(feature = "testing")]
            get: |node| testing::ToProp::to_prop(&node.$field),
        }),*]
    };
}

static STYLE: &[StyleField] = style_fields![
    display,
    box_sizing,
    position_type,
    overflow,
    left,
    right,
    top,
    bottom,
    width,
    height,
    min_width,
    min_height,
    max_width,
    max_height,
    aspect_ratio,
    align_items,
    justify_items,
    align_self,
    justify_self,
    align_content,
    justify_content,
    direction,
    margin,
    padding,
    border,
    border_radius,
    flex_direction,
    flex_wrap,
    flex_grow,
    flex_shrink,
    flex_basis,
    row_gap,
    column_gap,
    grid_auto_flow,
];

/// Every property by its name.
static PROPS: LazyLock<HashMap<&'static str, Prop>> = LazyLock::new(|| {
    let mut props: HashMap<&'static str, Prop> = STYLE
        .iter()
        .enumerate()
        .map(|(i, field)| (field.name, Prop::Style(i as u8)))
        .collect();
    props.extend([
        ("background_color", Prop::BackgroundColor),
        ("border_color", Prop::BorderColor),
        ("z_index", Prop::ZIndex),
        ("text", Prop::Text),
        ("font_size", Prop::FontSize),
        ("color", Prop::Color),
        ("image", Prop::Image),
        ("name", Prop::Name),
    ]);
    props
});

impl Prop {
    pub fn parse(name: &str) -> Option<Prop> {
        if let Some(event) = name.strip_prefix("on_") {
            return UiEvent::ALL
                .iter()
                .find(|(event_name, _)| *event_name == event)
                .map(|(_, event)| Prop::On(*event));
        }
        PROPS.get(name).copied()
    }
}

/// A `bevy_ui` value made from a plain script value.
trait FromProp: Sized {
    fn from_prop(value: &ScriptValue) -> Result<Self, InteropError>;
}

fn as_f32(value: &ScriptValue) -> Option<f32> {
    match value {
        ScriptValue::Integer(i) => Some(*i as f32),
        ScriptValue::Float(f) => Some(*f as f32),
        _ => None,
    }
}

impl FromProp for f32 {
    fn from_prop(value: &ScriptValue) -> Result<Self, InteropError> {
        as_f32(value)
            .ok_or_else(|| InteropError::string(format!("expected a number, got: {value:?}")))
    }
}

impl FromProp for Option<f32> {
    fn from_prop(value: &ScriptValue) -> Result<Self, InteropError> {
        f32::from_prop(value).map(Some)
    }
}

/// A number of pixels, or a string: `"auto"`, `"50%"`, `"10px"`, `"5vw"`, `"5vh"`, `"5vmin"`, `"5vmax"`.
impl FromProp for Val {
    fn from_prop(value: &ScriptValue) -> Result<Self, InteropError> {
        if let Some(px) = as_f32(value) {
            return Ok(Val::Px(px));
        }
        let invalid = || {
            InteropError::string(format!(
                "invalid size {value:?}, use a number of pixels, \"50%\", \"10px\", \"5vw\", \"5vh\" or \"auto\""
            ))
        };
        let ScriptValue::String(text) = value else {
            return Err(invalid());
        };
        let text = text.trim();
        if text == "auto" {
            return Ok(Val::Auto);
        }
        let units: [(&str, fn(f32) -> Val); 6] = [
            ("%", Val::Percent),
            ("px", Val::Px),
            ("vw", Val::Vw),
            ("vh", Val::Vh),
            ("vmin", Val::VMin),
            ("vmax", Val::VMax),
        ];
        units
            .iter()
            .find_map(|(unit, make)| {
                let number = text.strip_suffix(unit)?.trim().parse::<f32>().ok()?;
                Some(make(number))
            })
            .ok_or_else(invalid)
    }
}

/// One size for every side, a list `{ vertical, horizontal }` or `{ top, right, bottom, left }`,
/// or a table with any of `left`, `right`, `top`, `bottom`.
impl FromProp for UiRect {
    fn from_prop(value: &ScriptValue) -> Result<Self, InteropError> {
        match value {
            ScriptValue::List(list) => {
                let sides = list
                    .iter()
                    .map(Val::from_prop)
                    .collect::<Result<Vec<_>, _>>()?;
                match sides[..] {
                    [all] => Ok(UiRect::all(all)),
                    [vertical, horizontal] => Ok(UiRect::axes(horizontal, vertical)),
                    [top, right, bottom, left] => Ok(UiRect {
                        left,
                        right,
                        top,
                        bottom,
                    }),
                    _ => Err(InteropError::str(
                        "sides are one size, { vertical, horizontal } or { top, right, bottom, left }",
                    )),
                }
            }
            ScriptValue::Map(map) => {
                let mut rect = UiRect::DEFAULT;
                for (side, value) in map {
                    let value = Val::from_prop(value)?;
                    match side.as_str() {
                        "left" => rect.left = value,
                        "right" => rect.right = value,
                        "top" => rect.top = value,
                        "bottom" => rect.bottom = value,
                        other => {
                            return Err(InteropError::string(format!(
                                "unknown side `{other}`, use left, right, top or bottom"
                            )));
                        }
                    }
                }
                Ok(rect)
            }
            value => Val::from_prop(value).map(UiRect::all),
        }
    }
}

/// One radius for every corner, or a list `{ top_left, top_right, bottom_right, bottom_left }`.
impl FromProp for BorderRadius {
    fn from_prop(value: &ScriptValue) -> Result<Self, InteropError> {
        match value {
            ScriptValue::List(list) => {
                let corners = list
                    .iter()
                    .map(Val::from_prop)
                    .collect::<Result<Vec<_>, _>>()?;
                match corners[..] {
                    [top_left, top_right, bottom_right, bottom_left] => Ok(BorderRadius::new(
                        top_left,
                        top_right,
                        bottom_right,
                        bottom_left,
                    )),
                    _ => Err(InteropError::str(
                        "corners are one radius or { top_left, top_right, bottom_right, bottom_left }",
                    )),
                }
            }
            value => Val::from_prop(value).map(BorderRadius::all),
        }
    }
}

/// The variant of an enum by its name, written in any case with or without underscores: `"space_between"`.
fn enum_from_name<T: FromReflect + Typed>(value: &ScriptValue) -> Result<T, InteropError> {
    let normalize = |name: &str| name.replace('_', "").to_lowercase();
    let TypeInfo::Enum(info) = T::type_info() else {
        return Err(InteropError::str("not an enum"));
    };
    let invalid = || {
        let names = info
            .iter()
            .map(|v| format!("\"{}\"", snake_case(v.name())))
            .collect::<Vec<_>>();
        InteropError::string(format!(
            "expected one of {}, got: {value:?}",
            names.join(", ")
        ))
    };
    let ScriptValue::String(name) = value else {
        return Err(invalid());
    };
    let wanted = normalize(name);
    let variant = info
        .iter()
        .find(|v| normalize(v.name()) == wanted)
        .ok_or_else(invalid)?;
    T::from_reflect(&DynamicEnum::new(variant.name(), DynamicVariant::Unit)).ok_or_else(invalid)
}

fn snake_case(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.extend(c.to_lowercase());
    }
    out
}

macro_rules! enum_props {
    ($($ty:ty),* $(,)?) => {
        $(impl FromProp for $ty {
            fn from_prop(value: &ScriptValue) -> Result<Self, InteropError> {
                enum_from_name(value)
            }
        })*
    };
}

enum_props!(
    BoxSizing,
    PositionType,
    AlignItems,
    JustifyItems,
    AlignSelf,
    JustifySelf,
    AlignContent,
    JustifyContent,
    InlineDirection,
    FlexDirection,
    FlexWrap,
    GridAutoFlow,
    OverflowAxis,
);

/// The name of the variant, `false` is `"none"`.
impl FromProp for Display {
    fn from_prop(value: &ScriptValue) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Bool(false) => Ok(Display::None),
            ScriptValue::Bool(true) => Ok(Display::DEFAULT),
            value => enum_from_name(value),
        }
    }
}

/// One behaviour for both axes (`"visible"`, `"clip"`, `"hidden"`, `"scroll"`), or a table with `x` and `y`.
impl FromProp for Overflow {
    fn from_prop(value: &ScriptValue) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Map(map) => {
                let mut overflow = Overflow::DEFAULT;
                for (axis, value) in map {
                    let value = OverflowAxis::from_prop(value)?;
                    match axis.as_str() {
                        "x" => overflow.x = value,
                        "y" => overflow.y = value,
                        other => {
                            return Err(InteropError::string(format!(
                                "unknown axis `{other}`, use x or y"
                            )));
                        }
                    }
                }
                Ok(overflow)
            }
            value => {
                let axis = OverflowAxis::from_prop(value)?;
                Ok(Overflow { x: axis, y: axis })
            }
        }
    }
}

/// A hex colour (`"#ffcc00"`, `"#ffcc0080"`), or a list of red, green, blue and optionally alpha from 0 to 1.
fn color(value: &ScriptValue) -> Result<Color, InteropError> {
    match value {
        ScriptValue::String(hex) => Srgba::hex(hex.as_ref())
            .map(Color::Srgba)
            .map_err(|e| InteropError::string(format!("invalid colour `{hex}`: {e}"))),
        ScriptValue::List(list) => {
            let parts = list
                .iter()
                .map(f32::from_prop)
                .collect::<Result<Vec<_>, _>>()?;
            match parts[..] {
                [r, g, b] => Ok(Color::srgb(r, g, b)),
                [r, g, b, a] => Ok(Color::srgba(r, g, b, a)),
                _ => Err(InteropError::str(
                    "colours are { red, green, blue } or { red, green, blue, alpha }",
                )),
            }
        }
        other => Err(InteropError::string(format!(
            "expected a hex colour or a list of red, green and blue, got: {other:?}"
        ))),
    }
}

pub(crate) fn as_text(value: &ScriptValue) -> String {
    match value {
        ScriptValue::String(s) => s.to_string(),
        ScriptValue::Integer(i) => i.to_string(),
        ScriptValue::Float(f) => f.to_string(),
        ScriptValue::Bool(b) => b.to_string(),
        ScriptValue::Unit => String::new(),
        other => format!("{other:?}"),
    }
}

/// Where properties are written: a mounted entity, or [`UiParts`] before spawning one.
pub(crate) trait PropTarget {
    fn node(&mut self) -> Result<&mut Node, InteropError>;
    fn set_text(&mut self, text: String) -> Result<(), InteropError>;
    fn text_font(&mut self) -> &mut TextFont;
    fn set_bg(&mut self, bg: BackgroundColor);
    fn set_name(&mut self, name: Name);
    fn set_image(&mut self, path: String);
    /// Sets components which few elements have
    fn insert(&mut self, bundle: impl Bundle);
}

impl PropTarget for EntityWorldMut<'_> {
    fn node(&mut self) -> Result<&mut Node, InteropError> {
        self.get_mut::<Node>()
            .map(bevy_ecs::world::Mut::into_inner)
            .ok_or_else(|| InteropError::str("ui entity has no Node"))
    }

    fn set_text(&mut self, new: String) -> Result<(), InteropError> {
        let mut text = self
            .get_mut::<Text>()
            .ok_or_else(|| InteropError::str("`text` can only be set on text elements"))?;
        // only mark it changed when it did, text layout is expensive
        if text.0 != new {
            text.0 = new;
        }
        Ok(())
    }

    fn text_font(&mut self) -> &mut TextFont {
        if !self.contains::<TextFont>() {
            self.insert(TextFont::default());
        }
        self.get_mut::<TextFont>()
            .map(bevy_ecs::world::Mut::into_inner)
            .unwrap_or_else(|| unreachable!("inserted above"))
    }

    fn set_bg(&mut self, bg: BackgroundColor) {
        self.insert(bg);
    }

    fn set_name(&mut self, name: Name) {
        self.insert(name);
    }

    fn set_image(&mut self, path: String) {
        // images come from bevy's `ImagePlugin`, re-initialising them would wipe every loaded image
        let world = self.world();
        if !world.contains_resource::<bevy_asset::Assets<bevy_image::Image>>() {
            bevy_log::warn!("`image = \"{path}\"` ignored: the app has no image assets");
            return;
        }
        let image = world.resource::<AssetServer>().load(path);
        self.insert(ImageNode::new(image));
    }

    fn insert(&mut self, bundle: impl Bundle) {
        EntityWorldMut::insert(self, bundle);
    }
}

/// The components of an element before it is spawned, so it is spawned with the common ones at once.
#[derive(Default)]
pub(crate) struct UiParts {
    pub node: Node,
    pub text: Option<Text>,
    pub bg: Option<BackgroundColor>,
    pub name: Option<Name>,
    pub font: Option<TextFont>,
    pub image: Option<String>,
    /// Components few elements have, inserted after spawning
    pub rest: Vec<Box<dyn FnOnce(&mut EntityWorldMut) + Send>>,
}

impl UiParts {
    /// Adds what the spawn bundle doesn't hold to the spawned entity.
    pub fn finish(self, entity: &mut EntityWorldMut) {
        if let Some(font) = self.font {
            entity.insert(font);
        }
        if let Some(image) = self.image {
            entity.set_image(image);
        }
        for insert in self.rest {
            insert(entity);
        }
    }

    /// Whether [`Self::finish`] has anything to do.
    pub fn needs_finish(&self) -> bool {
        self.font.is_some() || self.image.is_some() || !self.rest.is_empty()
    }
}

impl PropTarget for UiParts {
    fn node(&mut self) -> Result<&mut Node, InteropError> {
        Ok(&mut self.node)
    }

    fn set_text(&mut self, new: String) -> Result<(), InteropError> {
        match &mut self.text {
            Some(text) => {
                text.0 = new;
                Ok(())
            }
            None => Err(InteropError::str("`text` can only be set on text elements")),
        }
    }

    fn text_font(&mut self) -> &mut TextFont {
        self.font.get_or_insert_default()
    }

    fn set_bg(&mut self, bg: BackgroundColor) {
        self.bg = Some(bg);
    }

    fn set_name(&mut self, name: Name) {
        self.name = Some(name);
    }

    fn set_image(&mut self, path: String) {
        self.image = Some(path);
    }

    fn insert(&mut self, bundle: impl Bundle) {
        self.rest.push(Box::new(move |entity| {
            entity.insert(bundle);
        }));
    }
}

/// Writes a property value onto the target, `Unit` resets it.
pub(crate) fn apply_prop(
    target: &mut impl PropTarget,
    prop: Prop,
    value: &ScriptValue,
) -> Result<(), InteropError> {
    let unset = matches!(value, ScriptValue::Unit);
    let named = |e: InteropError, name: &str| InteropError::string(format!("`{name}`: {e:?}"));
    match prop {
        Prop::Style(index) => {
            let field = &STYLE[index as usize];
            (field.set)(target.node()?, value).map_err(|e| named(e, field.name))?;
        }
        Prop::BackgroundColor => {
            let color = if unset { Color::NONE } else { color(value)? };
            target.set_bg(BackgroundColor(color));
        }
        Prop::BorderColor => {
            let color = if unset { Color::NONE } else { color(value)? };
            target.insert(BorderColor::all(color));
        }
        Prop::ZIndex => {
            let index = if unset { 0.0 } else { f32::from_prop(value)? };
            target.insert(ZIndex(index as i32));
        }
        Prop::Text => target.set_text(as_text(value))?,
        Prop::FontSize => {
            target.text_font().font_size = if unset {
                TextFont::default().font_size
            } else {
                FontSize::Px(f32::from_prop(value)?)
            };
        }
        Prop::Color => {
            let color = if unset {
                TextColor::default().0
            } else {
                color(value)?
            };
            target.insert(TextColor(color));
        }
        Prop::Image => {
            let ScriptValue::String(path) = value else {
                return Err(InteropError::string(format!(
                    "`image` is the path of an image asset, got: {value:?}"
                )));
            };
            target.set_image(path.to_string());
        }
        Prop::Name => target.set_name(Name::new(as_text(value))),
        Prop::On(_) => {}
    }
    Ok(())
}

/// Reading properties back from entities, as scripts set them, for tests.
#[cfg(feature = "testing")]
pub(crate) mod testing {
    use bevy_ecs::world::{EntityRef, World};
    use bevy_mod_scripting_bindings::{InteropError, ScriptValue};
    use bevy_reflect::{Reflect, ReflectRef};

    use super::*;

    /// A `bevy_ui` value as a script sets it: the inverse of `FromProp`, in one canonical form.
    pub(crate) trait ToProp {
        fn to_prop(&self) -> ScriptValue;
    }

    fn text(text: String) -> ScriptValue {
        ScriptValue::String(text.into())
    }

    impl ToProp for f32 {
        fn to_prop(&self) -> ScriptValue {
            ScriptValue::Float(*self as f64)
        }
    }

    impl ToProp for Option<f32> {
        fn to_prop(&self) -> ScriptValue {
            self.map_or(ScriptValue::Unit, |v| v.to_prop())
        }
    }

    /// Pixels are numbers, the rest strings: `"auto"`, `"50%"`, `"5vw"`.
    impl ToProp for Val {
        fn to_prop(&self) -> ScriptValue {
            match *self {
                Val::Auto => text("auto".into()),
                Val::Px(px) => px.to_prop(),
                Val::Percent(v) => text(format!("{v}%")),
                Val::Vw(v) => text(format!("{v}vw")),
                Val::Vh(v) => text(format!("{v}vh")),
                Val::VMin(v) => text(format!("{v}vmin")),
                Val::VMax(v) => text(format!("{v}vmax")),
                Val::Em(v) => text(format!("{v}em")),
                Val::Rem(v) => text(format!("{v}rem")),
            }
        }
    }

    /// One size when every side is the same, `{ vertical, horizontal }` when opposite sides are, else the sides by name.
    impl ToProp for UiRect {
        fn to_prop(&self) -> ScriptValue {
            let UiRect {
                left,
                right,
                top,
                bottom,
            } = *self;
            if left == right && left == top && left == bottom {
                return left.to_prop();
            }
            if left == right && top == bottom {
                return ScriptValue::List([top.to_prop(), left.to_prop()].into());
            }
            ScriptValue::Map(
                [
                    ("left", left),
                    ("right", right),
                    ("top", top),
                    ("bottom", bottom),
                ]
                .into_iter()
                .map(|(side, v)| (side.to_owned(), v.to_prop()))
                .collect(),
            )
        }
    }

    /// One size when it is round (an `auto` height is the width), else its width and height.
    impl ToProp for bevy_ui::CornerRadius {
        fn to_prop(&self) -> ScriptValue {
            if self.x == self.y || self.y == Val::Auto {
                return self.x.to_prop();
            }
            ScriptValue::List([self.x, self.y].iter().map(ToProp::to_prop).collect())
        }
    }

    /// One radius when every corner is the same, else the list of corners.
    impl ToProp for BorderRadius {
        fn to_prop(&self) -> ScriptValue {
            let corners = [
                self.top_left,
                self.top_right,
                self.bottom_right,
                self.bottom_left,
            ];
            if corners.iter().all(|c| *c == corners[0]) {
                return corners[0].to_prop();
            }
            ScriptValue::List(corners.iter().map(ToProp::to_prop).collect())
        }
    }

    /// The name of the variant, in snake case.
    fn variant_name(value: &dyn Reflect) -> ScriptValue {
        match value.reflect_ref() {
            ReflectRef::Enum(e) => text(snake_case(e.variant_name())),
            _ => ScriptValue::Unit,
        }
    }

    macro_rules! enum_to_props {
        ($($ty:ty),* $(,)?) => {
            $(impl ToProp for $ty {
                fn to_prop(&self) -> ScriptValue {
                    variant_name(self)
                }
            })*
        };
    }

    enum_to_props!(
        Display,
        BoxSizing,
        PositionType,
        AlignItems,
        JustifyItems,
        AlignSelf,
        JustifySelf,
        AlignContent,
        JustifyContent,
        InlineDirection,
        FlexDirection,
        FlexWrap,
        GridAutoFlow,
        OverflowAxis,
    );

    /// One behaviour when both axes have it, else `x` and `y`.
    impl ToProp for Overflow {
        fn to_prop(&self) -> ScriptValue {
            if self.x == self.y {
                return self.x.to_prop();
            }
            ScriptValue::Map(
                [("x", self.x), ("y", self.y)]
                    .into_iter()
                    .map(|(axis, v)| (axis.to_owned(), v.to_prop()))
                    .collect(),
            )
        }
    }

    fn hex(color: Color) -> ScriptValue {
        text(color.to_srgba().to_hex())
    }

    /// The properties which can be read back, by name: all but handlers.
    pub(crate) fn readable() -> impl Iterator<Item = (&'static str, Prop)> {
        PROPS
            .iter()
            .map(|(name, prop)| (*name, *prop))
            .filter(|(_, prop)| !matches!(prop, Prop::On(_)))
    }

    /// The value of a property of an entity as a script sets it, nil when the entity doesn't have it.
    pub(crate) fn read_prop(entity: EntityRef, prop: Prop) -> ScriptValue {
        let unit = ScriptValue::Unit;
        match prop {
            Prop::Style(index) => entity
                .get::<Node>()
                .map_or(unit, |node| (STYLE[index as usize].get)(node)),
            Prop::BackgroundColor => entity.get::<BackgroundColor>().map_or(unit, |c| hex(c.0)),
            Prop::BorderColor => entity.get::<BorderColor>().map_or(unit, |c| hex(c.top)),
            Prop::ZIndex => entity
                .get::<ZIndex>()
                .map_or(unit, |z| ScriptValue::Integer(z.0 as i64)),
            Prop::Text => entity.get::<Text>().map_or(unit, |t| text(t.0.clone())),
            Prop::FontSize => entity
                .get::<TextFont>()
                .map_or(unit, |f| match f.font_size {
                    FontSize::Px(px) => px.to_prop(),
                    other => text(format!("{other:?}")),
                }),
            Prop::Color => entity.get::<TextColor>().map_or(unit, |c| hex(c.0)),
            Prop::Image => entity
                .get::<ImageNode>()
                .and_then(|image| image.image.path())
                .map_or(unit, |path| text(path.to_string())),
            Prop::Name => entity
                .get::<Name>()
                .map_or(unit, |n| text(n.as_str().to_owned())),
            Prop::On(_) => unit,
        }
    }

    /// What each property reads back as when it is not set (set to nil), on an entity of a world of its own.
    pub(crate) fn defaults() -> HashMap<Prop, ScriptValue> {
        let mut world = World::new();
        let mut entity = world.spawn((Node::default(), Text::default()));
        readable()
            .filter(|(_, prop)| *prop != Prop::Image)
            .filter_map(|(_, prop)| {
                apply_prop(&mut entity, prop, &ScriptValue::Unit).ok()?;
                Some((prop, read_prop(entity.as_readonly(), prop)))
            })
            .collect()
    }

    /// A property value in the form [`read_prop`] gives it, by setting it on an entity of a world of its own.
    pub(crate) fn normalize_prop(
        prop: Prop,
        value: &ScriptValue,
    ) -> Result<ScriptValue, InteropError> {
        if let (Prop::Image, ScriptValue::String(path)) = (prop, value) {
            // loading images needs an asset server, the path is what is compared
            return Ok(text(path.to_string()));
        }
        let mut world = World::new();
        let mut entity = world.spawn((Node::default(), Text::default()));
        apply_prop(&mut entity, prop, value)?;
        Ok(read_prop(entity.as_readonly(), prop))
    }
}
