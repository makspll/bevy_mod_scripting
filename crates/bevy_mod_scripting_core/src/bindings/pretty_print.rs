use crate::reflection_extensions::TypeIdExtensions;

use super::{
    script_value::ScriptValue, ReflectBase, ReflectBaseType, ReflectReference, WorldGuard,
};
use bevy::reflect::{PartialReflect, ReflectRef};
use itertools::Itertools;
use std::{any::TypeId, borrow::Cow};

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
    const UNREGISTERED_TYPE: &'static str = "Unregistered TypeId";
    const UNKNOWN_FIELD: &'static str = "<Unknown Field>";

    pub fn new(reference: ReflectReference) -> Self {
        Self { reference }
    }

    fn pretty_print_base(base: &ReflectBaseType, world: WorldGuard, out: &mut String) {
        let type_id = base.type_id;
        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        let type_path = type_registry
            .get_type_info(type_id)
            .map(|t| t.type_path_table().short_path())
            .unwrap_or_else(|| ReflectReferencePrinter::UNREGISTERED_TYPE);

        let base_kind = match base.base_id {
            ReflectBase::Component(e, _) => format!("Component on entity {}", e),
            ReflectBase::Resource(_) => "Resource".to_owned(),
            ReflectBase::Owned(_) => "Allocation".to_owned(),
            ReflectBase::World => "World".to_owned(),
        };

        out.push_str(&format!("{}({})", base_kind, type_path));
    }

    /// Given a reflect reference, prints the type path of the reference resolving the type names with short names.
    /// I.e. `MyType(Component).field_name[0].field_name[1] -> FieldType::Name`
    pub fn pretty_print(&self, world: WorldGuard) -> String {
        let mut pretty_path = String::new();

        pretty_path.push_str("<Reference to ");

        let tail_type_id = self.reference.tail_type_id(world.clone()).ok().flatten();
        let type_registry = world.type_registry();

        Self::pretty_print_base(&self.reference.base, world.clone(), &mut pretty_path);

        pretty_path.push_str(&self.reference.reflect_path.to_string());

        if let Some(tail_type_id) = tail_type_id {
            let type_path = {
                let type_registry = type_registry.read();
                type_registry
                    .get_type_info(tail_type_id)
                    .map(|t| t.type_path_table().short_path())
                    .unwrap_or(Self::UNREGISTERED_TYPE)
            };
            pretty_path.push_str(&format!(" -> {}", type_path));
        }

        pretty_path.push('>');

        pretty_path
    }

    pub fn pretty_print_value_opaque(&self, v: &dyn PartialReflect, output: &mut String) {
        let type_id = v
            .get_represented_type_info()
            .map(|t| t.type_id())
            .type_id_or_fake_id();

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
            id if id == TypeId::of::<bool>() => downcast_case!(v, output, bool),
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

    /// Prints the actual value of the reference. Tries to use best available method to print the value.
    pub fn pretty_print_value(&self, world: WorldGuard) -> String {
        let mut output = String::new();

        // instead of relying on type registrations, simply traverse the reflection tree and print sensible values
        self.reference
            .with_reflect(world, |r| {
                self.pretty_print_value_inner(r, &mut output);
            })
            .unwrap_or_else(|e| {
                output.push_str(&format!("<Error in printing: {}>", e));
            });

        output
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
            bevy::reflect::ReflectRef::Struct(s) => {
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
                    bevy::reflect::VariantType::Tuple => BracketType::Round,
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
            ReflectRef::Function(f) => {
                output.push_str("Function(");
                output.push_str(
                    f.info()
                        .name()
                        .unwrap_or(&Cow::Borrowed("<unnamed function>"))
                        .as_ref(),
                );
                output.push(')');
            }
        }
    }
}

/// Alais for [`DisplayWithWorldAndDummy`] + [`std::fmt::Display`], ideally display should warn that it's not the full representation.
pub trait DisplayWithWorldAndDummy: DisplayWithWorld + std::fmt::Display {}
impl<T: DisplayWithWorld + std::fmt::Display> DisplayWithWorldAndDummy for T {}

/// For types which can't be pretty printed without world access.
/// Implementors should try to print the best value they can, and never panick.
pub trait DisplayWithWorld: std::fmt::Debug {
    /// Display the `shallowest` representation of the type using world access.
    /// For references this is the type path and the type of the value they are pointing to.
    fn display_with_world(&self, world: WorldGuard) -> String;

    /// Display the most literal representation of the type using world access.
    /// I.e. for references this would be the pointed to value itself.
    fn display_value_with_world(&self, world: WorldGuard) -> String {
        self.display_with_world(world)
    }
}

#[macro_export]
macro_rules! impl_dummy_display (
    ($t:ty) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Displaying {} without world access: {:#?}", stringify!($t), self)?;
                Ok(())
            }
        }
    };
);

impl_dummy_display!(ReflectReference);

impl DisplayWithWorld for ReflectReference {
    fn display_with_world(&self, world: WorldGuard) -> String {
        ReflectReferencePrinter::new(self.clone()).pretty_print(world)
    }

    fn display_value_with_world(&self, world: WorldGuard) -> String {
        ReflectReferencePrinter::new(self.clone()).pretty_print_value(world)
    }
}

impl_dummy_display!(ReflectBaseType);

impl DisplayWithWorld for ReflectBaseType {
    fn display_with_world(&self, world: WorldGuard) -> String {
        let mut string = String::new();
        ReflectReferencePrinter::pretty_print_base(self, world, &mut string);
        string
    }

    fn display_value_with_world(&self, world: WorldGuard) -> String {
        self.display_with_world(world)
    }
}

impl DisplayWithWorld for TypeId {
    fn display_with_world(&self, world: WorldGuard) -> String {
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
}

impl_dummy_display!(ScriptValue);

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
            ScriptValue::Unit => "()".to_owned(),
            ScriptValue::Bool(b) => b.to_string(),
            ScriptValue::Integer(i) => i.to_string(),
            ScriptValue::Float(f) => f.to_string(),
            ScriptValue::String(cow) => cow.to_string(),
            ScriptValue::World => "World".to_owned(),
            ScriptValue::Error(script_error) => script_error.to_string(),
            ScriptValue::List(vec) => {
                let mut string = String::new();
                ReflectReferencePrinter::pretty_print_key_values(
                    BracketType::Square,
                    vec.iter()
                        .map(|v| (None::<String>, v.display_value_with_world(world.clone()))),
                    &mut string,
                );
                string
            }
        }
    }
}
