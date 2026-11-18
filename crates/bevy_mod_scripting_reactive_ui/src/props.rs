//! The properties of elements as typed arguments ([`UiProps`]), and the types documenting their values.
//!
//! Values stay script values until they are applied (see [`crate::style`]), the types here only check the names
//! of the properties and document what each one takes.

use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use bevy_mod_scripting_bindings::{
    ArgMeta, ByName, CallScope, FromScript, GetTypeDependencies, InteropError, IntoScript, Lazy,
    ScriptValue, ThroughTypeInfo, TypedThrough, V, WithDeps, WorldGuard,
};
use bevy_mod_scripting_derive::ScriptData;
use bevy_reflect::{TypeRegistry, Typed};
use bevy_ui::{
    AlignContent, AlignItems, AlignSelf, BoxSizing, Display, FlexDirection, FlexWrap, GridAutoFlow,
    InlineDirection, JustifyContent, JustifyItems, JustifySelf, OverflowAxis, PositionType,
};

use crate::{
    element::{PropValue, UiElement},
    expr::UiValue,
    style::Prop,
};

/// A ui element, built by `node`, `text`, a component and so on. Scripts hold a handle to it.
pub type Element = Lazy<V<UiElement>>;

/// The children of an element: a list of elements.
pub type Children = Lazy<Vec<V<UiElement>>>;

/// A property value: a `T`, a value computed in Rust ([`UiValue`]), or a function returning a `T`, re-run when
/// what it reads changes. Functions can be given with their dependencies, see [`WithDeps`].
pub struct Reactive<T>(ScriptValue, PhantomData<fn() -> T>);

impl<T> FromScript for Reactive<T> {
    type This<'w> = Self;

    // only documents props, see `parse_props`
    fn from_script(value: ScriptValue, _: &CallScope) -> Result<Self, InteropError> {
        Ok(Reactive(value, PhantomData))
    }
}

impl<T> IntoScript for Reactive<T> {
    fn into_script(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(self.0)
    }
}

impl<T> ArgMeta for Reactive<T> {}

impl<T: TypedThrough> TypedThrough for Reactive<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Union(vec![
            T::through_type_info(),
            ThroughTypeInfo::Val(UiValue::type_info()),
            ThroughTypeInfo::Function {
                params: Vec::new(),
                returns: Box::new(T::through_type_info()),
            },
        ])
    }
}

impl<T: GetTypeDependencies> GetTypeDependencies for Reactive<T> {
    type Underlying = ScriptValue;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        T::register_type_dependencies(registry);
        registry.register::<UiValue>();
    }
}

/// A size: a number of pixels, or a string: `"auto"`, `"50%"`, `"10px"`, `"5vw"`, `"5vh"`, `"5vmin"`, `"5vmax"`.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum Size {
    Pixels(f64),
    Css(String),
}

/// The sizes of the sides of a rectangle, any of them can be left out.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub struct SideSizes {
    left: Option<Size>,
    right: Option<Size>,
    top: Option<Size>,
    bottom: Option<Size>,
}

/// One size for every side, a list `{ vertical, horizontal }` or `{ top, right, bottom, left }`, or the sides by name.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum Sides {
    All(Size),
    List(Vec<Size>),
    Named(SideSizes),
}

/// One radius for every corner, or a list `{ top_left, top_right, bottom_right, bottom_left }`.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum Corners {
    All(Size),
    List(Vec<Size>),
}

/// Whether the element is shown, or how it lays out its children.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum DisplayMode {
    Shown(bool),
    Named(ByName<Display>),
}

/// What overflows on each axis, any of them can be left out.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub struct OverflowAxes {
    x: Option<ByName<OverflowAxis>>,
    y: Option<ByName<OverflowAxis>>,
}

/// What overflows on both axes, or on each.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum OverflowMode {
    Both(ByName<OverflowAxis>),
    Axes(OverflowAxes),
}

/// A hex colour (`"#ffcc00"`, `"#ffcc0080"`), or a list of red, green, blue and optionally alpha from 0 to 1.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum ColorValue {
    Hex(String),
    Channels(Vec<f64>),
}

/// Text, numbers are written as they are.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum Label {
    Text(String),
    Number(f64),
}

/// A plain value, where a value computed in Rust can be given as well.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum Constant {
    Text(String),
    Number(f64),
    Flag(bool),
}

/// What tells an element apart from its siblings, entities compare by value.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub enum Key {
    Text(String),
    Number(f64),
    Entity(V<Entity>),
}

/// A pointer event, given to handlers. Pressing, releasing and dragging say which `button`, drags and scrolls by how much.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub struct PointerEvent {
    /// Where the pointer is, in logical pixels
    x: f64,
    y: f64,
    /// `"Primary"`, `"Secondary"` or `"Middle"`
    button: Option<String>,
    delta_x: Option<f64>,
    delta_y: Option<f64>,
}

/// A handler of a pointer event.
pub type Handler = WithDeps<fn(PointerEvent)>;

/// The properties of an element, all optional. Layout properties are the fields of `bevy_ui`'s `Node`.
///
/// Each takes a value, a value computed in Rust (`UiValue`) or a function re-run when what it reads changes.
///
/// It documents what elements take: they are given a [`Lazy<UiProps>`] parsed in one pass with [`parse_props`],
/// building only the properties given (converting a struct of all of them doubled the cost of building elements).
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub struct UiProps {
    /// Tells the element apart from its siblings, so it is kept when they change
    key: Option<Lazy<Key>>,
    display: Option<Reactive<DisplayMode>>,
    box_sizing: Option<Reactive<ByName<BoxSizing>>>,
    position_type: Option<Reactive<ByName<PositionType>>>,
    overflow: Option<Reactive<OverflowMode>>,
    left: Option<Reactive<Size>>,
    right: Option<Reactive<Size>>,
    top: Option<Reactive<Size>>,
    bottom: Option<Reactive<Size>>,
    width: Option<Reactive<Size>>,
    height: Option<Reactive<Size>>,
    min_width: Option<Reactive<Size>>,
    min_height: Option<Reactive<Size>>,
    max_width: Option<Reactive<Size>>,
    max_height: Option<Reactive<Size>>,
    aspect_ratio: Option<Reactive<f64>>,
    align_items: Option<Reactive<ByName<AlignItems>>>,
    justify_items: Option<Reactive<ByName<JustifyItems>>>,
    align_self: Option<Reactive<ByName<AlignSelf>>>,
    justify_self: Option<Reactive<ByName<JustifySelf>>>,
    align_content: Option<Reactive<ByName<AlignContent>>>,
    justify_content: Option<Reactive<ByName<JustifyContent>>>,
    direction: Option<Reactive<ByName<InlineDirection>>>,
    margin: Option<Reactive<Sides>>,
    padding: Option<Reactive<Sides>>,
    border: Option<Reactive<Sides>>,
    border_radius: Option<Reactive<Corners>>,
    flex_direction: Option<Reactive<ByName<FlexDirection>>>,
    flex_wrap: Option<Reactive<ByName<FlexWrap>>>,
    flex_grow: Option<Reactive<f64>>,
    flex_shrink: Option<Reactive<f64>>,
    flex_basis: Option<Reactive<Size>>,
    row_gap: Option<Reactive<Size>>,
    column_gap: Option<Reactive<Size>>,
    grid_auto_flow: Option<Reactive<ByName<GridAutoFlow>>>,
    background_color: Option<Reactive<ColorValue>>,
    border_color: Option<Reactive<ColorValue>>,
    z_index: Option<Reactive<f64>>,
    /// The text of a text element
    text: Option<Reactive<Label>>,
    font_size: Option<Reactive<f64>>,
    /// The colour of text
    color: Option<Reactive<ColorValue>>,
    /// The path of an image asset
    image: Option<Reactive<String>>,
    /// The `Name` of the entity, i.e. to find it in tests
    name: Option<Reactive<Label>>,
    on_click: Option<Handler>,
    on_press: Option<Handler>,
    on_release: Option<Handler>,
    on_pointer_enter: Option<Handler>,
    on_pointer_leave: Option<Handler>,
    /// Called with whether the pointer is over the element
    on_hover: Option<WithDeps<fn(bool)>>,
    on_drag: Option<Handler>,
    on_scroll: Option<Handler>,
}

/// The key of an element and its properties, sorted so that diffs walk them in the same order, from a table of
/// properties ([`UiProps`]) or the text of a text element.
pub(crate) fn parse_props(
    props: ScriptValue,
    world: &WorldGuard,
) -> Result<(Option<String>, Vec<(Prop, PropValue)>), InteropError> {
    let map = match props {
        ScriptValue::Unit => return Ok((None, Vec::new())),
        ScriptValue::String(text) => {
            return Ok((
                None,
                vec![(Prop::Text, PropValue::Static(ScriptValue::String(text)))],
            ));
        }
        ScriptValue::Map(map) => map,
        ScriptValue::List(list) if list.is_empty() => return Ok((None, Vec::new())),
        other => {
            return Err(InteropError::string(format!(
                "expected a table of properties, got: {other:?}"
            )));
        }
    };
    let mut key = None;
    let mut props = Vec::with_capacity(map.len());
    for (name, value) in map {
        if name == "key" {
            key = Some(crate::element::key_of(&value, world)?);
            continue;
        }
        let prop = Prop::parse(&name)
            .ok_or_else(|| InteropError::string(format!("unknown ui property `{name}`")))?;
        let handler = matches!(prop, Prop::On(_));
        let value = match value {
            // a function with its dependencies, see `WithDeps`
            ScriptValue::List(list) if matches!(list.front(), Some(ScriptValue::Function(_))) => {
                ScriptValue::Function(WithDeps::<fn()>::curry(list)?)
            }
            value => value,
        };
        let value = match value {
            ScriptValue::Function(f) if handler => PropValue::Handler(f),
            ScriptValue::Function(f) => PropValue::Bound(f),
            value if handler => {
                return Err(InteropError::string(format!(
                    "`{name}` must be a function, got: {value:?}"
                )));
            }
            ScriptValue::Reference(r) => match r.downcast::<UiValue>(world.clone()) {
                Ok(value) => PropValue::Expr(value.0),
                Err(_) => PropValue::Static(ScriptValue::Reference(r)),
            },
            value => PropValue::Static(value),
        };
        props.push((prop, value));
    }
    props.sort_by_key(|(p, _)| *p);
    Ok((key, props))
}
