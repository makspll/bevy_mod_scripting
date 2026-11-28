//! Overlays over glam types.

use crate::unsafe_c_overlay;

unsafe_c_overlay! {
    // SAFETY: three f32s at 0/4/8
    #[probe(glam::Vec3::new(1.1, 2.2, 3.3))]
    glam::Vec3 as Vec3C {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
    }
}

unsafe_c_overlay! {
    // SAFETY: four f32 lanes on every glam backend; x/y/z are reached through Deref on SIMD backends,
    // the fourth lane is a private field (scalar) or an unnamed lane (SIMD), always an initialized f32
    #[probe(glam::Vec3A::new(1.1, 2.2, 3.3))]
    glam::Vec3A as Vec3AC {
        #[probe = 1.1] x: f32,
        #[probe = 2.2] y: f32,
        #[probe = 3.3] z: f32,
        #[opaque] #[probe = _] _w: f32,
    }
}

unsafe_c_overlay! {
    // SAFETY: three Vec3 columns
    #[probe(glam::Mat3::from_cols(
        glam::Vec3::new(1.1, 2.2, 3.3),
        glam::Vec3::new(4.4, 5.1, 6.6),
        glam::Vec3::new(7.7, 8.8, 9.9),
    ))]
    glam::Mat3 as Mat3C {
        #[probe = glam::Vec3::new(1.1, 2.2, 3.3)] x_axis: glam::Vec3,
        #[probe = glam::Vec3::new(4.4, 5.1, 6.6)] y_axis: glam::Vec3,
        #[probe = glam::Vec3::new(7.7, 8.8, 9.9)] z_axis: glam::Vec3,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FfiField;

    fn view<T: FfiField>(value: &T) -> &T::Overlay {
        // SAFETY: FfiField guarantees Overlay mirrors T
        unsafe { &*(value as *const T).cast::<T::Overlay>() }
    }

    #[test]
    fn glam_overlays_mirror_real_values() {
        let v = glam::Vec3::new(1.0, 2.0, 3.0);
        let c: &Vec3C = view(&v);
        assert_eq!((c.x, c.y, c.z), (1.0, 2.0, 3.0));

        let v = glam::Vec3A::new(1.0, 2.0, 3.0);
        let c: &Vec3AC = view(&v);
        assert_eq!((c.x, c.y, c.z), (1.0, 2.0, 3.0));
        assert_eq!(align_of::<Vec3AC>(), align_of::<glam::Vec3A>());

        let m = glam::Mat3::from_cols(
            glam::Vec3::new(1.0, 2.0, 3.0),
            glam::Vec3::new(4.0, 5.0, 6.0),
            glam::Vec3::new(7.0, 8.0, 9.0),
        );
        let c: &Mat3C = view(&m);
        assert_eq!((c.x_axis.x, c.y_axis.y, c.z_axis.z), (1.0, 5.0, 9.0));

        assert_eq!(<glam::Vec3A as FfiField>::INIT_MASK, &[true; 16]);
        assert_eq!(<glam::Mat3 as FfiField>::INIT_MASK, &[true; 36]);
    }
}
