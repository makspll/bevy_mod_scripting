use bevy::{prelude::*, math::{DMat4, DVec4, DMat3}};
use bevy_mod_scripting::{ScriptingPlugin, RLuaScriptHost, ScriptHost, AddScriptHost, LuaWrapper, LuaValue, LuaVec3, LuaDMat3, LuaDMat4, mlu::mlua::{Lua, UserData}};
use criterion::{black_box,criterion_group,criterion_main,Criterion, BenchmarkId};
use parking_lot::RwLock;
use std::{sync::Arc,fmt, ops::Add};

criterion_group!(
    benches,
    addition,
    mat_transpose,
    mat_inverse,
    convert_to_lua,
);
criterion_main!(benches);

fn wrap_owned<T : LuaValue>(v : T) -> LuaWrapper<T> {
    black_box(LuaWrapper::Owned(v, Arc::new(RwLock::new(()))))
}



enum WrapperSize{
    Small(u8),
    Medium(Vec3),
    Large(DMat4),
}
impl fmt::Display for WrapperSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WrapperSize::Small(_) => write!(f,"u8"),
            WrapperSize::Medium(_) => write!(f,"Vec3"),
            WrapperSize::Large(_) => write!(f,"DMat4"),
        }
    }
}

fn convert_to_lua(c: &mut Criterion){
    let mut group = c.benchmark_group("ToLua");

    let lua = Lua::new();
    lua.context(|c| {

        group.bench_function("Vec3", |b| {
            let v = LuaVec3::new(Vec3::new(1.0,2.0,3.0));
            b.iter(|| c.create_userdata(v.clone()).unwrap() )
            
        });

        group.bench_function("DMat4", |b| {
            let v = LuaDMat4::new(DMat4::from_cols(
                                                    DVec4::new(69.0,69.0,69.0,69.0),
                                                    DVec4::new(69.0,69.0,69.0,69.0),
                                                    DVec4::new(69.0,69.0,69.0,69.0),
                                                    DVec4::new(69.0,69.0,69.0,69.0)));
            b.iter(|| c.create_userdata(v.clone()).unwrap())
        });

        #[derive(Clone)]
        struct Baseline(DMat4);
        impl UserData for Baseline {}
        
        group.bench_function("NoMethodsDMat4", |b| {
            let v = Baseline(DMat4::from_cols(
                DVec4::new(69.0,69.0,69.0,69.0),
                DVec4::new(69.0,69.0,69.0,69.0),
                DVec4::new(69.0,69.0,69.0,69.0),
                DVec4::new(69.0,69.0,69.0,69.0)));

            b.iter(|| c.create_userdata(v.clone()).unwrap() )
        });
    });
}

fn addition(c: &mut Criterion){
    let mut group = c.benchmark_group("Addition");
    for i in [WrapperSize::Small(69),
                            WrapperSize::Medium(Vec3::new(69.0,69.0,69.0)),
                            WrapperSize::Large(DMat4::from_cols(
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0)))].iter(){
        
        group.bench_with_input(BenchmarkId::new("Wrapped",i), i, |b,i| {
            match i {
                WrapperSize::Small(v) =>  {
                    let s = wrap_owned(*v); 
                    let o = wrap_owned(*v); 
                    b.iter(|| s.inner() + o.inner())
                },
                WrapperSize::Medium(v) => {
                    let s = wrap_owned(*v); 
                    let o = wrap_owned(*v); 
                    b.iter(|| s.inner() + o.inner())
                },
                WrapperSize::Large(v) => {
                    let s = wrap_owned(*v); 
                    let o = wrap_owned(*v); 
                    b.iter(|| s.inner() + o.inner())
                },
            };
        });


        group.bench_with_input(BenchmarkId::new("Baseline",i), i, |b,i| {
            match i {
                WrapperSize::Small(v) =>  {
                    let s = *v; 
                    let o = *v; 
                    b.iter(|| s + o)
                },
                WrapperSize::Medium(v) => {
                    let s = *v; 
                    let o = *v; 
                    b.iter(|| s + o)
                },
                WrapperSize::Large(v) => {
                    let s = *v; 
                    let o = *v; 
                    b.iter(|| s + o)
                },
            };
        });
    }
    
}


fn mat_transpose(c: &mut Criterion){
    let mut group = c.benchmark_group("Transpose");
    for i in [DMat4::from_cols(
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0))].iter(){
        let wrapped = wrap_owned(*i); 
                                                
        group.bench_function("Ref", |b| {
            b.iter(|| wrapped.val(|s| s.transpose()))
        });

        group.bench_function("Copy", |b| {
            b.iter(|| wrapped.inner().transpose())
        });

        group.bench_function("Baseline", |b| {
            b.iter(|| i.transpose())
        });
    }
    
}

fn mat_inverse(c: &mut Criterion){
    let mut group = c.benchmark_group("Inverse");
    for i in [DMat4::from_cols(
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0),
                                                DVec4::new(69.0,69.0,69.0,69.0))].iter(){
        let wrapped = wrap_owned(*i); 
                                                
        group.bench_function("Ref", |b| {
            b.iter(|| wrapped.val(|s| s.inverse()))
        });

        group.bench_function("Copy", |b| {
            b.iter(|| wrapped.inner().inverse())
        });

        group.bench_function("Baseline", |b| {
            b.iter(|| i.inverse())
        });
    }
    
}



