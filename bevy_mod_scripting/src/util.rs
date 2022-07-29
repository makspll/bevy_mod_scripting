/// Implements tealr::TypeName,tealr::TypeBody and mlua::Userdata based on non-generic single token type name implementing TealData
macro_rules! impl_tealr_type {
    ($v:ty) => {
        impl tealr::TypeName for $v {
            fn get_type_parts() -> ::std::borrow::Cow<'static, [tealr::NamePart]> {
                ::std::borrow::Cow::Borrowed(&[tealr::NamePart::Type(tealr::TealType {
                    name: ::std::borrow::Cow::Borrowed(stringify!($v)),
                    generics: None,
                    type_kind: tealr::KindOfType::External,
                })])
            }
        }

        impl mlua::UserData for $v {
            fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
                let mut wrapper = ::tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
                <Self as ::tealr::mlu::TealData>::add_fields(&mut wrapper)
            }

            fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(
                methods: &mut M,
            ) {
                let mut x = ::tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
                <Self as ::tealr::mlu::TealData>::add_methods(&mut x);
            }
        }

        impl tealr::TypeBody for $v {
            fn get_type_body() -> tealr::TypeGenerator {
                let mut gen = ::tealr::RecordGenerator::new::<Self>(false);
                gen.is_user_data = true;
                <Self as ::tealr::mlu::TealData>::add_fields(&mut gen);
                <Self as ::tealr::mlu::TealData>::add_methods(&mut gen);
                <_ as ::std::convert::From<_>>::from(gen)
            }
        }
    };
}

// /// Implements UserData for type which implements TealData, can handle generics after the type name:
// /// ```rust,ignore
// /// impl_user_data!(MyType<'a,T : Debug>);
// /// ```
// macro_rules! impl_user_data {
//     ($v:ident $(< $( $lt:tt $( : $clt:tt $(+ $dlt:tt $(<'a>)? )* )? ),+ >)? ) => {
//         impl $(< $( $lt $( : $clt $(+ $dlt $(<'a>)?)* )? ),+ >)? ::tealr::mlu::mlua::UserData for $v $(< $( $lt ),+ >)?  {
//             fn add_methods<'lua, M: ::tealr::mlu::mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
//                 let mut x = ::tealr::mlu::UserDataWrapper::from_user_data_methods(methods);
//                 <Self as ::tealr::mlu::TealData>::add_methods(&mut x);
//             }
//             fn add_fields<'lua, F: ::tealr::mlu::mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
//                 let mut wrapper = ::tealr::mlu::UserDataWrapper::from_user_data_fields(fields);
//                 <Self as ::tealr::mlu::TealData>::add_fields(&mut wrapper)
//             }
//         }

//     }
// }
pub(crate) use impl_tealr_type;
// pub(crate) use impl_user_data;
