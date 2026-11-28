//! An overlay's layout as declared in `unsafe_c_overlay!`, and the const checks against the real type.
//!
//! The macro only fills in a [`Layout`] and the probes, everything here runs at compile time.

use core::mem::MaybeUninit;

use const_panic::concat_panic;

use crate::FfiField;

/// An overlay as declared: its fields and pads in order, and the sizes it must match.
pub struct Layout<'a> {
    /// The overlay's Rust name, for errors
    pub overlay: &'static str,
    /// The real type's name as written, for errors
    pub target: &'static str,
    /// `size_of` the real type
    pub size: usize,
    /// `align_of` the real type
    pub align: usize,
    /// `size_of` the generated overlay struct
    pub overlay_size: usize,
    /// `align_of` the generated overlay struct
    pub overlay_align: usize,
    /// In declaration order
    pub fields: &'a [Field],
}

/// A field or pad of a [`Layout`].
pub struct Field {
    /// As declared
    pub name: &'static str,
    /// What it holds
    pub kind: FieldKind,
}

/// See [`Field`].
pub enum FieldKind {
    /// `#[pad] name: [u8; len]`, never read
    Pad(usize),
    /// A field of a [`FfiField`] type.
    Data {
        /// `size_of` its overlay
        size: usize,
        /// `align_of` its overlay
        align: usize,
        /// Its [`FfiField::INIT_MASK`]
        mask: &'static [bool],
        /// `offset_of!` it in the generated overlay struct
        offset: usize,
    },
}

impl Field {
    /// A field of type `T`, at `offset` in the generated overlay struct.
    pub const fn data<T: FfiField>(name: &'static str, offset: usize) -> Self {
        Self {
            name,
            kind: FieldKind::Data {
                size: size_of::<T::Overlay>(),
                align: align_of::<T::Overlay>(),
                mask: T::INIT_MASK,
                offset,
            },
        }
    }

    /// `#[pad] name: [u8; len]`.
    pub const fn pad(name: &'static str, len: usize) -> Self {
        Self {
            name,
            kind: FieldKind::Pad(len),
        }
    }
}

/// A field's expected value in a [`Probe`].
pub struct Expected<'a> {
    /// The bytes of the value
    pub bytes: &'a [MaybeUninit<u8>],
    /// The value as written, for errors
    pub text: &'static str,
}

/// One `#[probe(..)]`: the bytes of the constructed value, and what each field should hold.
pub struct Probe<'a> {
    /// The bytes of the real type's value
    pub image: &'a [MaybeUninit<u8>],
    /// One per [`Layout::fields`], `None` for pads and fields not probed (`_`)
    pub expected: &'a [Option<Expected<'a>>],
}

impl Layout<'_> {
    /// The bytes of the real type the declared fields initialize. `N` is the real type's size.
    pub const fn init_mask<const N: usize>(&self) -> [bool; N] {
        self.mask_where::<N>(None)
    }

    /// Checks the declared layout against the real type, then each probe. `N` is the real type's size.
    ///
    /// Panics at compile time with what is wrong.
    #[track_caller]
    pub const fn check<const N: usize>(&self, probes: &[Probe]) {
        self.check_layout();
        let multi = probes.len() > 1;

        // every probed byte is non-zero in at least one probe
        let mut f = 0;
        while f < self.fields.len() {
            if let FieldKind::Data { mask, .. } = self.fields[f].kind {
                let (mut i, mut nonzero, mut probed) = (0, true, false);
                while i < mask.len() {
                    let (mut p, mut any) = (0, !mask[i]);
                    while p < probes.len() {
                        if let Some(expected) = &probes[p].expected[f] {
                            probed = true;
                            // SAFETY: `mask` marks the initialized bytes of the field's type
                            any |= mask[i] && unsafe { expected.bytes[i].assume_init() } != 0;
                        }
                        p += 1;
                    }
                    nonzero &= any;
                    i += 1;
                }
                if probed && !nonzero {
                    let name = self.fields[f].name;
                    if multi {
                        concat_panic!(
                            "probe values for `",
                            display: name,
                            "` have a byte that is zero in every probe, make every byte non-zero in at least one"
                        );
                    } else {
                        concat_panic!(
                            "probe value for `",
                            display: name,
                            "` has a zero byte, use a value with every byte non-zero"
                        );
                    }
                }
            }
            f += 1;
        }

        // each probe stored the expected values
        let mut p = 0;
        while p < probes.len() {
            let mut f = 0;
            while f < self.fields.len() {
                if let (FieldKind::Data { mask, offset, .. }, Some(expected)) =
                    (&self.fields[f].kind, &probes[p].expected[f])
                    && !masked_eq(probes[p].image, *offset, expected.bytes, mask)
                {
                    let (overlay, name, text) = (self.overlay, self.fields[f].name, expected.text);
                    if multi {
                        concat_panic!(
                            "probe failed: `", display: overlay, "::", display: name, "` expected `",
                            display: text, "` in probe ", p
                        );
                    } else {
                        concat_panic!(
                            "probe failed: `", display: overlay, "::", display: name, "` expected `",
                            display: text, "`"
                        );
                    }
                }
                f += 1;
            }
            p += 1;
        }

        // each probed value occurs once in at least one probe, which pins the field's offset: the value can't be
        // another field's bytes read at the wrong place. Fields not probed in a probe (`_`) hold unknown bytes and are
        // left out of its image.
        let mut f = 0;
        while f < self.fields.len() {
            if let FieldKind::Data { mask, .. } = self.fields[f].kind {
                let (mut p, mut probed, mut once) = (0, false, false);
                while p < probes.len() {
                    if let Some(expected) = &probes[p].expected[f] {
                        probed = true;
                        let image_mask = self.mask_where::<N>(Some(&probes[p]));
                        once |=
                            occurrences(probes[p].image, &image_mask, expected.bytes, mask) == 1;
                    }
                    p += 1;
                }
                if probed && !once {
                    let name = self.fields[f].name;
                    if multi {
                        concat_panic!(
                            "probe values for `",
                            display: name,
                            "` also occur elsewhere in every probe, make its bytes unique in at least one"
                        );
                    } else {
                        concat_panic!(
                            "probe value for `",
                            display: name,
                            "` also occurs elsewhere in the probe, use a value whose bytes appear nowhere else"
                        );
                    }
                }
            }
            f += 1;
        }
    }

    /// Fields are aligned and at their struct offsets, and add up to the real type.
    #[track_caller]
    const fn check_layout(&self) {
        let (overlay, target) = (self.overlay, self.target);
        let (mut f, mut at) = (0, 0);
        while f < self.fields.len() {
            match self.fields[f].kind {
                FieldKind::Pad(len) => at += len,
                FieldKind::Data {
                    size,
                    align,
                    offset,
                    ..
                } => {
                    let name = self.fields[f].name;
                    if at % align != 0 {
                        concat_panic!(
                            "`", display: overlay, "::", display: name,
                            "` is misaligned, declare the padding before it with `#[pad]`"
                        );
                    }
                    // aligned fields of a `#[repr(C)]` struct get no padding between them, so this always holds
                    if at != offset {
                        concat_panic!(
                            "`", display: overlay, "::", display: name, "` is at offset ", offset,
                            " in the overlay struct, but the declaration puts it at ", at
                        );
                    }
                    at += size;
                }
            }
            f += 1;
        }
        if at != self.size || self.overlay_size != self.size {
            concat_panic!(
                "the fields and pads of `", display: overlay, "` don't add up to the size of `",
                display: target, "`"
            );
        }
        if self.overlay_align != self.align {
            concat_panic!(
                "`", display: overlay, "` has a different alignment than `", display: target, "`"
            );
        }
    }

    /// The initialized bytes of the declared fields, only the fields `probe` expects a value for if given.
    const fn mask_where<const N: usize>(&self, probe: Option<&Probe>) -> [bool; N] {
        let mut out = [false; N];
        let (mut f, mut at) = (0, 0);
        while f < self.fields.len() {
            match self.fields[f].kind {
                FieldKind::Pad(len) => at += len,
                FieldKind::Data { size, mask, .. } => {
                    let included = match probe {
                        Some(probe) => probe.expected[f].is_some(),
                        None => true,
                    };
                    let mut i = 0;
                    // bytes past the end are dropped, an overlay too big for its type fails `check_layout`
                    while included && i < mask.len() && at + i < N {
                        out[at + i] = mask[i];
                        i += 1;
                    }
                    at += size;
                }
            }
            f += 1;
        }
        out
    }
}

/// `image[at..]` starts with `value` on the bytes where `mask` is true; other bytes are never read.
const fn masked_eq(
    image: &[MaybeUninit<u8>],
    at: usize,
    value: &[MaybeUninit<u8>],
    mask: &[bool],
) -> bool {
    let mut i = 0;
    while i < mask.len() {
        // SAFETY: `mask` marks initialized bytes, `check_layout` keeps fields inside the image
        if mask[i] && unsafe { image[at + i].assume_init() != value[i].assume_init() } {
            return false;
        }
        i += 1;
    }
    true
}

/// The number of offsets of `image` at which `needle` occurs, comparing the bytes initialized in both.
const fn occurrences(
    image: &[MaybeUninit<u8>],
    image_mask: &[bool],
    needle: &[MaybeUninit<u8>],
    needle_mask: &[bool],
) -> usize {
    let (mut count, mut at) = (0, 0);
    while at + needle.len() <= image.len() {
        let (mut i, mut hit) = (0, true);
        while i < needle.len() && hit {
            // SAFETY: both masks mark initialized bytes
            hit = !needle_mask[i]
                || (image_mask[at + i]
                    && unsafe { image[at + i].assume_init() == needle[i].assume_init() });
            i += 1;
        }
        if hit {
            count += 1;
        }
        at += 1;
    }
    count
}
