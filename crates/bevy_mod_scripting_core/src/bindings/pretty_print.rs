//! Pretty printing for reflect references and other types.

use crate::reflection_extensions::{FakeType, TypeIdExtensions};

use crate::bindings::{
    ReflectAllocationId, ReflectBase, ReflectBaseType, ReflectReference, WorldGuard,
    access_map::ReflectAccessId, script_value::ScriptValue,
};
use ::{
    bevy_ecs::component::ComponentId,
    bevy_reflect::{PartialReflect, ReflectRef},
};
use bevy_ecs::world::World;
use bevy_platform::collections::HashMap;
use bevy_reflect::VariantType;
use itertools::Itertools;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
};

/// A utility for printing reflect references in a human readable format.
pub struct ReflectReferencePrinter {
    pub(crate) reference: ReflectReference,
}

#[derive(Clone, Copy, Debug)]
enum BracketType {
    Square,
    Curly,
    Round,
}

impl BracketType {
    fn open(self) -> char {
        match self {
            BracketType::Square => '[',
            BracketType::Curly => '{',
            BracketType::Round => '(',
        }
    }

    fn close(self) -> char {
        match self {
            BracketType::Square => ']',
            BracketType::Curly => '}',
            BracketType::Round => ')',
        }
    }

    fn surrounded<F: FnOnce(&mut String)>(self, output: &mut String, f: F) {
        output.push(self.open());
        f(output);
        output.push(self.close());
    }
}

macro_rules! downcast_case {
    ($id:ident, $out:ident, $t:path) => {{
        $out.push_str(stringify!($t));
        $out.push('(');
        if let Some($id) = $id.as_partial_reflect().try_downcast_ref::<$t>() {
            $out.push_str(&format!("{:?}", $id));
        } else {
            $out.push_str("<Could not downcast>");
        }
        $out.push(')');
    }};
}

impl ReflectReferencePrinter {
    const UNREGISTERED_TYPE: &'static str = "Unregistered";
    const UNKNOWN_FIELD: &'static str = "<Unknown Field>";

    /// Creates a new reflect reference printer
    pub fn new(reference: ReflectReference) -> Self {
        Self { reference }
    }

    /// Given a reflect reference, prints the type path of the reference resolving the type names with short names.
    /// I.e. `MyType(Component).field_name[0].field_name[1] -> FieldType::Name`
    pub fn pretty_print(&self, world: Option<WorldGuard>) -> String {
        let mut pretty_path = String::new();

        pretty_path.push_str("<Reference to ");
        if let Some(world) = world {
            let tail_type_id = self.reference.tail_type_id(world.clone()).ok().flatten();

            Self::pretty_print_base(&self.reference.base, Some(world.clone()), &mut pretty_path);

            pretty_path.push_str(&self.reference.reflect_path.to_string());

            if let Some(tail_type_id) = tail_type_id {
                let type_path = tail_type_id.display_with_world(world);
                pretty_path.push_str(&format!(" -> {type_path}"));
            }
        } else {
            Self::pretty_print_base(&self.reference.base, None, &mut pretty_path);
        }
        pretty_path.push('>');
        pretty_path
    }

    /// Prints the actual value of the reference. Tries to use best available method to print the value.
    pub fn pretty_print_value(&self, world: Option<WorldGuard>) -> String {
        let mut output = String::new();

        match world {
            Some(world) => {
                // instead of relying on type registrations, simply traverse the reflection tree and print sensible values
                self.reference
                    .with_reflect(world, |r| {
                        self.pretty_print_value_inner(r, &mut output);
                    })
                    .unwrap_or_else(|e| {
                        output.push_str(&format!("<Error in printing: {e}>"));
                    });
            }
            None => {
                output.push_str("<Referenced Value>");
            }
        }

        output
    }

    fn pretty_print_base(base: &ReflectBaseType, world: Option<WorldGuard>, out: &mut String) {
        let type_id = base.type_id();
        let type_path = if let Some(world) = world {
            type_id.display_with_world(world.clone())
        } else {
            type_id.display_without_world()
        };

        let base_kind = match base.base_id {
            ReflectBase::Component(e, _) => format!("Component on entity {e}"),
            ReflectBase::Resource(_) => "Resource".to_owned(),
            ReflectBase::Owned(ref id) => format!("Allocation({id})"),
        };

        out.push_str(&format!("{base_kind}({type_path})"));
    }

    /// Pretty prints a value of an opaque type.
    pub fn pretty_print_value_opaque(&self, v: &dyn PartialReflect, output: &mut String) {
        let type_id = v
            .get_represented_type_info()
            .map(|t| t.type_id())
            .or_fake_id();

        output.push_str("Reflect(");
        match type_id {
            id if id == TypeId::of::<usize>() => downcast_case!(v, output, usize),
            id if id == TypeId::of::<isize>() => downcast_case!(v, output, isize),
            id if id == TypeId::of::<f32>() => downcast_case!(v, output, f32),
            id if id == TypeId::of::<f64>() => downcast_case!(v, output, f64),
            id if id == TypeId::of::<u128>() => downcast_case!(v, output, u128),
            id if id == TypeId::of::<u64>() => downcast_case!(v, output, u64),
            id if id == TypeId::of::<u32>() => downcast_case!(v, output, u32),
            id if id == TypeId::of::<u16>() => downcast_case!(v, output, u16),
            id if id == TypeId::of::<u8>() => downcast_case!(v, output, u8),
            id if id == TypeId::of::<i128>() => downcast_case!(v, output, i128),
            id if id == TypeId::of::<i64>() => downcast_case!(v, output, i64),
            id if id == TypeId::of::<i32>() => downcast_case!(v, output, i32),
            id if id == TypeId::of::<i16>() => downcast_case!(v, output, i16),
            id if id == TypeId::of::<i8>() => downcast_case!(v, output, i8),
            id if id == TypeId::of::<String>() => downcast_case!(v, output, String),
            id if id == TypeId::of::<std::path::PathBuf>() => {
                downcast_case!(v, output, std::path::PathBuf)
            }
            id if id == TypeId::of::<std::ffi::OsString>() => {
                downcast_case!(v, output, std::ffi::OsString)
            }
            id if id == TypeId::of::<Cow<str>>() => {
                downcast_case!(v, output, Cow<str>)
            }
            id if id == TypeId::of::<char>() => downcast_case!(v, output, char),
            id if id == TypeId::of::<bool>() => downcast_case!(v, output, bool),
            id if id == TypeId::of::<ScriptValue>() => downcast_case!(v, output, ScriptValue),
            _ => {
                output.push_str(
                    v.get_represented_type_info()
                        .map(|t| t.type_path())
                        .unwrap_or(Self::UNREGISTERED_TYPE),
                );
            }
        }
        output.push(')');
    }

    fn pretty_print_key_values<
        K: AsRef<str>,
        V: AsRef<str>,
        I: IntoIterator<Item = (Option<K>, V)>,
    >(
        bracket: BracketType,
        i: I,
        output: &mut String,
    ) {
        bracket.surrounded(output, |output| {
            let mut iter = i.into_iter().peekable();
            while let Some((key, val)) = iter.next() {
                if let Some(key) = key {
                    output.push_str(key.as_ref());
                    output.push_str(": ");
                }
                output.push_str(val.as_ref());
                if iter.peek().is_some() {
                    output.push_str(", ");
                }
            }
        });
    }

    fn pretty_print_value_struct<
        'k,
        N: Iterator<Item = &'k str>,
        M: Iterator<Item = &'k dyn PartialReflect>,
    >(
        &self,
        field_names: N,
        field_values: M,
        output: &mut String,
    ) {
        let field_names = field_names.into_iter();
        let fields = field_values.into_iter();
        let fields_iter = fields.zip_longest(field_names).map(|e| {
            let (val, name) = match e {
                itertools::EitherOrBoth::Both(val, name) => (val, name),
                itertools::EitherOrBoth::Left(val) => (val, Self::UNKNOWN_FIELD),
                itertools::EitherOrBoth::Right(name) => (().as_partial_reflect(), name),
            };
            let mut field_printed = String::new();
            self.pretty_print_value_inner(val, &mut field_printed);
            (Some(name), field_printed)
        });
        Self::pretty_print_key_values(BracketType::Curly, fields_iter, output);
    }

    fn pretty_print_value_inner(&self, v: &dyn PartialReflect, output: &mut String) {
        match v.reflect_ref() {
            ReflectRef::Struct(s) => {
                let field_names = s
                    .get_represented_struct_info()
                    .map(|info| info.field_names())
                    .unwrap_or_default()
                    .iter();
                let field_values = s.iter_fields();

                self.pretty_print_value_struct(field_names.copied(), field_values, output);
            }
            ReflectRef::TupleStruct(t) => {
                let fields_iter = t.iter_fields().enumerate().map(|(i, val)| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (Some(i.to_string()), field_printed)
                });
                Self::pretty_print_key_values(BracketType::Round, fields_iter, output);
            }
            ReflectRef::Tuple(t) => {
                let fields_iter = t.iter_fields().map(|val| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (None::<String>, field_printed)
                });
                Self::pretty_print_key_values(BracketType::Round, fields_iter, output);
            }
            ReflectRef::List(l) => {
                let fields_iter = l.iter().map(|val| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (None::<String>, field_printed)
                });
                Self::pretty_print_key_values(BracketType::Square, fields_iter, output);
            }
            ReflectRef::Array(a) => {
                let fields_iter = a.iter().map(|val| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (None::<String>, field_printed)
                });
                Self::pretty_print_key_values(BracketType::Square, fields_iter, output);
            }
            ReflectRef::Map(m) => {
                let fields_iter = m.iter().map(|(key, val)| {
                    let mut key_printed = String::new();
                    self.pretty_print_value_inner(key, &mut key_printed);

                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (Some(key_printed), field_printed)
                });
                Self::pretty_print_key_values(BracketType::Curly, fields_iter, output);
            }
            ReflectRef::Set(s) => {
                let fields_iter = s.iter().map(|val| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(val, &mut field_printed);
                    (None::<String>, field_printed)
                });
                Self::pretty_print_key_values(BracketType::Curly, fields_iter, output);
            }
            ReflectRef::Enum(e) => {
                output.push_str(&e.variant_path());
                let bracket_type = match e.variant_type() {
                    VariantType::Tuple => BracketType::Round,
                    _ => BracketType::Curly,
                };
                let key_vals = e.iter_fields().map(|v| {
                    let mut field_printed = String::new();
                    self.pretty_print_value_inner(v.value(), &mut field_printed);
                    (v.name(), field_printed)
                });
                Self::pretty_print_key_values(bracket_type, key_vals, output);
            }
            ReflectRef::Opaque(o) => {
                self.pretty_print_value_opaque(o, output);
            }
            // for function_reflection from bevy or other feature gated things
            #[allow(unreachable_patterns)]
            _ => {
                output.push_str(&format!("{v:?}"));
            }
        }
    }
}

// /// Alais for [`DisplayWithWorldAndDummy`] + [`std::fmt::Display`], ideally display should warn that it's not the full representation.
// pub trait DisplayWithWorldAndDummy: DisplayWithWorld + std::fmt::Display {}
// impl<T: DisplayWithWorld + std::fmt::Display> DisplayWithWorldAndDummy for T {}

/// For types which can't be pretty printed without world access.
/// Implementors should try to print the best value they can, and never panick.
pub trait DisplayWithWorld: std::fmt::Debug + AsAny {
    /// # Warning
    /// Display this type without world access. It is not recommended to use this method for anything other than debugging or necessary trait impl corners.
    /// For many types this will just print type id's with no further information.
    ///
    /// Prefer using [`DisplayWithWorld::display_with_world`] or [`DisplayWithWorld::display_value_with_world`] instead.
    fn display_without_world(&self) -> String;

    /// Display the `shallowest` representation of the type using world access.
    /// For references this is the type path and the type of the value they are pointing to.
    fn display_with_world(&self, world: WorldGuard) -> String;

    /// Display the most literal representation of the type using world access.
    /// I.e. for references this would be the pointed to value itself.
    fn display_value_with_world(&self, world: WorldGuard) -> String {
        self.display_with_world(world)
    }
}

#[doc(hidden)]
pub trait AsAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

#[doc(hidden)]
impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl dyn DisplayWithWorld {
    /// Downcasts the `DisplayWithWorld` trait object to a concrete type.
    /// Trampoline function to allow downcasting of errors.
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[profiling::all_functions]
impl DisplayWithWorld for ReflectReference {
    fn display_with_world(&self, world: WorldGuard) -> String {
        ReflectReferencePrinter::new(self.clone()).pretty_print(Some(world))
    }

    fn display_value_with_world(&self, world: WorldGuard) -> String {
        ReflectReferencePrinter::new(self.clone()).pretty_print_value(Some(world))
    }

    fn display_without_world(&self) -> String {
        ReflectReferencePrinter::new(self.clone()).pretty_print(None)
    }
}
#[profiling::all_functions]
impl DisplayWithWorld for ReflectBaseType {
    fn display_with_world(&self, world: WorldGuard) -> String {
        let mut string = String::new();
        ReflectReferencePrinter::pretty_print_base(self, Some(world), &mut string);
        string
    }

    fn display_value_with_world(&self, world: WorldGuard) -> String {
        self.display_with_world(world)
    }

    fn display_without_world(&self) -> String {
        let mut string = String::new();
        ReflectReferencePrinter::pretty_print_base(self, None, &mut string);
        string
    }
}
#[profiling::all_functions]
impl DisplayWithWorld for ComponentId {
    fn display_without_world(&self) -> String {
        format!("ComponentOrResource({})", self.index())
    }

    fn display_with_world(&self, world: WorldGuard) -> String {
        let component_name = world
            .as_unsafe_world_cell()
            .ok()
            .and_then(|c| c.components().get_info(*self))
            .map(|info| info.name());

        match component_name {
            Some(n) => format!("ComponentOrResource({n})"),
            None => "ComponentOrResource(<Unknown>)".to_owned(),
        }
    }
}
#[profiling::all_functions]
impl DisplayWithWorld for ReflectAccessId {
    fn display_without_world(&self) -> String {
        match self.kind {
            super::access_map::ReflectAccessKind::ComponentOrResource => {
                let component_id = ComponentId::from(*self);
                component_id.display_without_world()
            }
            super::access_map::ReflectAccessKind::Allocation => {
                format!("Allocation({})", self.id)
            }
            super::access_map::ReflectAccessKind::Global => "Global".to_owned(),
        }
    }

    fn display_with_world(&self, world: WorldGuard) -> String {
        match self.kind {
            super::access_map::ReflectAccessKind::ComponentOrResource => {
                let component_id = ComponentId::from(*self);
                component_id.display_with_world(world)
            }
            super::access_map::ReflectAccessKind::Allocation => {
                let allocation_id = ReflectAllocationId::from(*self);
                let allocator = world.allocator();
                let allocator = allocator.read();
                let raid = ReflectAccessId::for_allocation(allocation_id.clone());

                if world.claim_read_access(raid) {
                    if let Some(allocation) = allocator.get(&allocation_id) {
                        let ptr = allocation.get_ptr();
                        let val = unsafe { &*ptr };
                        let o = format!("Allocation({val:?})");
                        unsafe { world.release_access(raid) };
                        o
                    } else {
                        format!("Allocation({allocation_id})")
                    }
                } else {
                    format!("Allocation({allocation_id})")
                }
            }
            super::access_map::ReflectAccessKind::Global => "Global".to_owned(),
        }
    }
}
#[profiling::all_functions]
impl DisplayWithWorld for TypeId {
    fn display_with_world(&self, world: WorldGuard) -> String {
        if *self == TypeId::of::<FakeType>() {
            return "Unknown Type".to_owned();
        } else if *self == TypeId::of::<World>() {
            // does not implement Reflect, so we do this manually
            return "World".to_owned();
        }

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        type_registry
            .get_type_info(*self)
            .map(|t| t.type_path_table().path().to_owned())
            .unwrap_or_else(|| {
                format!("{}({:?})", ReflectReferencePrinter::UNREGISTERED_TYPE, self)
            })
            .to_string()
    }

    fn display_value_with_world(&self, world: WorldGuard) -> String {
        self.display_with_world(world)
    }

    fn display_without_world(&self) -> String {
        format!("{self:?}")
    }
}
#[profiling::all_functions]
impl DisplayWithWorld for ScriptValue {
    fn display_with_world(&self, world: WorldGuard) -> String {
        match self {
            ScriptValue::Reference(r) => r.display_with_world(world),
            _ => self.display_value_with_world(world),
        }
    }

    fn display_value_with_world(&self, world: WorldGuard) -> String {
        match self {
            ScriptValue::Reference(r) => r.display_value_with_world(world),
            ScriptValue::FunctionMut(f) => format!("FunctionMut({})", f.name()),
            ScriptValue::Function(f) => format!("Function({})", f.name()),
            ScriptValue::Unit => "()".to_owned(),
            ScriptValue::Bool(b) => b.to_string(),
            ScriptValue::Integer(i) => i.to_string(),
            ScriptValue::Float(f) => f.to_string(),
            ScriptValue::String(cow) => cow.to_string(),
            ScriptValue::Error(script_error) => script_error.display_with_world(world),
            ScriptValue::List(vec) => vec.display_with_world(world),
            ScriptValue::Map(hash_map) => hash_map.display_with_world(world),
        }
    }

    fn display_without_world(&self) -> String {
        match self {
            ScriptValue::Unit => "()".to_owned(),
            ScriptValue::Bool(b) => b.to_string(),
            ScriptValue::Integer(i) => i.to_string(),
            ScriptValue::Float(f) => f.to_string(),
            ScriptValue::String(cow) => cow.to_string(),
            ScriptValue::List(vec) => {
                let mut string = String::new();
                ReflectReferencePrinter::pretty_print_key_values(
                    BracketType::Square,
                    vec.iter()
                        .map(|v| (None::<String>, v.display_without_world())),
                    &mut string,
                );
                string
            }
            ScriptValue::Reference(reflect_reference) => reflect_reference.display_without_world(),
            ScriptValue::FunctionMut(dynamic_script_function_mut) => {
                format!("Function({})", dynamic_script_function_mut.name())
            }
            ScriptValue::Function(dynamic_script_function) => {
                format!("Function({})", dynamic_script_function.name())
            }
            ScriptValue::Error(interop_error) => interop_error.display_without_world(),
            ScriptValue::Map(hash_map) => hash_map.display_without_world(),
        }
    }
}
#[profiling::all_functions]
impl<T: DisplayWithWorld + 'static> DisplayWithWorld for Vec<T> {
    fn display_with_world(&self, world: WorldGuard) -> String {
        let mut string = String::new();
        BracketType::Square.surrounded(&mut string, |string| {
            for (i, v) in self.iter().enumerate() {
                string.push_str(&v.display_with_world(world.clone()));
                if i != self.len() - 1 {
                    string.push_str(", ");
                }
            }
        });
        string
    }

    fn display_value_with_world(&self, world: WorldGuard) -> String {
        let mut string = String::new();
        BracketType::Square.surrounded(&mut string, |string| {
            for (i, v) in self.iter().enumerate() {
                string.push_str(&v.display_value_with_world(world.clone()));
                if i != self.len() - 1 {
                    string.push_str(", ");
                }
            }
        });
        string
    }

    fn display_without_world(&self) -> String {
        let mut string = String::new();
        BracketType::Square.surrounded(&mut string, |string| {
            for (i, v) in self.iter().enumerate() {
                string.push_str(&v.display_without_world());
                if i != self.len() - 1 {
                    string.push_str(", ");
                }
            }
        });
        string
    }
}
#[profiling::all_functions]
impl DisplayWithWorld for String {
    fn display_with_world(&self, _world: WorldGuard) -> String {
        self.to_string()
    }

    fn display_value_with_world(&self, _world: WorldGuard) -> String {
        self.to_string()
    }

    fn display_without_world(&self) -> String {
        self.to_string()
    }
}

/// Implements DisplayWithWorld for a HashMap-like type that has key-value pairs
/// and supports iter() and len() methods.
macro_rules! impl_display_with_world_for_map {
    ($map_type:path) => {
        #[profiling::all_functions]
        impl<K: DisplayWithWorld + 'static, V: DisplayWithWorld + 'static> DisplayWithWorld
            for $map_type
        {
            fn display_with_world(&self, world: WorldGuard) -> String {
                let mut string = String::new();
                BracketType::Curly.surrounded(&mut string, |string| {
                    for (i, (k, v)) in self.iter().enumerate() {
                        string.push_str(&k.display_with_world(world.clone()));
                        string.push_str(": ");
                        string.push_str(&v.display_with_world(world.clone()));
                        if i != self.len() - 1 {
                            string.push_str(", ");
                        }
                    }
                });
                string
            }

            fn display_value_with_world(&self, world: WorldGuard) -> String {
                let mut string = String::new();
                BracketType::Curly.surrounded(&mut string, |string| {
                    for (i, (k, v)) in self.iter().enumerate() {
                        string.push_str(&k.display_value_with_world(world.clone()));
                        string.push_str(": ");
                        string.push_str(&v.display_value_with_world(world.clone()));
                        if i != self.len() - 1 {
                            string.push_str(", ");
                        }
                    }
                });
                string
            }

            fn display_without_world(&self) -> String {
                let mut string = String::new();
                BracketType::Curly.surrounded(&mut string, |string| {
                    for (i, (k, v)) in self.iter().enumerate() {
                        string.push_str(&k.display_without_world());
                        string.push_str(": ");
                        string.push_str(&v.display_without_world());
                        if i != self.len() - 1 {
                            string.push_str(", ");
                        }
                    }
                });
                string
            }
        }
    };
}

impl_display_with_world_for_map!(HashMap<K, V>);
impl_display_with_world_for_map!(std::collections::HashMap<K, V>);

#[cfg(test)]
mod test {
    use bevy_ecs::reflect::AppTypeRegistry;
    use bevy_reflect::Reflect;

    use crate::bindings::{
        AppReflectAllocator, ReflectAllocationId,
        function::script_function::AppScriptFunctionRegistry,
    };

    use super::*;

    fn setup_world() -> World {
        let mut world = World::default();

        let type_registry = AppTypeRegistry::default();
        world.insert_resource(type_registry);

        let allocator = AppReflectAllocator::default();
        world.insert_resource(allocator);

        let script_function_registry = AppScriptFunctionRegistry::default();
        world.insert_resource(script_function_registry);

        world
    }

    #[test]
    fn test_type_id() {
        let mut world = setup_world();
        let world = WorldGuard::new_exclusive(&mut world);

        let type_id = TypeId::of::<usize>();
        assert_eq!(type_id.display_with_world(world.clone()), "usize");
        assert_eq!(type_id.display_value_with_world(world.clone()), "usize");
        assert_eq!(type_id.display_without_world(), format!("{type_id:?}"));

        let type_id = TypeId::of::<FakeType>();
        assert_eq!(type_id.display_with_world(world.clone()), "Unknown Type");
        assert_eq!(
            type_id.display_value_with_world(world.clone()),
            "Unknown Type"
        );
        assert_eq!(type_id.display_without_world(), format!("{type_id:?}"));
    }

    #[test]
    fn test_reflect_base_type() {
        let mut world = setup_world();
        let world = WorldGuard::new_exclusive(&mut world);

        let type_id = TypeId::of::<usize>();

        assert_eq!(
            ReflectBaseType {
                base_id: ReflectBase::Owned(ReflectAllocationId::new(0)),
                type_id,
            }
            .display_with_world(world.clone()),
            "Allocation(0)(usize)"
        );

        assert_eq!(
            ReflectBaseType {
                base_id: ReflectBase::Owned(ReflectAllocationId::new(0)),
                type_id,
            }
            .display_value_with_world(world.clone()),
            "Allocation(0)(usize)"
        );

        assert_eq!(
            ReflectBaseType {
                base_id: ReflectBase::Owned(ReflectAllocationId::new(0)),
                type_id,
            }
            .display_without_world(),
            format!("Allocation(0)({type_id:?})")
        );
    }

    #[test]
    fn test_reflect_reference() {
        let mut world = setup_world();

        let world = WorldGuard::new_exclusive(&mut world);

        let type_id = TypeId::of::<usize>();

        let allocator = world.allocator();
        let mut allocator_write = allocator.write();
        let reflect_reference = ReflectReference::new_allocated(2usize, &mut allocator_write);
        let id = match reflect_reference.base.base_id {
            ReflectBase::Owned(ref id) => id.to_string(),
            _ => panic!("Expected owned allocation"),
        };

        drop(allocator_write);

        assert_eq!(
            reflect_reference.display_with_world(world.clone()),
            format!("<Reference to Allocation({id})(usize) -> usize>")
        );

        assert_eq!(
            reflect_reference.display_value_with_world(world.clone()),
            "Reflect(usize(2))"
        );

        assert_eq!(
            reflect_reference.display_without_world(),
            format!("<Reference to Allocation({id})({type_id:?})>")
        );
    }

    #[test]
    fn test_hashmap() {
        let mut world = setup_world();
        let world = WorldGuard::new_exclusive(&mut world);

        let mut map = HashMap::new();
        map.insert("hello".to_owned(), ScriptValue::Bool(true));

        assert_eq!(map.display_with_world(world.clone()), "{hello: true}");

        assert_eq!(map.display_value_with_world(world.clone()), "{hello: true}");
    }

    #[test]
    fn test_script_value_in_reference() {
        let mut world = setup_world();
        let world = WorldGuard::new_exclusive(&mut world);

        #[derive(Reflect)]
        struct Test {
            val: ScriptValue,
        }

        let test = Test {
            val: ScriptValue::Bool(true),
        };

        let allocator = world.allocator();
        let mut allocator_write = allocator.write();

        let reflect_reference = ReflectReference::new_allocated(test, &mut allocator_write);
        drop(allocator_write);
        assert_eq!(
            reflect_reference.display_value_with_world(world.clone()),
            "{val: Reflect(ScriptValue(Bool(true)))}"
        );
    }
}
