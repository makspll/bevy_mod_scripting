use bevy::ecs::system::SystemParam;
use bevy::{ecs::event::EventId, prelude::*};
use std::marker::PhantomData;
use std::sync::atomic::Ordering::Relaxed;
use std::{collections::BinaryHeap, sync::atomic::AtomicU32};



pub trait PriorityEvent : Send + Sync + 'static {}
impl <E : Send + Sync + 'static> PriorityEvent for E {}

#[derive(Debug)]
struct EventInstance<E> {
    prio: u32,
    event_id: u32,
    event: E,
}

impl<E> EventInstance<E> {
    fn new(event: E, prio: u32) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        Self {
            prio,
            event_id: COUNTER.fetch_add(1, Relaxed),
            event,
        }
    }
}

impl<E> PartialEq for EventInstance<E> {
    fn eq(&self, other: &Self) -> bool {
        self.prio == other.prio && self.event_id == other.event_id
    }
}
impl<E> Eq for EventInstance<E> {}

impl<E> Ord for EventInstance<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.prio.cmp(&other.prio) {
            std::cmp::Ordering::Equal => self.event_id.cmp(&other.event_id),
            v => v,
        }
        .reverse()
    }
}
impl<E> PartialOrd for EventInstance<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<E: Clone> Clone for EventInstance<E> {
    fn clone(&self) -> Self {
        Self {
            prio: self.prio.clone(),
            event_id: self.event_id.clone(),
            event: self.event.clone(),
        }
    }
}

/// An event priority queue.
/// Used when the ordering of events should be influenced by other factors.
/// This implementation does NOT provide double buffering.
/// Writers and readers are expected to remove events as soon as they are read,
/// this implies a one to one mapping between events and event handlers.
#[derive(Debug)]
pub struct PriorityEvents<E> {
    events: BinaryHeap<EventInstance<E>>,
}

impl<E> Default for PriorityEvents<E> {
    fn default() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }
}

#[derive(SystemParam)]
pub struct PriorityEventReader<'w, 's, E: PriorityEvent> {
    events: ResMut<'w, PriorityEvents<E>>,
    #[system_param(ignore)]
    marker: PhantomData<&'s usize>,
}

pub struct PriorityIterator<'w, E: PriorityEvent> {
    min: u32,
    events: &'w mut PriorityEvents<E>,
}

impl<'w, E: PriorityEvent> Iterator for PriorityIterator<'w, E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e) = self.events.events.peek() {
            if e.prio > self.min {
                return None;
            };
        };

        self.events.events.pop().map(|e| e.event)
    }
}

impl<'s, E: PriorityEvent> PriorityEventReader<'_, 's, E> {
    /// Iterates over events this reader has not seen yet, while also clearing them.
    /// Will not remove any events of priority lower than min (0 is highest, inf is lowest)
    pub fn iter_min_prio(&mut self, min: u32) -> impl Iterator<Item = E> + '_ {
        PriorityIterator {
            min,
            events: self.events.as_mut(),
        }
    }

    /// Determines the number of events available to be read, without consuming any
    pub fn len(&self) -> usize {
        self.events.events.len()
    }

    /// Determines if there are any events to be read, without consuming any.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(SystemParam)]
pub struct PriorityEventWriter<'w, 's, E: PriorityEvent> {
    events: ResMut<'w, PriorityEvents<E>>,

    #[system_param(ignore)]
    marker: PhantomData<&'s usize>,
}

impl<'w, 's, E: PriorityEvent> PriorityEventWriter<'w, 's, E> {
    pub fn send(&mut self, event: E, prio: u32) {
        self.events.events.push(EventInstance::new(event, prio));
    }

    pub fn send_batch(&mut self, events: impl Iterator<Item = E>, prio: u32) {
        self.events
            .events
            .extend(events.map(|v| EventInstance::new(v, prio)))
    }

    pub fn send_default(&mut self, prio: u32)
    where
        E: Default,
    {
        self.events
            .events
            .push(EventInstance::new(E::default(), prio))
    }
}


/// a convenience for initialising prioritised event types
pub trait AddPriorityEvent {
    fn add_priority_event<E : PriorityEvent>(&mut self) -> &mut Self;
}

impl AddPriorityEvent for App{
    fn add_priority_event<E : PriorityEvent>(&mut self) -> &mut Self {
        self.init_resource::<PriorityEvents<E>>();

        self
    }
}

#[cfg(test)]
mod tests {
    use bevy::{ecs::system::SystemState, prelude::World};

    use super::*;

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    struct TestEvent(usize);

    fn collect_events<E: Copy>(events: BinaryHeap<EventInstance<E>>) -> Vec<E> {
        events
            .into_sorted_vec()
            .iter()
            .map(|e| e.event)
            .rev()
            .collect()
    }

    #[test]
    fn test_events() {
        let mut world = World::new();
        let mut state_writer: SystemState<PriorityEventWriter<TestEvent>> =
            SystemState::new(&mut world);
        let mut state_reader: SystemState<PriorityEventReader<TestEvent>> =
            SystemState::new(&mut world);

        world.init_resource::<PriorityEvents<TestEvent>>();

        // stage 1

        {
            let mut w = state_writer.get_mut(&mut world);

            // system writes three events, out of order
            w.send(TestEvent(0), 5);
            w.send(TestEvent(1), 1);
            w.send(TestEvent(2), 0);
        }

        // events are send and ordered in decreasing priority order
        assert_eq!(
            collect_events(world.resource::<PriorityEvents<TestEvent>>().events.clone()),
            vec![TestEvent(2), TestEvent(1), TestEvent(0)]
        );

        // stage 2

        {
            let mut w = state_reader.get_mut(&mut world);

            // system reads only top event
            w.iter_min_prio(0).for_each(drop);
        }

        // first event is consumed immediately
        assert_eq!(
            collect_events(world.resource::<PriorityEvents<TestEvent>>().events.clone()),
            vec![TestEvent(1), TestEvent(0)]
        );

        // stage 3 

        {
            let mut w = state_reader.get_mut(&mut world);

            // system reads all events
            w.iter_min_prio(5).for_each(drop);
        }

        // first event is consumed immediately
        assert_eq!(
            collect_events(world.resource::<PriorityEvents<TestEvent>>().events.clone()),
            Vec::default()
        );
    }
}
