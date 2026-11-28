//! Expect: the trait bound `foreign::Vec3: FfiField` is not satisfied (no overlay declared for Vec3)
#[path = "../support/foreign.rs"]
mod foreign;
use bevy_mod_scripting_ffi::unsafe_c_overlay;

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

fn main() {}
