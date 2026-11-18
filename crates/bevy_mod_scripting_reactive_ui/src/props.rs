//! The properties of elements as typed arguments ([`UiProps`]), and the types documenting their values.
//!
//! Values stay script values until they are applied (see [`crate::style`]), the types here only check the names
//! of the properties and document what each one takes.

use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use bevy_mod_scripting_bindings::{
    ArgMeta, ByName, CallScope, FromScript, GetTypeDependencies, InteropError, IntoScript,
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

/// A script value documented as a `T`, kept as it is: the UI converts it when it is applied.
pub struct As<T>(pub ScriptValue, PhantomData<fn() -> T>);

impl<T> As<T> {
    pub fn new(value: ScriptValue) -> Self {
        As(value, PhantomData)
    }
}

impl<T> FromScript for As<T> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, _: &CallScope) -> Result<Self, InteropError> {
        Ok(As(value, PhantomData))
    }
}

impl<T> IntoScript for As<T> {
    fn into_script(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(self.0)
    }
}

impl<T> ArgMeta for As<T> {}

impl<T: TypedThrough> TypedThrough for As<T> {
    fn through_type_info() -> ThroughTypeInfo {
        T::through_type_info()
    }
}

impl<T: GetTypeDependencies> GetTypeDependencies for As<T> {
    type Underlying = ScriptValue;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        T::register_type_dependencies(registry)
    }
}

/// A ui element, built by `node`, `text`, a component and so on. Scripts hold a handle to it.
pub type Element = As<V<UiElement>>;

/// The children of an element: a list of elements.
pub type Children = As<Vec<V<UiElement>>>;

/// A property value: a `T`, a value computed in Rust ([`UiValue`]), or a function returning a `T`, re-run when
/// what it reads changes. Functions can be given with their dependencies, see [`WithDeps`].
pub struct Reactive<T>(ScriptValue, PhantomData<fn() -> T>);

impl<T> FromScript for Reactive<T> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        let value = match value {
            ScriptValue::List(list) if matches!(list.front(), Some(ScriptValue::Function(_))) => {
                let f = WithDeps::<fn()>::from_script(ScriptValue::List(list), scope)?;
                ScriptValue::Function(f.into_inner())
            }
            value => value,
        };
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

/// How a property is stored on its element.
trait IntoPropValue {
    fn into_prop_value(self, world: &WorldGuard) -> PropValue;
}

impl<T> IntoPropValue for Reactive<T> {
    fn into_prop_value(self, world: &WorldGuard) -> PropValue {
        match self.0 {
            ScriptValue::Function(f) => PropValue::Bound(f),
            ScriptValue::Reference(r) => match r.downcast::<UiValue>(world.clone()) {
                Ok(value) => PropValue::Expr(value.0),
                Err(_) => PropValue::Static(ScriptValue::Reference(r)),
            },
            v => PropValue::Static(v),
        }
    }
}

impl<F> IntoPropValue for WithDeps<F> {
    fn into_prop_value(self, _: &WorldGuard) -> PropValue {
        PropValue::Handler(self.into_inner())
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

/// What an entity of the UI shows, for tests: the rest is described as Rust prints it.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub struct Description {
    pub text: Option<String>,
    pub name: Option<String>,
    /// Hex colours
    pub background_color: Option<String>,
    pub color: Option<String>,
    pub border_color: Option<String>,
    /// The `Node`, as Rust prints it
    pub layout: Option<String>,
    pub font_size: Option<String>,
    /// Whether it shows an image
    pub image: bool,
    /// How many children it has
    pub children: i64,
}

/// A handler of a pointer event.
pub type Handler = WithDeps<fn(PointerEvent)>;

macro_rules! ui_props {
    ($($(#[$doc:meta])* $field:ident: $ty:ty),* $(,)?) => {
        /// The properties of an element, all optional. Layout properties are the fields of `bevy_ui`'s `Node`.
        ///
        /// Each takes a value, a value computed in Rust (`UiValue`) or a function re-run when what it reads changes.
        #[derive(ScriptData)]
        #[script_data(namespace = "reactive_ui", bms_bindings_path = "bevy_mod_scripting_bindings")]
        pub struct UiProps {
            /// Tells the element apart from its siblings, so it is kept when they change
            key: Option<As<Key>>,
            $($(#[$doc])* $field: Option<$ty>,)*
        }

        impl UiProps {
            /// The key, and the properties sorted so that diffs walk them in the same order.
            pub(crate) fn into_props(
                self,
                world: &WorldGuard,
            ) -> Result<(Option<String>, Vec<(Prop, PropValue)>), InteropError> {
                let key = self
                    .key
                    .map(|key| crate::element::key_of(&key.0, world))
                    .transpose()?;
                let mut props = Vec::new();
                $(if let Some(value) = self.$field {
                    let prop = Prop::parse(stringify!($field)).ok_or_else(|| {
                        InteropError::str(concat!("unknown ui property `", stringify!($field), "`"))
                    })?;
                    props.push((prop, value.into_prop_value(world)));
                })*
                props.sort_by_key(|(p, _)| *p);
                Ok((key, props))
            }
        }
    };
}

ui_props! {
    display: Reactive<DisplayMode>,
    box_sizing: Reactive<ByName<BoxSizing>>,
    position_type: Reactive<ByName<PositionType>>,
    overflow: Reactive<OverflowMode>,
    left: Reactive<Size>,
    right: Reactive<Size>,
    top: Reactive<Size>,
    bottom: Reactive<Size>,
    width: Reactive<Size>,
    height: Reactive<Size>,
    min_width: Reactive<Size>,
    min_height: Reactive<Size>,
    max_width: Reactive<Size>,
    max_height: Reactive<Size>,
    aspect_ratio: Reactive<f64>,
    align_items: Reactive<ByName<AlignItems>>,
    justify_items: Reactive<ByName<JustifyItems>>,
    align_self: Reactive<ByName<AlignSelf>>,
    justify_self: Reactive<ByName<JustifySelf>>,
    align_content: Reactive<ByName<AlignContent>>,
    justify_content: Reactive<ByName<JustifyContent>>,
    direction: Reactive<ByName<InlineDirection>>,
    margin: Reactive<Sides>,
    padding: Reactive<Sides>,
    border: Reactive<Sides>,
    border_radius: Reactive<Corners>,
    flex_direction: Reactive<ByName<FlexDirection>>,
    flex_wrap: Reactive<ByName<FlexWrap>>,
    flex_grow: Reactive<f64>,
    flex_shrink: Reactive<f64>,
    flex_basis: Reactive<Size>,
    row_gap: Reactive<Size>,
    column_gap: Reactive<Size>,
    grid_auto_flow: Reactive<ByName<GridAutoFlow>>,
    background_color: Reactive<ColorValue>,
    border_color: Reactive<ColorValue>,
    z_index: Reactive<f64>,
    /// The text of a text element
    text: Reactive<Label>,
    font_size: Reactive<f64>,
    /// The colour of text
    color: Reactive<ColorValue>,
    /// The path of an image asset
    image: Reactive<String>,
    /// The `Name` of the entity, i.e. to find it in tests
    name: Reactive<Label>,
    on_click: Handler,
    on_press: Handler,
    on_release: Handler,
    on_pointer_enter: Handler,
    on_pointer_leave: Handler,
    /// Called with whether the pointer is over the element
    on_hover: WithDeps<fn(bool)>,
    on_drag: Handler,
    on_scroll: Handler,
}
