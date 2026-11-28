//! Overlays that must compile and mirror the real values.

#[path = "ui/support/foreign.rs"]
mod foreign;

use bevy_mod_scripting_ffi::{CDecl, CNameConflict, FfiField, cdef, unsafe_c_overlay};

unsafe_c_overlay! {
    #[probe(foreign::Vec3::new(1.1, 2.2, 3.3))]
    foreign::Vec3 as Vec3C {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
    }
}

unsafe_c_overlay! {
    #[probe(foreign::Vec3A::new(1.1, 2.2, 3.3))]
    foreign::Vec3A as Vec3AC {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
        #[opaque] #[probe = _] _w: f32,
    }
}

unsafe_c_overlay! {
    #[probe(foreign::Mat3::from_cols(
        foreign::Vec3::new(1.1, 2.2, 3.3),
        foreign::Vec3::new(4.4, 5.1, 6.6),
        foreign::Vec3::new(7.7, 8.8, 9.9),
    ))]
    foreign::Mat3 as Mat3C {
        #[probe = foreign::Vec3::new(1.1, 2.2, 3.3)] x_axis: foreign::Vec3,
        #[probe = foreign::Vec3::new(4.4, 5.1, 6.6)] y_axis: foreign::Vec3,
        #[probe = foreign::Vec3::new(7.7, 8.8, 9.9)] z_axis: foreign::Vec3,
    }
}

unsafe_c_overlay! {
    #[probe(foreign::Foo::new(0x11, 0x2233_4455))]
    #[c_name = "foreign_Foo"]
    foreign::Foo as FooC {
        #[probe = 0x11] a: u8,
        #[pad] _p0: [u8; 3],
        #[probe = 0x2233_4455] b: u32,
    }
}

unsafe_c_overlay! {
    #[probe(foreign::FooPair::new(
        foreign::Foo::new(0x11, 0x2233_4455),
        foreign::Foo::new(0x66, 0x7788_99AA),
    ))]
    foreign::FooPair as FooPairC {
        #[probe = foreign::Foo::new(0x11, 0x2233_4455)] first: foreign::Foo,
        #[probe = foreign::Foo::new(0x66, 0x7788_99AA)] second: foreign::Foo,
    }
}

unsafe_c_overlay! {
    #[probe(foreign::Handle::new(0x0102_0304, 0x0506_0708))]
    foreign::Handle as HandleC {
        #[opaque] #[probe = 0x0102_0304] id: u32,
        #[opaque] #[probe = 0x0506_0708] generation: u32,
    }
}

unsafe_c_overlay! {
    #[probe(foreign::Velocity::new(1.1, foreign::Vec3A::new(2.2, 3.3, 4.4)))]
    foreign::Velocity as VelocityC {
        #[probe = 1.1] speed: f32,
        #[pad] _p0: [u8; 12],
        #[probe = foreign::Vec3A::new(2.2, 3.3, 4.4)] dir: foreign::Vec3A,
    }
}

/// Reads a real value through its overlay.
fn view<T: FfiField>(value: &T) -> T::Overlay {
    // SAFETY: FfiField guarantees Overlay mirrors T
    unsafe { *(value as *const T).cast::<T::Overlay>() }
}

/// Reads an overlay value made outside Rust (like a script's `ffi.new`) back as the real type.
/// The overlay sits after a 4-byte field in a 16-aligned holder, so if its align is below the real type's
/// it lands misaligned and the read aborts (debug assertions, Miri).
fn round_trip<T: FfiField + Copy>(overlay: T::Overlay) -> T {
    #[repr(C, align(16))]
    struct Holder<O> {
        _lead: f32,
        value: O,
    }
    let holder = Holder {
        _lead: 0.0,
        value: overlay,
    };
    // SAFETY: FfiField guarantees Overlay mirrors T, including alignment
    unsafe { *(&holder.value as *const T::Overlay).cast::<T>() }
}

#[test]
fn overlays_mirror_real_values() {
    let c: Vec3C = view(&foreign::Vec3::new(1.0, 2.0, 3.0));
    assert_eq!((c.x, c.y, c.z), (1.0, 2.0, 3.0));
    assert_eq!(round_trip::<foreign::Vec3>(c).y, 2.0);

    let c: Vec3AC = view(&foreign::Vec3A::new(1.0, 2.0, 3.0));
    assert_eq!((c.x, c.y, c.z), (1.0, 2.0, 3.0));
    assert_eq!(round_trip::<foreign::Vec3A>(c).y, 2.0);

    let c: VelocityC = view(&foreign::Velocity::new(
        1.0,
        foreign::Vec3A::new(2.0, 3.0, 4.0),
    ));
    assert_eq!((c.speed, c.dir.z), (1.0, 4.0));
    assert_eq!(round_trip::<foreign::Velocity>(c).dir.x, 2.0);

    let m = foreign::Mat3::from_cols(
        foreign::Vec3::new(1.0, 2.0, 3.0),
        foreign::Vec3::new(4.0, 5.0, 6.0),
        foreign::Vec3::new(7.0, 8.0, 9.0),
    );
    let c: Mat3C = view(&m);
    assert_eq!((c.x_axis.x, c.y_axis.y, c.z_axis.z), (1.0, 5.0, 9.0));

    let c: FooC = view(&foreign::Foo::new(7, 9));
    assert_eq!((c.a, c.b), (7, 9));

    let c: FooPairC = view(&foreign::FooPair::new(
        foreign::Foo::new(1, 2),
        foreign::Foo::new(3, 4),
    ));
    assert_eq!((c.first.a, c.second.b), (1, 4));

    let c: HandleC = view(&foreign::Handle::new(3, 4));
    assert_eq!((c.id, c.generation), (3, 4));
}

#[test]
fn generated_metadata() {
    // C declarations: own typedef only, field types first and once, `#[c_name]` respected
    const FOO: &str = "typedef struct __attribute__((aligned(4))) foreign_Foo { uint8_t a; uint8_t _p0[3]; uint32_t b; } foreign_Foo;\n";
    const PAIR: &str = "typedef struct __attribute__((aligned(4))) FooPairC { foreign_Foo first; foreign_Foo second; } FooPairC;\n";
    let (foo, pair) = (
        <foreign::Foo as FfiField>::C_DECL,
        <foreign::FooPair as FfiField>::C_DECL,
    );
    assert_eq!((foo.cdef, pair.cdef), (FOO, PAIR));
    assert_eq!(cdef(&[pair, foo]).unwrap(), format!("{FOO}{PAIR}"));
    assert!(
        <foreign::Vec3A as FfiField>::C_DECL
            .cdef
            .starts_with("typedef struct __attribute__((aligned(16))) Vec3AC {")
    );
    assert!(
        pair.header.contains(
            "_Static_assert(offsetof(FooPairC, second) == 8, \"FooPairC.second offset\");"
        )
    );
    let impostor = CDecl {
        cdef: "typedef struct foreign_Foo { float x; } foreign_Foo;\n",
        ..foo
    };
    assert_eq!(cdef(&[foo, impostor]), Err(CNameConflict("foreign_Foo")));

    // init masks
    const FOO_MASK: [bool; 8] = [true, false, false, false, true, true, true, true];
    assert_eq!(<foreign::Foo as FfiField>::INIT_MASK, &FOO_MASK);
    assert_eq!(
        <foreign::FooPair as FfiField>::INIT_MASK,
        &[FOO_MASK, FOO_MASK].concat()[..]
    );
    assert_eq!(<foreign::Mat3 as FfiField>::INIT_MASK, &[true; 36]);
    assert_eq!(<foreign::Vec3A as FfiField>::INIT_MASK, &[true; 16]);
}
