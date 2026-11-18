//! Testing UIs from scripts, in the spirit of React Testing Library: `import_namespace("reactive_ui_test")`.
//!
//! `render` mounts an element and returns a [`Screen`], which finds elements (`get_by_name`, `query_by_text`,
//! `within`), fires events at them (`click`) and checks what they show (`style`, `expect_style`). Events and `act`
//! run the work they cause before returning, so a test is a list of steps, without waiting for frames.
//!
//! ```lua
//! local t = import_namespace("reactive_ui_test")
//! local s = t.render(Counter({ start = 1 }))
//! s:click(s:get_by_name("inc"))
//! s:expect_style(s:get_by_name("count"), { text = "n=2" })
//! ```

use std::collections::{HashMap, VecDeque};

use bevy_app::{App, Plugin};
use bevy_ecs::{entity::Entity, hierarchy::Children, name::Name, world::World};
use bevy_mod_scripting_bindings::{
    A, ByName, FunctionCallContext, InteropError, Lazy, NamespaceMembers, ScriptValue, V, ValueOf,
    WithDeps,
};
use bevy_mod_scripting_derive::{ScriptData, script_bindings};
use bevy_picking::Pickable;
use bevy_reflect::Reflect;
use bevy_ui::{
    AlignContent, AlignItems, AlignSelf, BoxSizing, FlexDirection, FlexWrap, GridAutoFlow,
    InlineDirection, JustifyContent, JustifyItems, JustifySelf, Node, PositionType, Val,
    widget::Text,
};

use crate::{
    current_attachment,
    element::{UiContext, element_from_script, script_values_equal},
    events,
    props::{ColorValue, Corners, DisplayMode, Element, Label, OverflowMode, Sides, Size},
    runtime::{ContextsGuard, Cx, UpdateCounts, runtime},
    settle,
    style::{
        Prop, UiEvent,
        testing::{defaults, normalize_prop, read_prop, readable},
    },
};

/// Adds the `reactive_ui_test` script namespace, for testing UIs from scripts: render elements, find them by name or
/// text, click them and check what they show. See the "Testing UIs" page of the book.
#[derive(Default)]
pub struct ReactiveUiTestPlugin;

impl Plugin for ReactiveUiTestPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Screen>()
            .register_type::<ProvidedContext>();
        register_ui_test_functions(app.world_mut());
        register_screen_functions(app.world_mut());
    }

    fn finish(&self, app: &mut App) {
        layout::add_layout_schedule(app);
    }
}

/// Lays screens out on demand for `Screen::rect`, with `bevy_ui`'s layout systems in a schedule of their own.
mod layout {
    use bevy_app::{
        App, on_r_inserted, on_r_removed, propagate_inherited, propagate_output, update_source,
    };
    use bevy_camera::{Camera, Camera2d, ComputedCameraValues, RenderTargetInfo, Viewport};
    use bevy_ecs::{
        hierarchy::ChildOf,
        schedule::{ApplyDeferred, IntoScheduleConfigs, ScheduleLabel},
        world::World,
    };
    use bevy_math::UVec2;
    use bevy_ui::{
        ComputedUiRenderTargetInfo, ComputedUiTargetCamera, UiPlugin, UiScale,
        sync_font_size_to_em_size, ui_layout_system, ui_surface::UiSurface,
        update::propagate_ui_target_cameras,
    };

    /// The size of the window screens are laid out in when the app has no camera, in logical pixels.
    pub(super) const SIZE: UVec2 = UVec2::new(1280, 720);

    #[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
    struct UiTestLayout;

    pub(super) fn add_layout_schedule(app: &mut App) {
        if !app.is_plugin_added::<UiPlugin>() {
            // what `UiPlugin` sets up for the layout, see `bevy_ui`'s layout tests
            app.init_resource::<UiScale>()
                .init_resource::<UiSurface>()
                .init_resource::<bevy_text::TextPipeline>()
                .init_resource::<bevy_text::FontCx>()
                .init_resource::<bevy_text::RemSize>()
                .init_resource::<bevy_text::ScaleCx>()
                .add_observer(on_r_inserted::<ComputedUiTargetCamera, (), ChildOf>)
                .add_observer(on_r_removed::<ComputedUiTargetCamera, (), ChildOf>)
                .add_observer(on_r_inserted::<ComputedUiRenderTargetInfo, (), ChildOf>)
                .add_observer(on_r_removed::<ComputedUiRenderTargetInfo, (), ChildOf>);
        }
        app.add_systems(
            UiTestLayout,
            (
                ApplyDeferred,
                propagate_ui_target_cameras,
                ApplyDeferred,
                (
                    update_source::<ComputedUiTargetCamera, (), ChildOf>,
                    propagate_inherited::<ComputedUiTargetCamera, (), ChildOf>,
                    propagate_output::<ComputedUiTargetCamera, ()>,
                    update_source::<ComputedUiRenderTargetInfo, (), ChildOf>,
                    propagate_inherited::<ComputedUiRenderTargetInfo, (), ChildOf>,
                    propagate_output::<ComputedUiRenderTargetInfo, ()>,
                )
                    .chain(),
                ApplyDeferred,
                sync_font_size_to_em_size,
                ui_layout_system,
            )
                .chain(),
        );
    }

    /// Lays out every UI node of the world, with a camera of [`SIZE`] when there is none.
    pub(super) fn lay_out(world: &mut World) {
        if world.query::<&Camera>().iter(world).next().is_none() {
            world.spawn((
                Camera2d,
                Camera {
                    computed: ComputedCameraValues {
                        target_info: Some(RenderTargetInfo {
                            physical_size: SIZE,
                            scale_factor: 1.0,
                        }),
                        ..Default::default()
                    },
                    viewport: Some(Viewport {
                        physical_size: SIZE,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ));
        }
        world.run_schedule(UiTestLayout);
    }
}

/// Where an element is, in logical pixels from the top left of the window.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

/// What a test sees of a UI: the elements below a root entity, made by `render` or `screen`.
#[derive(Clone, Copy, Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct Screen {
    root: Entity,
}

/// A value provided to what `render` renders, made by `context`.
#[derive(Clone, Debug, Reflect)]
#[reflect(opaque, Clone)]
pub struct ProvidedContext(UiContext, ScriptValue);

/// What an element shows, as its properties are set: layout, colours, text and image, as in `UiProps`.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub struct UiStyle {
    display: Option<DisplayMode>,
    box_sizing: Option<ByName<BoxSizing>>,
    position_type: Option<ByName<PositionType>>,
    overflow: Option<OverflowMode>,
    left: Option<Size>,
    right: Option<Size>,
    top: Option<Size>,
    bottom: Option<Size>,
    width: Option<Size>,
    height: Option<Size>,
    min_width: Option<Size>,
    min_height: Option<Size>,
    max_width: Option<Size>,
    max_height: Option<Size>,
    aspect_ratio: Option<f64>,
    align_items: Option<ByName<AlignItems>>,
    justify_items: Option<ByName<JustifyItems>>,
    align_self: Option<ByName<AlignSelf>>,
    justify_self: Option<ByName<JustifySelf>>,
    align_content: Option<ByName<AlignContent>>,
    justify_content: Option<ByName<JustifyContent>>,
    direction: Option<ByName<InlineDirection>>,
    margin: Option<Sides>,
    padding: Option<Sides>,
    border: Option<Sides>,
    border_radius: Option<Corners>,
    flex_direction: Option<ByName<FlexDirection>>,
    flex_wrap: Option<ByName<FlexWrap>>,
    flex_grow: Option<f64>,
    flex_shrink: Option<f64>,
    flex_basis: Option<Size>,
    row_gap: Option<Size>,
    column_gap: Option<Size>,
    grid_auto_flow: Option<ByName<GridAutoFlow>>,
    background_color: Option<ColorValue>,
    border_color: Option<ColorValue>,
    z_index: Option<f64>,
    global_z_index: Option<f64>,
    text: Option<Label>,
    font_size: Option<f64>,
    color: Option<ColorValue>,
    image: Option<String>,
    name: Option<Label>,
}

/// What ran since the screen was made or `updates` was last called.
#[derive(ScriptData)]
#[script_data(
    namespace = "reactive_ui",
    bms_bindings_path = "bevy_mod_scripting_bindings"
)]
pub struct Updates {
    /// How many times each component rendered, by its name
    renders: HashMap<String, i64>,
    /// How many bound properties were computed again
    bindings: i64,
    /// How many `show` and `list` regions ran again
    regions: i64,
    /// How many effects ran again
    effects: i64,
}

/// The entities below `root`, breadth first.
fn descendants(world: &World, root: Entity) -> Vec<Entity> {
    let mut found = Vec::new();
    let mut queue = VecDeque::from([root]);
    while let Some(entity) = queue.pop_front() {
        if entity != root {
            found.push(entity);
        }
        if let Some(children) = world.get::<Children>(entity) {
            queue.extend(children.iter());
        }
    }
    found
}

/// What an element is called in errors: its name, or its entity.
fn label(world: &World, entity: Entity) -> String {
    world
        .get::<Name>(entity)
        .map_or_else(|| format!("{entity}"), |name| format!("`{name}`"))
}

/// A value as a script writes it.
fn show(value: &ScriptValue) -> String {
    match value {
        ScriptValue::Unit => "nil".into(),
        ScriptValue::String(s) => format!("\"{s}\""),
        ScriptValue::Integer(i) => i.to_string(),
        ScriptValue::Float(f) => f.to_string(),
        ScriptValue::Bool(b) => b.to_string(),
        ScriptValue::List(list) => {
            let items = list.iter().map(show).collect::<Vec<_>>();
            format!("{{ {} }}", items.join(", "))
        }
        ScriptValue::Map(map) => {
            let mut items = map
                .iter()
                .map(|(k, v)| format!("{k} = {}", show(v)))
                .collect::<Vec<_>>();
            items.sort();
            format!("{{ {} }}", items.join(", "))
        }
        other => format!("{other:?}"),
    }
}

/// Writes the line of an element and those of its children, see `Screen::snapshot`.
fn snapshot_of(
    world: &World,
    entity: Entity,
    depth: usize,
    defaults: &bevy_platform::collections::HashMap<Prop, ScriptValue>,
    out: &mut String,
) {
    let Ok(e) = world.get_entity(entity) else {
        return;
    };
    let kind = if e.contains::<Text>() {
        "text"
    } else if e.contains::<bevy_ui::widget::ImageNode>() {
        "image"
    } else {
        "node"
    };
    let mut props = readable()
        .map(|(name, prop)| (name, read_prop(e, prop), prop))
        .filter(|(_, value, prop)| {
            !matches!(value, ScriptValue::Unit)
                && !defaults
                    .get(prop)
                    .is_some_and(|default| script_values_equal(default, value))
        })
        .map(|(name, value, _)| format!("{name}={}", show(&value)))
        .collect::<Vec<_>>();
    props.sort();
    out.push_str(&"  ".repeat(depth));
    out.push_str(kind);
    for prop in props {
        out.push(' ');
        out.push_str(&prop);
    }
    out.push('\n');
    if let Some(children) = e.get::<Children>() {
        for child in children.iter() {
            snapshot_of(world, *child, depth + 1, defaults, out);
        }
    }
}

/// Finds elements of a screen by what they are called or show.
#[derive(Clone, Copy)]
enum By {
    Name,
    Text,
}

impl By {
    fn matches(self, world: &World, entity: Entity, wanted: &str) -> bool {
        match self {
            By::Name => world
                .get::<Name>(entity)
                .is_some_and(|n| n.as_str() == wanted),
            By::Text => world.get::<Text>(entity).is_some_and(|t| t.0 == wanted),
        }
    }

    fn describe(self, wanted: &str) -> String {
        match self {
            By::Name => format!("named `{wanted}`"),
            By::Text => format!("with the text `{wanted}`"),
        }
    }
}

impl Screen {
    fn all(
        self,
        ctx: &FunctionCallContext,
        by: By,
        wanted: &str,
    ) -> Result<Vec<Entity>, InteropError> {
        Ok(ctx.world()?.with_world_access(|w| {
            descendants(w, self.root)
                .into_iter()
                .filter(|e| by.matches(w, *e, wanted))
                .collect()
        })?)
    }

    /// The only match, nil when there is none, an error when there are more.
    fn query(
        self,
        ctx: &FunctionCallContext,
        by: By,
        wanted: &str,
    ) -> Result<Option<V<Entity>>, InteropError> {
        match self.all(ctx, by, wanted)?[..] {
            [] => Ok(None),
            [one] => Ok(Some(V(one))),
            ref many => Err(InteropError::string(format!(
                "{} elements {}, use `all_by_*` to find more than one",
                many.len(),
                by.describe(wanted)
            ))),
        }
    }

    fn get(
        self,
        ctx: &FunctionCallContext,
        by: By,
        wanted: &str,
    ) -> Result<V<Entity>, InteropError> {
        self.query(ctx, by, wanted)?
            .ok_or_else(|| InteropError::string(format!("no element {}", by.describe(wanted))))
    }

    /// Fires an event at the element, its handler or its nearest ancestor's runs, with what it changes.
    fn fire(ctx: &FunctionCallContext, entity: Entity, event: UiEvent) -> Result<(), InteropError> {
        Screen::queue(ctx, entity, event)?;
        settle(ctx)
    }

    /// Queues the handler an event at the element reaches, for the next UI update.
    fn queue(
        ctx: &FunctionCallContext,
        entity: Entity,
        event: UiEvent,
    ) -> Result<(), InteropError> {
        let runtime = runtime(ctx)?;
        Ok(ctx
            .world()?
            .with_world_access(|w| events::simulate(w, &runtime, entity, event))?)
    }
}

/// The event with the name, i.e. `"click"`.
fn event_named(name: &str) -> Result<UiEvent, InteropError> {
    UiEvent::ALL
        .iter()
        .find(|(event, _)| *event == name)
        .map(|(_, event)| *event)
        .ok_or_else(|| {
            let names = UiEvent::ALL.map(|(name, _)| name).join(", ");
            InteropError::string(format!("unknown event `{name}`, use one of: {names}"))
        })
}

/// Starts counting updates, see [`Screen::updates`].
fn count_updates(ctx: &FunctionCallContext) -> Result<(), InteropError> {
    runtime(ctx)?
        .lock()
        .updates
        .get_or_insert_with(UpdateCounts::default);
    Ok(())
}

/// Tests UIs: `import_namespace("reactive_ui_test")`.
#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "ui_test_functions",
    namespace = "reactive_ui_test"
)]
impl NamespaceMembers {
    /// Mounts an element under a root of its own filling the window, and runs what it starts (effects).
    /// Any element can be rendered, not only components.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local t = import_namespace("reactive_ui_test")
    /// local s = t.render(ui.text("hello"))
    /// assert(s:query_by_text("hello") ~= nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let t = import_namespace("reactive_ui_test");
    /// let s = t.render(ui.text("hello"));
    /// assert!(s.query_by_text("hello") != ());
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `element`: The element, i.e. `Counter({ start = 1 })`.
    /// * `contexts`: Values provided to it, made with `context`.
    /// Returns:
    /// * `screen`: The screen showing it.
    fn render(
        ctx: FunctionCallContext,
        element: Element,
        contexts: Option<Vec<V<ProvidedContext>>>,
    ) -> Result<V<Screen>, InteropError> {
        let attachment = current_attachment(&ctx)?;
        let runtime = runtime(&ctx)?;
        let Some(element) = element_from_script(&element.0, &runtime)? else {
            return Err(InteropError::str("`render` takes an element"));
        };
        let world = ctx.world()?;
        let root = world.with_world_mut_access(|w| {
            let node = Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..Default::default()
            };
            w.spawn((node, Pickable::IGNORE)).id()
        })?;
        count_updates(&ctx)?;
        {
            let _provided = contexts
                .into_iter()
                .flatten()
                .map(|provided| {
                    let ProvidedContext(context, value) = provided.into_inner();
                    ContextsGuard::provide(context, value)
                })
                .collect::<Vec<_>>();
            let cx = Cx {
                runtime: &runtime,
                world,
                attachment: &attachment,
                language: ctx.language(),
            };
            cx.mount(&element, root)?;
        }
        settle(&ctx)?;
        Ok(V(Screen { root }))
    }

    /// A screen of a UI already mounted, i.e. by `render` of `reactive_ui`, once the work queued so far ran.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local Menu = ui.component("Menu", function() return ui.text({ name = "menu", text = "menu" }) end)
    /// local root = ui.render("menu", Menu())
    /// local s = import_namespace("reactive_ui_test").screen(root)
    /// assert(s:text(s:get_by_name("menu")) == "menu")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn menu() {
    ///     import_namespace("reactive_ui").text(#{ name: "menu", text: "menu" })
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     let root = ui.render("menu", ui.component("Menu", menu).call());
    ///     let s = import_namespace("reactive_ui_test").screen(root);
    ///     assert!(s.text(s.get_by_name("menu")) == "menu");
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `root`: The root entity.
    /// Returns:
    /// * `screen`: The screen showing what is below it.
    fn screen(ctx: FunctionCallContext, root: V<Entity>) -> Result<V<Screen>, InteropError> {
        count_updates(&ctx)?;
        settle(&ctx)?;
        Ok(V(Screen { root: *root }))
    }

    /// A value to provide to what `render` renders, as `provide` does, without a node around it.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local t = import_namespace("reactive_ui_test")
    /// local Theme = ui.create_context("light")
    /// local Page = ui.component("Page", function() return ui.text({ name = "theme", text = ui.use_context(Theme) }) end)
    /// local s = t.render(Page(), { t.context(Theme, "dark") })
    /// assert(s:text(s:get_by_name("theme")) == "dark")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn page() {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.text(#{ name: "theme", text: ui.use_context(state()["Theme"]) })
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     let t = import_namespace("reactive_ui_test");
    ///     state()["Theme"] = ui.create_context("light");
    ///     let s = t.render(ui.component("Page", page).call(), [t.context(state()["Theme"], "dark")]);
    ///     assert!(s.text(s.get_by_name("theme")) == "dark");
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `context`: The context.
    /// * `value`: The value provided.
    /// Returns:
    /// * `provided`: The context with its value.
    fn context(
        _ctx: FunctionCallContext,
        context: V<UiContext<A>>,
        value: ValueOf<A, ScriptValue>,
    ) -> Result<V<ProvidedContext>, InteropError> {
        Ok(V(ProvidedContext(*context, value.into_inner())))
    }
}

/// Finds elements, fires events at them and checks what they show.
#[script_bindings(
    remote,
    bms_bindings_path = "bevy_mod_scripting_bindings",
    name = "screen_functions"
)]
impl Screen {
    /// The element with the `Name`, an error unless there is exactly one. Elements are named with the `name` property.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({ name = "panel" }))
    /// local panel = s:get_by_name("panel")
    /// assert(panel ~= nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{ name: "panel" }, []));
    /// let panel = s.get_by_name("panel");
    /// assert!(panel != ());
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `name`: The name.
    /// Returns:
    /// * `element`: The element.
    fn get_by_name(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        name: String,
    ) -> Result<V<Entity>, InteropError> {
        screen.get(&ctx, By::Name, &name)
    }

    /// The element with the `Name`, nil when there is none, an error when there are more. For checking that
    /// something is not shown.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({ name = "panel" }))
    /// assert(s:query_by_name("tooltip") == nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{ name: "panel" }, []));
    /// assert!(s.query_by_name("tooltip") == ());
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `name`: The name.
    /// Returns:
    /// * `element`: The element, or nil.
    fn query_by_name(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        name: String,
    ) -> Result<Option<V<Entity>>, InteropError> {
        screen.query(&ctx, By::Name, &name)
    }

    /// The elements with the `Name`, breadth first.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({}, { ui.node({ name = "slot" }), ui.node({ name = "slot" }) }))
    /// assert(#s:all_by_name("slot") == 2)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{}, [ui.node(#{ name: "slot" }, []), ui.node(#{ name: "slot" }, [])]));
    /// assert!(s.all_by_name("slot").len() == 2);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `name`: The name.
    /// Returns:
    /// * `elements`: The elements.
    fn all_by_name(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        name: String,
    ) -> Result<Vec<V<Entity>>, InteropError> {
        Ok(screen
            .all(&ctx, By::Name, &name)?
            .into_iter()
            .map(V)
            .collect())
    }

    /// The text element showing the text, an error unless there is exactly one. Finds what a player reads, without
    /// naming it.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({}, { ui.text("Save"), ui.text("Load") }))
    /// assert(s:get_by_text("Save") ~= s:get_by_text("Load"))
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{}, [ui.text("Save"), ui.text("Load")]));
    /// assert!(s.get_by_text("Save") != s.get_by_text("Load"));
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `text`: The whole text.
    /// Returns:
    /// * `element`: The element.
    fn get_by_text(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        text: String,
    ) -> Result<V<Entity>, InteropError> {
        screen.get(&ctx, By::Text, &text)
    }

    /// The text element showing the text, nil when there is none, an error when there are more.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.text("Save"))
    /// assert(s:query_by_text("Saved!") == nil)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.text("Save"));
    /// assert!(s.query_by_text("Saved!") == ());
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `text`: The whole text.
    /// Returns:
    /// * `element`: The element, or nil.
    fn query_by_text(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        text: String,
    ) -> Result<Option<V<Entity>>, InteropError> {
        screen.query(&ctx, By::Text, &text)
    }

    /// The text elements showing the text, breadth first.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({}, { ui.text("x1"), ui.text("x1"), ui.text("x2") }))
    /// assert(#s:all_by_text("x1") == 2)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{}, [ui.text("x1"), ui.text("x1"), ui.text("x2")]));
    /// assert!(s.all_by_text("x1").len() == 2);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `text`: The whole text.
    /// Returns:
    /// * `elements`: The elements.
    fn all_by_text(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        text: String,
    ) -> Result<Vec<V<Entity>>, InteropError> {
        Ok(screen
            .all(&ctx, By::Text, &text)?
            .into_iter()
            .map(V)
            .collect())
    }

    /// A screen of what is below the element: its queries only find elements inside it, i.e. the text of one row.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({}, {
    ///     ui.node({ name = "a" }, { ui.text("buy") }),
    ///     ui.node({ name = "b" }, { ui.text("buy") }),
    /// }))
    /// local a = s:within(s:get_by_name("a"))
    /// assert(#s:all_by_text("buy") == 2 and #a:all_by_text("buy") == 1)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{}, [
    ///     ui.node(#{ name: "a" }, [ui.text("buy")]),
    ///     ui.node(#{ name: "b" }, [ui.text("buy")]),
    /// ]));
    /// let a = s.within(s.get_by_name("a"));
    /// assert!(s.all_by_text("buy").len() == 2 && a.all_by_text("buy").len() == 1);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    /// Returns:
    /// * `screen`: The screen of the element.
    fn within(
        _ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
    ) -> Result<V<Screen>, InteropError> {
        let _ = screen;
        Ok(V(Screen { root: *element }))
    }

    /// The children of the element, in order.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({ name = "list" }, { ui.text("first"), ui.text("second") }))
    /// local children = s:children(s:get_by_name("list"))
    /// assert(s:text(children[2]) == "second")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{ name: "list" }, [ui.text("first"), ui.text("second")]));
    /// let children = s.children(s.get_by_name("list"));
    /// assert!(s.text(children[1]) == "second");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    /// Returns:
    /// * `children`: Its children.
    fn children(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
    ) -> Result<Vec<V<Entity>>, InteropError> {
        let _ = screen;
        Ok(ctx.world()?.with_world_access(|w| {
            w.get::<Children>(*element)
                .map_or_else(Vec::new, |c| c.iter().copied().map(V).collect())
        })?)
    }

    /// The text of a text element, nil for other elements.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.text({ name = "gold", text = "120 gold" }))
    /// assert(s:text(s:get_by_name("gold")) == "120 gold")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "gold", text: "120 gold" }));
    /// assert!(s.text(s.get_by_name("gold")) == "120 gold");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    /// Returns:
    /// * `text`: Its text.
    fn text(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
    ) -> Result<Option<String>, InteropError> {
        let _ = screen;
        Ok(ctx
            .world()?
            .with_world_access(|w| w.get::<Text>(*element).map(|t| t.0.clone()))?)
    }

    /// Clicks the element, running its handler (or its nearest ancestor's) and what it changes.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local on = ui.signal(false)
    /// local s = import_namespace("reactive_ui_test").render(ui.node({ on_click = function() on:set(true) end }, {
    ///     ui.text({ name = "label", text = function() return on:get() and "on" or "off" end }),
    /// }))
    /// s:click(s:get_by_name("label"))
    /// assert(s:text(s:get_by_name("label")) == "on")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let on = ui.signal(false);
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{ on_click: [|on, _event| on.set(true), on] }, [
    ///     ui.text(#{ name: "label", text: ui.when(ui.path(on, ""), "on", "off") }),
    /// ]));
    /// s.click(s.get_by_name("label"));
    /// assert!(s.text(s.get_by_name("label")) == "on");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    fn click(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
    ) -> Result<(), InteropError> {
        let _ = screen;
        Screen::fire(&ctx, *element, UiEvent::Click)
    }

    /// Fires a pointer event at the element, running its handler (or its nearest ancestor's) and what it changes.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local hovered = ui.signal(false)
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.node({ name = "item", on_hover = function(over) hovered:set(over) end })
    /// )
    /// s:interact(s:get_by_name("item"), "pointer_enter")
    /// assert(hovered:peek())
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let hovered = ui.signal(false);
    /// let s = import_namespace("reactive_ui_test").render(
    ///     ui.node(#{ name: "item", on_hover: [|hovered, over| hovered.set(over), hovered] }, [])
    /// );
    /// s.interact(s.get_by_name("item"), "pointer_enter");
    /// assert!(hovered.peek());
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    /// * `event`: The event: `"click"`, `"press"`, `"release"`, `"pointer_enter"`, `"pointer_leave"`, `"hover"`, `"drag"` or `"scroll"`.
    fn interact(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
        event: String,
    ) -> Result<(), InteropError> {
        let _ = screen;
        Screen::fire(&ctx, *element, event_named(&event)?)
    }

    /// Queues a pointer event at the element for the next UI update, without running it: for what happens across
    /// updates, and for benchmarks timing the update.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local clicks = ui.signal(0)
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.node({ name = "button", on_click = function() clicks:set(clicks:peek() + 1) end })
    /// )
    /// s:dispatch(s:get_by_name("button"), "click")
    /// assert(clicks:peek() == 0, "it runs in the next update")
    /// s:act(function() end)
    /// assert(clicks:peek() == 1)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let clicks = ui.signal(0);
    /// let s = import_namespace("reactive_ui_test").render(
    ///     ui.node(#{ name: "button", on_click: [|clicks, _event| clicks.set(clicks.peek() + 1), clicks] }, [])
    /// );
    /// s.dispatch(s.get_by_name("button"), "click");
    /// assert!(clicks.peek() == 0, "it runs in the next update");
    /// s.act(|| ());
    /// assert!(clicks.peek() == 1);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    /// * `event`: The event, as `interact` takes it.
    fn dispatch(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
        event: String,
    ) -> Result<(), InteropError> {
        let _ = screen;
        Screen::queue(&ctx, *element, event_named(&event)?)
    }

    /// Calls the function, then runs what it changed: for changes other than events, i.e. setting signals or
    /// changing watched components.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local gold = ui.signal(0)
    /// local s = import_namespace("reactive_ui_test").render(ui.text({ name = "gold", text = ui.format("{} gold", { ui.path(gold, "") }) }))
    /// s:act(function() gold:set(120) end)
    /// assert(s:text(s:get_by_name("gold")) == "120 gold")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let gold = ui.signal(0);
    /// let s = import_namespace("reactive_ui_test").render(ui.text(#{ name: "gold", text: ui.format("{} gold", [ui.path(gold, "")]) }));
    /// s.act([|gold| gold.set(120), gold]);
    /// assert!(s.text(s.get_by_name("gold")) == "120 gold");
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `f`: The function.
    fn act(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        f: WithDeps<fn()>,
    ) -> Result<(), InteropError> {
        let _ = screen;
        if let ScriptValue::Error(e) = f.into_inner().call(vec![], ctx.clone())? {
            return Err(e);
        }
        settle(&ctx)
    }

    /// What the element shows, as its properties are set: only those it has, in one form however they were written.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({ name = "panel", width = "50%", padding = 4 }))
    /// local style = s:style(s:get_by_name("panel"))
    /// assert(style.width == "50%" and style.padding == 4)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{ name: "panel", width: "50%", padding: 4 }, []));
    /// let style = s.style(s.get_by_name("panel"));
    /// assert!(style.width == "50%" && style.padding == 4.0);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    /// Returns:
    /// * `style`: Its properties.
    fn style(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
    ) -> Result<Lazy<UiStyle>, InteropError> {
        let _ = screen;
        let style = ctx.world()?.with_world_access(|w| {
            let entity = w
                .get_entity(*element)
                .map_err(|_| InteropError::str("the element is gone"))?;
            Ok::<_, InteropError>(
                readable()
                    .map(|(name, prop)| (name.to_owned(), read_prop(entity, prop)))
                    .filter(|(_, value)| !matches!(value, ScriptValue::Unit))
                    .collect(),
            )
        })??;
        Ok(Lazy::new(ScriptValue::Map(style)))
    }

    /// Checks that the element shows the properties given, as they would be set: `{ width = "50%" }`.
    /// Fails with every property which differs: ``` `panel`: width: expected "50%", got "40%" ```.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(
    ///     ui.node({ name = "panel", flex_direction = "Column", background_color = { 1, 0.8, 0 } })
    /// )
    /// s:expect_style(s:get_by_name("panel"), { flex_direction = "Column", background_color = "#ffcc00" })
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(
    ///     ui.node(#{ name: "panel", flex_direction: "Column", background_color: [1.0, 0.8, 0.0] }, [])
    /// );
    /// s.expect_style(s.get_by_name("panel"), #{ flex_direction: "Column", background_color: "#ffcc00" });
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    /// * `expected`: The properties to check.
    fn expect_style(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
        expected: Lazy<UiStyle>,
    ) -> Result<(), InteropError> {
        let _ = screen;
        let ScriptValue::Map(expected) = expected.into_inner() else {
            return Err(InteropError::str(
                "`expect_style` takes a table of properties",
            ));
        };
        let mut expected = expected.into_iter().collect::<Vec<_>>();
        expected.sort_by(|(a, _), (b, _)| a.cmp(b));
        let (label, differences) = ctx.world()?.with_world_access(|w| {
            let entity = w
                .get_entity(*element)
                .map_err(|_| InteropError::str("the element is gone"))?;
            let mut differences = Vec::new();
            for (name, value) in &expected {
                let prop = match Prop::parse(name) {
                    Some(Prop::On(_)) | None => {
                        return Err(InteropError::string(format!(
                            "`{name}` is not a property which can be checked"
                        )));
                    }
                    Some(prop) => prop,
                };
                let wanted = normalize_prop(prop, value)?;
                let actual = read_prop(entity, prop);
                if !script_values_equal(&wanted, &actual) {
                    differences.push(format!(
                        "{name}: expected {}, got {}",
                        show(&wanted),
                        show(&actual)
                    ));
                }
            }
            Ok((label(w, *element), differences))
        })??;
        if differences.is_empty() {
            return Ok(());
        }
        Err(InteropError::string(format!(
            "{label}: {}",
            differences.join("; ")
        )))
    }

    /// The tree below the element (the screen's root by default) as text, the same for the same UI: one line per
    /// element, indented by depth, with its kind and the properties it has which aren't the defaults, sorted by name.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({ name = "panel", flex_direction = "Column" }, {
    ///     ui.text({ text = "v1", font_size = 20 }),
    /// }))
    /// assert(s:snapshot() == [[
    /// node flex_direction="column" name="panel"
    ///   text font_size=20 text="v1"
    /// ]], s:snapshot())
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{ name: "panel", flex_direction: "Column" }, [
    ///     ui.text(#{ text: "v1", font_size: 20 }),
    /// ]));
    /// let snapshot = s.snapshot();
    /// assert!(snapshot == "node flex_direction=\"column\" name=\"panel\"\n  text font_size=20 text=\"v1\"\n", "{}", snapshot);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: Where to start, the screen's root if nil.
    /// Returns:
    /// * `snapshot`: The tree as text.
    fn snapshot(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: Option<V<Entity>>,
    ) -> Result<String, InteropError> {
        let whole = element.is_none();
        let start = element.map_or(screen.root, |e| *e);
        // the screen's root is a container of its own, what is in it is what was rendered
        let roots = if whole {
            ctx.world()?.with_world_access(|w| {
                w.get::<Children>(start)
                    .map_or_else(Vec::new, |c| c.iter().copied().collect())
            })?
        } else {
            vec![start]
        };
        let defaults = defaults();
        Ok(ctx.world()?.with_world_access(|w| {
            let mut out = String::new();
            for root in roots {
                snapshot_of(w, root, 0, &defaults, &mut out);
            }
            out
        })?)
    }

    /// Where the element is once laid out, in logical pixels from the top left of the window. Lays the UI out
    /// first: in a window of 1280x720 when the app has no camera, and without measuring text.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local s = import_namespace("reactive_ui_test").render(ui.node({ name = "bar", width = "50%", height = 10, margin = { left = 20 } }))
    /// local bar = s:rect(s:get_by_name("bar"))
    /// assert(bar.x == 20 and bar.y == 0 and bar.width == 640 and bar.height == 10)
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// let ui = import_namespace("reactive_ui");
    /// let s = import_namespace("reactive_ui_test").render(ui.node(#{ name: "bar", width: "50%", height: 10, margin: #{ left: 20 } }, []));
    /// let bar = s.rect(s.get_by_name("bar"));
    /// assert!(bar.x == 20.0 && bar.y == 0.0 && bar.width == 640.0 && bar.height == 10.0);
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// * `element`: The element.
    /// Returns:
    /// * `rect`: Its position and size.
    fn rect(
        ctx: FunctionCallContext,
        screen: V<Screen>,
        element: V<Entity>,
    ) -> Result<Rect, InteropError> {
        let _ = screen;
        ctx.world()?.with_world_mut_access(|w| {
            layout::lay_out(w);
            let (Some(node), Some(transform)) = (
                w.get::<bevy_ui::ComputedNode>(*element),
                w.get::<bevy_ui::UiGlobalTransform>(*element),
            ) else {
                return Err(InteropError::str(
                    "the element is not laid out, is it a ui node?",
                ));
            };
            let scale = node.inverse_scale_factor;
            let size = node.size * scale;
            let center = transform.translation * scale;
            Ok(Rect {
                x: (center.x - size.x / 2.0) as f64,
                y: (center.y - size.y / 2.0) as f64,
                width: size.x as f64,
                height: size.y as f64,
            })
        })?
    }

    /// What ran since the screen was made or this was last called: renders of each component by name, and how many
    /// bound properties, regions and effects ran again. For checking that a change only updates what it should.
    ///
    /// ```multi-lang-example,lua
    /// local ui = import_namespace("reactive_ui")
    /// local hp = ui.signal(100)
    /// local Bar = ui.component("Bar", function()
    ///     return ui.text({ text = function() return "hp " .. hp:get() end })
    /// end)
    /// local s = import_namespace("reactive_ui_test").render(Bar())
    /// assert(s:updates().renders.Bar == 1)
    /// s:act(function() hp:set(50) end)
    /// local updates = s:updates()
    /// assert(updates.renders.Bar == nil and updates.bindings == 1, "only the text ran again")
    /// ```
    ///
    /// ```multi-lang-example,rune
    /// fn bar(hp) {
    ///     let ui = import_namespace("reactive_ui");
    ///     ui.text(#{ text: [|hp| `hp ${hp.get()}`, hp] })
    /// }
    ///
    /// pub fn main() {
    ///     let ui = import_namespace("reactive_ui");
    ///     let hp = ui.signal(100);
    ///     let s = import_namespace("reactive_ui_test").render(ui.component("Bar", bar).call(hp));
    ///     assert!(s.updates().renders["Bar"] == 1);
    ///     s.act([|hp| hp.set(50), hp]);
    ///     let updates = s.updates();
    ///     assert!(updates.renders.get("Bar") == None && updates.bindings == 1, "only the text ran again");
    /// }
    /// ```
    ///
    /// Arguments:
    /// * `ctx`: The function call context.
    /// * `screen`: The screen.
    /// Returns:
    /// * `updates`: What ran.
    fn updates(ctx: FunctionCallContext, screen: V<Screen>) -> Result<Updates, InteropError> {
        let _ = screen;
        let counts = std::mem::take(
            runtime(&ctx)?
                .lock()
                .updates
                .get_or_insert_with(UpdateCounts::default),
        );
        Ok(Updates {
            renders: counts.renders.into_iter().collect(),
            bindings: counts.bindings,
            regions: counts.regions,
            effects: counts.effects,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::props::UiProps;

    use bevy_mod_scripting_bindings::{ScriptDataShape, ThroughTypeInfo, TypedThrough};

    fn field_names<T: TypedThrough>() -> Vec<&'static str> {
        match T::through_type_info() {
            ThroughTypeInfo::Data(info) => match info.shape {
                ScriptDataShape::Fields(fields) => fields.iter().map(|f| f.name).collect(),
                ScriptDataShape::Union(_) => Vec::new(),
            },
            _ => Vec::new(),
        }
    }

    /// `UiStyle` documents the properties which can be read back, the properties of [`UiProps`] but `key` and handlers.
    #[test]
    fn ui_style_documents_every_readable_property() {
        let mut documented = field_names::<UiStyle>();
        let mut readable = readable().map(|(name, _)| name).collect::<Vec<_>>();
        let mut props = field_names::<UiProps>()
            .into_iter()
            .filter(|name| *name != "key" && !name.starts_with("on_"))
            .collect::<Vec<_>>();
        documented.sort();
        readable.sort();
        props.sort();
        assert_eq!(documented, readable);
        assert_eq!(documented, props);
    }
}
