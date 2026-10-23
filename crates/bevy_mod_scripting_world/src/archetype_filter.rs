//! Per-entity restrictions on component access, for guards of non-exclusive systems with filtered queries.
//!
//! # Why this exists
//!
//! An [`crate::AccessByteSet`] grants access per [`ComponentId`], i.e. to the component on *every* entity.
//! Bevy schedules systems by their [`bevy_ecs::query::FilteredAccess`] instead, which also carries the query
//! filters: a system writing `C` with `With<A>` and a system writing `C` with `Without<A>` are considered disjoint
//! and run in parallel. If a script in the first system could reach `C` on an entity *without* `A`
//! (i.e. via `world.get_component`), both systems would hold mutable references to the same value.
//!
//! # Why archetypes
//!
//! An archetype is the exact set of component types an entity has, every entity lives in exactly one.
//! `With`/`Without`/`Or` only depend on which components an entity has, so they match whole archetypes,
//! and a query records the archetypes it matched. Tables are NOT the right unit: entities with and without a
//! sparse set component `A` share a table (and its `C` column) while living in different archetypes.
//!
//! Non-archetypal filters (`Added`/`Changed`) match every archetype containing the component, so the
//! restriction over-approximates them, which is fine since Bevy also assumes the whole component is accessed.
//!
//! # Invariant
//!
//! The (component, archetype) pairs allowed here must be a subset of the pairs Bevy believes the system accesses.
//! Building the filter from the matched archetypes of the same `QueryState`s whose accesses are reported to Bevy
//! guarantees this for every kind of filter.
//!
//! Archetypes cannot change while a non-exclusive system runs (structural changes need exclusive world access),
//! so the filter stays valid for the whole run of the system.

use bevy_ecs::{archetype::ArchetypeId, component::ComponentId};

use crate::bit_set::BitSet;

/// Restricts access to components to entities in a set of archetypes, per component.
///
/// Components without a restriction are not affected, i.e. resources and components accessed by exclusive guards.
#[derive(Clone, Debug, Default)]
pub struct ArchetypeFilter {
    /// Restricted components, each with the set of allowed [`ArchetypeId::index`]es.
    /// Systems restrict a handful of components, so a linear scan beats indexing by component id.
    restricted: Vec<(ComponentId, BitSet)>,
}

impl ArchetypeFilter {
    fn bits_mut(&mut self, component_id: ComponentId) -> &mut BitSet {
        let index = match self
            .restricted
            .iter()
            .position(|(id, _)| *id == component_id)
        {
            Some(index) => index,
            None => {
                self.restricted.push((component_id, BitSet::default()));
                self.restricted.len() - 1
            }
        };
        &mut self.restricted[index].1
    }

    /// Allows access to the given component on entities in the given archetype.
    ///
    /// Once called for a component, the component is restricted to the archetypes allowed this way.
    pub fn allow(&mut self, component_id: ComponentId, archetype_id: ArchetypeId) {
        self.bits_mut(component_id).insert(archetype_id.index());
    }

    /// Restricts the given component to no archetypes, until [`Self::allow`] is called for it.
    pub fn restrict(&mut self, component_id: ComponentId) {
        self.bits_mut(component_id);
    }

    /// Returns true if the component may be accessed on an entity in the given archetype.
    #[inline]
    pub fn is_allowed(&self, component_id: ComponentId, archetype_id: ArchetypeId) -> bool {
        match self.restricted.iter().find(|(id, _)| *id == component_id) {
            Some((_, archetypes)) => archetypes.contains(archetype_id.index()),
            None => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unrestricted_components_are_allowed_everywhere() {
        let filter = ArchetypeFilter::default();
        assert!(filter.is_allowed(ComponentId::new(3), ArchetypeId::new(70)));
    }

    #[test]
    fn restricted_components_are_only_allowed_in_allowed_archetypes() {
        let mut filter = ArchetypeFilter::default();
        let (component, other) = (ComponentId::new(2), ComponentId::new(5));
        filter.restrict(component);
        assert!(!filter.is_allowed(component, ArchetypeId::new(1)));

        filter.allow(component, ArchetypeId::new(1));
        filter.allow(component, ArchetypeId::new(130));
        assert!(filter.is_allowed(component, ArchetypeId::new(1)));
        assert!(filter.is_allowed(component, ArchetypeId::new(130)));
        assert!(!filter.is_allowed(component, ArchetypeId::new(0)));
        assert!(!filter.is_allowed(component, ArchetypeId::new(64)));
        assert!(filter.is_allowed(other, ArchetypeId::new(0)));
    }
}
