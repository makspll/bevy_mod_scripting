//! The events of elements: `bevy_picking`'s pointer events, queued for the handler of the innermost
//! element which has one for the event. Handlers run in the next flush, in their script's context.

use bevy_app::App;
use bevy_ecs::{
    entity::Entity,
    hierarchy::ChildOf,
    observer::On,
    system::{Query, Res},
    world::World,
};
use bevy_mod_scripting_bindings::ScriptValue;
use bevy_picking::{
    events::{
        PointerClick, PointerDrag, PointerEnter, PointerEvent, PointerLeave, PointerPress,
        PointerRelease, PointerScroll,
    },
    pointer::PointerButton,
};
use bevy_platform::collections::HashMap;
use parking_lot::Mutex;

use crate::{
    ReactiveUi,
    runtime::{QueuedEvent, Runtime, UiHandlers},
    style::UiEvent,
};

/// What handlers are given: the position of the pointer and what the event adds to it.
type EventData = HashMap<String, ScriptValue>;

/// A pointer event handed to elements.
trait Forwarded: PointerEvent {
    const EVENT: UiEvent;

    /// Adds what the event has besides the position of the pointer.
    fn describe(&self, _data: &mut EventData) {}
}

fn button(button: PointerButton) -> ScriptValue {
    ScriptValue::String(
        match button {
            PointerButton::Primary => "primary",
            PointerButton::Secondary => "secondary",
            PointerButton::Middle => "middle",
        }
        .into(),
    )
}

impl Forwarded for PointerClick {
    const EVENT: UiEvent = UiEvent::Click;

    fn describe(&self, data: &mut EventData) {
        data.insert("button".to_owned(), button(self.button));
        data.insert("count".to_owned(), ScriptValue::Integer(self.count as i64));
    }
}

impl Forwarded for PointerPress {
    const EVENT: UiEvent = UiEvent::Press;

    fn describe(&self, data: &mut EventData) {
        data.insert("button".to_owned(), button(self.button));
    }
}

impl Forwarded for PointerRelease {
    const EVENT: UiEvent = UiEvent::Release;

    fn describe(&self, data: &mut EventData) {
        data.insert("button".to_owned(), button(self.button));
    }
}

impl Forwarded for PointerEnter {
    const EVENT: UiEvent = UiEvent::PointerEnter;
}

impl Forwarded for PointerLeave {
    const EVENT: UiEvent = UiEvent::PointerLeave;
}

impl Forwarded for PointerDrag {
    const EVENT: UiEvent = UiEvent::Drag;

    fn describe(&self, data: &mut EventData) {
        data.insert("button".to_owned(), button(self.button));
        data.insert(
            "delta_x".to_owned(),
            ScriptValue::Float(self.delta.x as f64),
        );
        data.insert(
            "delta_y".to_owned(),
            ScriptValue::Float(self.delta.y as f64),
        );
    }
}

impl Forwarded for PointerScroll {
    const EVENT: UiEvent = UiEvent::Scroll;

    fn describe(&self, data: &mut EventData) {
        data.insert("delta_x".to_owned(), ScriptValue::Float(self.x as f64));
        data.insert("delta_y".to_owned(), ScriptValue::Float(self.y as f64));
    }
}

/// The arguments of a handler: the event, or for `hover` handlers whether the pointer is over the element.
fn arguments(event: UiEvent, hover: bool, data: EventData) -> Vec<ScriptValue> {
    if hover {
        vec![ScriptValue::Bool(event == UiEvent::PointerEnter)]
    } else {
        vec![ScriptValue::Map(data)]
    }
}

/// Queues the handler of the element for the event, if it has one. Returns whether it did.
fn queue(
    runtime: &Mutex<Runtime>,
    element: &UiHandlers,
    event: UiEvent,
    data: impl FnOnce() -> EventData,
) -> bool {
    let Some((handler, hover)) = element.handler(event) else {
        return false;
    };
    runtime.lock().events.push(QueuedEvent {
        attachment: element.attachment.clone(),
        handler: handler.clone(),
        args: arguments(event, hover, data()),
    });
    true
}

/// Queues the handler of the entity the event is at, if it has one. Returns whether it did.
fn forward<E: Forwarded>(event: &E, elements: &Query<&UiHandlers>, ui: &ReactiveUi) -> bool {
    let Ok(element) = elements.get(event.event_target()) else {
        return false;
    };
    queue(&ui.0, element, E::EVENT, || {
        let position = event.pointer().position;
        let mut data = EventData::default();
        data.insert("x".to_owned(), ScriptValue::Float(position.x as f64));
        data.insert("y".to_owned(), ScriptValue::Float(position.y as f64));
        event.describe(&mut data);
        data
    })
}

/// Hands the pointer events of `bevy_picking` to elements.
pub(crate) fn add_observers(app: &mut App) {
    macro_rules! observe {
        ($($event:ty),*) => {
            $(app.add_observer(
                |mut event: On<$event>, elements: Query<&UiHandlers>, ui: Res<ReactiveUi>| {
                    if forward(&*event, &elements, &ui) {
                        // the innermost element handling an event gets it, it doesn't bubble further
                        event.propagate(false);
                    }
                },
            );)*
        };
    }
    observe!(
        PointerClick,
        PointerPress,
        PointerRelease,
        PointerEnter,
        PointerLeave,
        PointerDrag,
        PointerScroll
    );
}

/// Queues the handler a pointer event on the entity would reach: its own, or its nearest ancestor's.
/// For tests and for driving a ui without a pointer.
pub(crate) fn simulate(
    world: &World,
    runtime: &Mutex<Runtime>,
    mut entity: Entity,
    event: UiEvent,
) {
    loop {
        if let Some(element) = world.get::<UiHandlers>(entity)
            && queue(runtime, element, event, EventData::default)
        {
            return;
        }
        // enter and leave are not given to ancestors
        if matches!(event, UiEvent::PointerEnter | UiEvent::PointerLeave) {
            return;
        }
        match world.get::<ChildOf>(entity) {
            Some(child_of) => entity = child_of.parent(),
            None => return,
        }
    }
}
