//! The layouts of components and resources which languages read and write in place, through views (see
//! [`bevy_mod_scripting_world::RowView`]).
//!
//! Rust doesn't specify the field order of a struct without a `repr`, but every value of a type has the same layout in one
//! binary: the offsets of its fields are measured once, on a live value, through reflection, and validated.

use std::{
    alloc::Layout,
    any::TypeId,
    collections::HashMap,
    ptr::NonNull,
    sync::{
        LazyLock,
        atomic::{AtomicUsize, Ordering},
    },
};

use bevy_ecs::ptr::Ptr;
use bevy_reflect::{PartialReflect, ReflectFromPtr, ReflectRef, TypeInfo, TypeRegistry};
use parking_lot::RwLock;

use crate::{
    CallScope, GetTypeDependencies, InteropError, IntoScript, ReflectReference, ScriptValue,
    WorldGuard,
    native::{FfiType, NativeReturnType, ReturnSink},
};

/// A whole component or resource returned to a language which may return it as a view (see [`ReturnSink::view`]), or
/// else as a reference. Views go stale when the world changes structurally or the callback returns, unlike references:
/// only functions whose results are used right away return them (`world.get_component`).
pub struct ViewRef(pub ReflectReference);

impl IntoScript for ViewRef {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }

    fn into_ret(
        self,
        ret: &mut dyn ReturnSink,
        scope: &CallScope,
        native: &NativeReturnType,
    ) -> Result<(), InteropError> {
        if self.0.reflect_path.is_empty() && ret.view(&self.0, scope)? {
            return Ok(());
        }
        self.0.into_ret(ret, scope, native)
    }
}

impl GetTypeDependencies for ViewRef {
    type Underlying = ReflectReference;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        ReflectReference::register_type_dependencies(registry)
    }
}

/// The fields of a struct which views read and write in place: the fields whose type has a C form, at their offsets.
/// Every other field is read and written by reflection. Layouts live for the program: languages key their view types
/// by their address.
#[derive(Debug)]
pub struct ViewLayout {
    /// The type
    pub type_id: TypeId,
    /// The type's path, for messages
    pub type_name: &'static str,
    /// The size of a value
    pub size: usize,
    /// The alignment of a value
    pub align: usize,
    /// The fields read and written in place, ordered by offset, not overlapping
    pub fields: Vec<ViewField>,
    /// The address of the value it was measured on, `usize::MAX` once checked on another one, see [`view_layout`]
    measured_at: AtomicUsize,
}

/// A field of a [`ViewLayout`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewField {
    /// The field's name, a C identifier
    pub name: &'static str,
    /// The offset in a value
    pub offset: usize,
    /// The C type: a scalar, or a native type with an [`crate::native::FfiLayout`]
    pub ty: FfiType,
    /// The size of the field
    pub size: usize,
}

/// The scalars views read and write in place, with their C types: the ones where every bit pattern is a valid value, and
/// `bool` (views only write `0` or `1` to it).
const SCALARS: &[(fn() -> TypeId, &str, usize)] = &[
    (TypeId::of::<f32>, "float", 4),
    (TypeId::of::<f64>, "double", 8),
    (TypeId::of::<i8>, "int8_t", 1),
    (TypeId::of::<i16>, "int16_t", 2),
    (TypeId::of::<i32>, "int32_t", 4),
    (TypeId::of::<i64>, "int64_t", 8),
    (TypeId::of::<isize>, "intptr_t", size_of::<isize>()),
    (TypeId::of::<u8>, "uint8_t", 1),
    (TypeId::of::<u16>, "uint16_t", 2),
    (TypeId::of::<u32>, "uint32_t", 4),
    (TypeId::of::<u64>, "uint64_t", 8),
    (TypeId::of::<usize>, "uintptr_t", size_of::<usize>()),
    (TypeId::of::<bool>, "bool", 1),
];

/// The C scalar of `type_id`, and its size (which is its alignment), see [`SCALARS`].
fn scalar(type_id: TypeId) -> Option<(&'static str, usize)> {
    SCALARS
        .iter()
        .find(|(id, _, _)| id() == type_id)
        .map(|(_, c, size)| (*c, *size))
}

/// Whether `name` can name a field of a C struct views declare.
fn is_view_field_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Measured layouts, `None` for types without views. Layouts are fixed per type in a binary.
static LAYOUTS: LazyLock<RwLock<HashMap<TypeId, Option<&'static ViewLayout>>>> =
    LazyLock::new(Default::default);

/// The view layout of `type_id`, measured on the value at `value` the first time, `None` if it has no field a view reads
/// in place. `natives` gives the size and alignment of native types with a C layout.
///
/// Checks, each failing one leaves the field (or the type) to reflection:
/// - D1: the type's [`ReflectFromPtr`] is of `type_id`, it's a struct
/// - D2: the field is inside the value
/// - D3: the field is aligned, and overlaps no other one
/// - D4: a value at another address has the same offsets (checked once, on the first other value): else the type
///   has no views from then on
///
/// # Safety
/// `value` is a live value of `type_id`, of `layout`, not mutably borrowed during the call.
pub unsafe fn view_layout(
    registry: &TypeRegistry,
    natives: &dyn Fn(TypeId) -> Option<(usize, usize)>,
    type_id: TypeId,
    value: *const u8,
    layout: Layout,
) -> Option<&'static ViewLayout> {
    let known = LAYOUTS.read().get(&type_id).copied();
    match known {
        Some(None) => None,
        Some(Some(known)) => {
            let measured_at = known.measured_at.load(Ordering::Relaxed);
            if measured_at == usize::MAX || measured_at == value as usize {
                return Some(known);
            }
            // D4: once, on another value
            // Safety: see the function's contract
            let again = unsafe { measure(registry, natives, type_id, value, layout) };
            if again.is_some_and(|again| again.fields == known.fields) {
                known.measured_at.store(usize::MAX, Ordering::Relaxed);
                return Some(known);
            }
            bevy_log::error!(
                "the fields of {} have other offsets in another value: it gets no views",
                known.type_name
            );
            LAYOUTS.write().insert(type_id, None);
            None
        }
        None => {
            // Safety: see the function's contract
            let measured = unsafe { measure(registry, natives, type_id, value, layout) };
            // leaked: once per type, see `ViewLayout`
            let measured = measured.map(|m| &*Box::leak(Box::new(m)));
            LAYOUTS.write().insert(type_id, measured);
            measured
        }
    }
}

/// Measures the layout, see [`view_layout`]. `None` if no field is read in place.
///
/// # Safety
/// See [`view_layout`].
unsafe fn measure(
    registry: &TypeRegistry,
    natives: &dyn Fn(TypeId) -> Option<(usize, usize)>,
    type_id: TypeId,
    value: *const u8,
    layout: Layout,
) -> Option<ViewLayout> {
    let registration = registry.get(type_id)?;
    // D1
    let from_ptr = registration.data::<ReflectFromPtr>()?;
    if from_ptr.type_id() != type_id {
        return None;
    }
    let TypeInfo::Struct(info) = registration.type_info() else {
        return None;
    };
    // Safety: a live value of `type_id`, see the function's contract
    let reflected = unsafe { from_ptr.ptr_as_reflect(Ptr::new(NonNull::new(value.cast_mut())?)) };
    let ReflectRef::Struct(reflected) = reflected.reflect_ref() else {
        return None;
    };
    let base = value as usize;
    let mut fields = Vec::new();
    for (i, field_info) in info.iter().enumerate() {
        let name = field_info.name();
        if !is_view_field_name(name) {
            continue;
        }
        let field_type = field_info.type_id();
        let (ty, size, align) = match (scalar(field_type), natives(field_type)) {
            (Some((c, size)), _) => (FfiType::Scalar(c), size, size),
            (None, Some((size, align))) => (FfiType::Value(field_type), size, align),
            (None, None) => continue,
        };
        let Some(field) = reflected.field_at(i) else {
            continue;
        };
        let address = field as *const dyn PartialReflect as *const u8 as usize;
        let offset = address.wrapping_sub(base);
        // D2
        if address < base
            || offset
                .checked_add(size)
                .is_none_or(|end| end > layout.size())
        {
            continue;
        }
        // D3, overlaps are dropped below
        if !offset.is_multiple_of(align) {
            continue;
        }
        fields.push(ViewField {
            name,
            offset,
            ty,
            size,
        });
    }
    fields.sort_by_key(|f| f.offset);
    // D3: no overlaps, the later field is dropped
    let mut end = 0;
    fields.retain(|f| {
        let keep = f.offset >= end;
        if keep {
            end = f.offset + f.size;
        }
        keep
    });
    (!fields.is_empty()).then(|| ViewLayout {
        type_id,
        type_name: info.type_path(),
        size: layout.size(),
        align: layout.align(),
        fields,
        measured_at: AtomicUsize::new(base),
    })
}

#[cfg(test)]
mod test {
    use std::{alloc::Layout, any::TypeId, mem::offset_of};

    use bevy_reflect::{GetTypeRegistration, Reflect, TypeRegistry};

    use super::{FfiType, view_layout};

    #[derive(Reflect, Default)]
    struct Mixed {
        flag: bool,
        name: String,
        x: f64,
        small: u8,
        list: Vec<u32>,
        y: f32,
        maybe: Option<f32>,
        native: [f32; 3],
    }

    #[derive(Reflect, Default)]
    struct Tuple(f32, f32);

    #[derive(Reflect, Default)]
    struct Empty;

    fn layout_of<T: GetTypeRegistration + Default + 'static>(
        value: &T,
        natives: &dyn Fn(TypeId) -> Option<(usize, usize)>,
    ) -> Option<&'static super::ViewLayout> {
        let mut registry = TypeRegistry::default();
        registry.register::<T>();
        // Safety: a live value of `T`, borrowed for the call
        unsafe {
            view_layout(
                &registry,
                natives,
                TypeId::of::<T>(),
                value as *const T as *const u8,
                Layout::new::<T>(),
            )
        }
    }

    #[test]
    fn view_layouts_are_measured_on_values() {
        // a "native" type: `[f32; 3]` stands for one with a C layout
        let natives = |type_id| (type_id == TypeId::of::<[f32; 3]>()).then_some((12, 4));
        let first = Mixed::default();
        let layout = layout_of(&first, &natives).expect("Mixed has fields in place");
        let fields = layout
            .fields
            .iter()
            .map(|f| (f.name, f.offset, f.ty))
            .collect::<Vec<_>>();
        // in place: scalars and natives at the offsets rustc chose, ordered by offset; the rest by reflection
        let mut expected = vec![
            ("flag", offset_of!(Mixed, flag), FfiType::Scalar("bool")),
            ("x", offset_of!(Mixed, x), FfiType::Scalar("double")),
            (
                "small",
                offset_of!(Mixed, small),
                FfiType::Scalar("uint8_t"),
            ),
            ("y", offset_of!(Mixed, y), FfiType::Scalar("float")),
            (
                "native",
                offset_of!(Mixed, native),
                FfiType::Value(TypeId::of::<[f32; 3]>()),
            ),
        ];
        expected.sort_by_key(|f| f.1);
        assert_eq!(fields, expected);
        assert_eq!(
            (layout.size, layout.align),
            (size_of::<Mixed>(), align_of::<Mixed>())
        );

        // D4: another value has the same offsets, the layout is kept
        let second = Mixed::default();
        let again = layout_of(&second, &natives).expect("still has views");
        assert_eq!(again.fields, layout.fields);

        // no named fields in place: no views
        assert!(layout_of(&Tuple::default(), &natives).is_none());
        assert!(layout_of(&Empty, &natives).is_none());
    }
}
