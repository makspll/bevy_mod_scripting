//! This example illustrates how reflection works for simple data structures, like
//! structs, tuples and vectors.

use bevy::{
    prelude::*,
    reflect::{DynamicList, ReflectRef},
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// Deriving reflect on a struct will implement the `Reflect` and `Struct` traits
#[derive(Reflect,Debug)]
pub struct A {
    e: Quat
}

fn setup() {

    // some value we want to affect via lua
    let mut value: Box<dyn Reflect> = Box::new(A {
        e: Quat::from_xyzw(1.0,2.0,3.0,4.0)
    });

    // transformed into a pointer
    let ptr_val : *mut dyn Reflect = value.as_mut() as *mut dyn Reflect; 

    // we get the `e` destination field as a pointer as well
    let ptr_dest : *mut dyn Reflect = match unsafe{&mut *ptr_val}.reflect_mut() {
        bevy::reflect::ReflectMut::Struct(s) => {
            s.field_mut("e").unwrap() as *mut dyn Reflect
        },
        _ => {panic!()}
    };

    // we downcast this destination back to a quat
    let base = unsafe {&mut *ptr_dest}.downcast_mut::<Quat>().unwrap();

    // we apply new quat to the base
    // SEGFAULT on `clone` in apply internals here
    base.apply(&Quat::from_xyzw(4.0,3.0,2.0,1.0));

    println!("{:?}",value.take::<A>());

}