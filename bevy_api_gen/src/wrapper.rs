use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{Impl, Crate, Item, Id, ItemEnum};
use serde::Deserialize;

use crate::{Newtype, PrettyWriter,Args, Config, Arg, ArgWrapperType, ArgType, stringify_type};

pub static WRAPPER_PREFIX : &str = "Lua";



#[derive(Debug)]
pub struct WrappedItem<'a> {
    pub wrapper_name : String,
    pub wrapped_type: &'a String,
    pub path_components: &'a [String],
    pub source: &'a Crate,
    pub config: &'a Newtype,
    pub item : &'a Item,
    /// The items coming from all trait implementations
    pub impl_items: IndexMap<&'a str,Vec<(&'a Impl, &'a Item)>>, 
    pub implemented_traits: IndexSet<String>,
    pub self_impl: Option<&'a Impl>,
    pub crates : &'a [Crate],
    /// If this type has some things which are "static" this is set to true later
    pub has_global_methods: bool,
}



impl WrappedItem<'_> {

    /// Writes full type path inline corresponding to `Reflect::type_name` of each type
    /// 
    /// As: 
    /// ```rust,ignore
    /// 
    /// this
    /// |
    /// |..........|
    /// my_type_path::Type : Value : 
    ///  UnaryOps( ...
    /// ```
    pub fn write_inline_full_path(&self, writer: &mut PrettyWriter, _: &Args) {
        if self.config.import_path.is_empty(){
            writer.write_inline(&self.path_components.join("::"));
        } else {
            writer.write_inline(&self.config.import_path);
        }
    }

    /// Writes the docstring for the type over multiple lines
    /// 
    /// As:
    /// ```rust,ignore
    /// 
    /// /// generated docstring
    /// /// here
    /// my_macro_key : Value : 
    ///  UnaryOps(
    ///  ... 
    ///  )
    ///  +
    ///  ...
    /// ```
    pub fn write_type_docstring(&self, writer : &mut PrettyWriter, _: &Args){
        let strings = if let Some(d) = &self.config.doc {
            d.to_string()
        } else {
            self.item.docs
            .as_ref()
            .cloned()
            .unwrap_or_else(||"".to_string())
        };
        writer.set_prefix("///".into());
        strings.lines().for_each(|l| 
            {writer.write_line(l);}
        );
        writer.clear_prefix();
    }

    /// Writes the docstring for the given auto method over multiple lines
    /// 
    /// As:
    /// ```rust,ignore
    /// 
    ///
    /// my_macro_key : Value : 
    ///  Methods(
    ///        /// generated docstring 
    ///        /// here
    ///        my_method(usize) -> u32
    ///  )
    ///  +
    ///  ...
    /// ```
    pub fn write_method_docstring(&self, id : &Id, writer : &mut PrettyWriter, _: &Args){
        writer.set_prefix("///".into());
        self.source.index
                .get(id)
                .unwrap().docs
                .as_ref()
                .cloned()
                .unwrap_or_else(||"".to_owned())
                .lines().for_each(|l| {writer.write_line(l);});
        writer.clear_prefix();
    }

    /// Writes the contents of the impl block for this wrapper
    /// 
    /// As:
    /// 
    /// ```rust,ignore
    ///     impl {
    ///     ... // this!
    ///     }
    /// ```
    pub fn write_impl_block_body(&self, writer: &mut PrettyWriter, _: &Args) {
        self.config.lua_methods
            .iter()
            .for_each(|v| {
                writer.write_postfixed_line(v, ";");
            })
    }

    /// Generates all derive flags for the type,
    /// 
    /// Returns additional imports necessary for the generated methods
    /// 
    /// ```rust,ignore
    /// my_type::Type : Value: 
    /// ... // flags go here
    /// ``` 
    pub fn write_derive_flags_body(&mut self, config: &Config, writer: &mut PrettyWriter, args: &Args) {

        if self.implemented_traits.contains("Clone"){
            // this flag requires cloning
            writer.write_line("FromLuaProxy +");
        }
        

        writer.write_line("Fields");
        writer.open_paren();

        match &self.item.inner{
            ItemEnum::Struct(struct_) => {
                struct_.fields.iter()
                    .map(|field_| self.source.index.get(field_).unwrap())
                    .filter_map(|field_| match &field_.inner{
                        ItemEnum::StructField(type_) => Some((field_.name.as_ref().unwrap(),type_,field_)),
                        _ => None
                    })
                    .filter_map(|(name,type_, field_)|{

                        let arg_type : ArgType = type_.try_into().ok()?;
                        let base_ident = arg_type
                            .base_ident() // resolve self
                            .unwrap_or_else(|()| self.wrapped_type.as_str());

                        // if the underlying ident is self, we shouldn't wrap it when printing it
                        let wrapper : ArgWrapperType = arg_type.is_self().then(|| ArgWrapperType::None)
                                                        .or_else(|| config.primitives.contains(base_ident).then_some(ArgWrapperType::Raw))
                                                        .or_else(|| config.types.contains_key(base_ident).then_some(ArgWrapperType::Wrapped))
                                                        // we allow this since we later resolve unknown types to be resolved as ReflectedValues
                                                        .unwrap_or(ArgWrapperType::None);
                        
                        let arg = Arg::new(arg_type, wrapper);
                        let mut reflectable_type = arg.to_string();
                        

                        // if we do not have an appropriate wrapper and this is not a primitive or it's not public
                        // we need to go back to the reflection API
                        if arg.wrapper == ArgWrapperType::None
                        {
                            if field_.attrs.iter().find(|attr| attr == &"#[reflect(ignore)]").is_some(){
                                return None
                            }

                            reflectable_type = "Raw(ReflectedValue)".to_owned();
                        } 

                        field_.docs.as_ref().map(|docs| {
                            writer.set_prefix("/// ".into());
                            docs.lines().for_each(|line|{
                                writer.write_line(line);
                            });
                            writer.clear_prefix();
                        });
                        writer.write_no_newline(name);
                        writer.write_inline(": ");
                        writer.write_inline(&reflectable_type);
                        writer.write_inline(",");
                        writer.newline();
                        

                        Some(())
                    }).for_each(drop);
            },
            _ => {}
        };
        writer.close_paren();

        writer.write_line("+ Methods");
        writer.open_paren();
        let mut is_global = false;
        self.impl_items.iter()
            .flat_map(|(_,items)| items.iter())
            .for_each(|(impl_,v)| { 

                // only select trait methods are allowed
                if let Some(trait_) = &impl_.trait_ {
                    if self.config.traits.iter().find(|f| {
                        stringify_type(trait_).and_then(|s| {
                            (&s == &f.name).then_some(())
                        }).is_some()
                    }
                    ).is_some(){
                        // keep going
                    } else {
                        return
                    }
                };

                let (decl,generics) = match &v.inner {
                    ItemEnum::Function(f) => (&f.decl,&f.generics),
                    ItemEnum::Method(m) => (&m.decl,&m.generics),
                    _ => return,
                };

                let mut errors = Vec::default();

                let mut inner_writer = PrettyWriter::new();

                self.write_method_docstring(&v.id, &mut inner_writer,args);

                inner_writer.write_inline(v.name.as_ref().unwrap());
                inner_writer.write_inline("(");
                decl.inputs
                    .iter()
                    .enumerate()
                    .for_each(|(i,(declaration_name,tp))| {

                        let arg_type : Result<ArgType,_> = tp.try_into();

                        if let Ok(arg_type) = arg_type {
                            let base_ident = arg_type
                                                .base_ident()// resolve self for lookup, print actual `self` tho
                                                .unwrap_or_else(|_| self.wrapped_type.as_str());

                            // if the underlying ident is self, we shouldn't wrap it when printing it
                            // if type is unknown no wrapper exists
                            let wrapper_type : Option<ArgWrapperType> = arg_type.is_self().then(|| ArgWrapperType::None)
                                                            .or_else(|| config.primitives.contains(base_ident).then_some(ArgWrapperType::Raw))
                                                            .or_else(|| config.types.contains_key(base_ident).then_some(ArgWrapperType::Wrapped));                                

                            if arg_type.is_self(){
                                is_global = true;
                            }
                            match wrapper_type {
                                Some(w) => inner_writer.write_inline(&Arg::new(arg_type,w).to_string()),
                                None => {
                                    inner_writer.write_inline(&format!("<invalid: {arg_type}>"));
                                    errors.push(format!("Unsupported argument {}",arg_type));
                                    return;
                                },
                            };

                            if declaration_name != "self" && i + 1 != decl.inputs.len() {
                                inner_writer.write_inline(",");
                            } else if declaration_name == "self" {
                                // macro needs to recognize the self receiver
                                inner_writer.write_inline(":");
                            }
                            
                        } else {
                            errors.push(format!("Unsupported argument {}",arg_type.unwrap_err()))
                        };
                });
                inner_writer.write_inline(")");
                
                decl.output
                    .as_ref()
                    .map(|tp| {
                        // let type_ = type_to_arg(tp, &mut |base_string : &String| 
                        //     Ok(to_auto_method_argument(base_string,&self.wrapped_type,config,false, WRAPPER_PREFIX)));
                        let arg_type : Result<ArgType,_> = tp.try_into();
                        if let Ok(arg_type) = arg_type {
                            let base_ident = arg_type
                                .base_ident()// resolve self for lookup
                                .unwrap_or_else(|_| self.wrapped_type.as_str());

                            // if the underlying ident is self, we shouldn't wrap it when printing it
                            // if type is unknown, no wrapper type exists
                            let wrapper_type : Option<ArgWrapperType> = arg_type.is_self().then(|| ArgWrapperType::None)
                                                            .or_else(|| config.primitives.contains(base_ident).then_some(ArgWrapperType::Raw))
                                                            .or_else(|| config.types.contains_key(base_ident).then_some(ArgWrapperType::Wrapped));
                            match wrapper_type {
                                Some(w) => {
                                    inner_writer.write_inline(" -> ");
                                    inner_writer.write_inline(&Arg::new(arg_type,w).to_string());
                                },
                                None => {
                                    errors.push(format!("Unsupported argument {arg_type}"));
                                    inner_writer.write_inline(&format!("<invalid: {arg_type}>"));
                                },
                            }

                        } else {
                            errors.push(format!("Unsupported argument {}",arg_type.unwrap_err()))
                        }
                    });

                if !generics.params.is_empty(){
                    errors.push("Generics on the method".to_owned());
                }

                if !errors.is_empty(){
                    if args.print_errors {
                        writer.set_prefix("// ".into());
                        writer.write_line(&format!("Exclusion reason: {}",errors.join(",")));
                        writer.extend(inner_writer);
                        writer.clear_prefix();
                        writer.newline();
                    }
                } else {
                    inner_writer.write_inline(",");
                    writer.extend(inner_writer);
                    writer.newline();
                }
        });
        self.has_global_methods = is_global;
        writer.close_paren();

        static BINARY_OPS : [(&str,&str); 5] = [("add","Add"),
                                        ("sub","Sub"),
                                        ("div","Div"),
                                        ("mul","Mul"),
                                        ("rem","Rem")];
        writer.write_line("+ BinOps");
        writer.open_paren();
        BINARY_OPS.into_iter().for_each(|(op,rep) |{
            self.impl_items.get(op).map(|items| {
                items.iter()
                .filter_map(|(impl_,item)| Some((impl_,item,(&impl_.for_).try_into().ok()?) ))
                .filter(|(_,_, self_type) : &(&&Impl,&&Item,ArgType)| {
                    let base_ident = self_type
                        .base_ident()
                        .unwrap_or_else(|_| self.wrapped_type.as_str());
                    let is_self_type_the_wrapper = base_ident == self.wrapped_type && config.types.contains_key(base_ident);
                    let is_primitive = config.primitives.contains(base_ident);

                    is_self_type_the_wrapper || is_primitive
                })
                .for_each(|(impl_,item, self_type)| {
                    let _ = match &item.inner {
                        ItemEnum::Method(m) => {
                            m.decl.inputs
                                .iter()
                                .enumerate()
                                .map(|(idx,(_,t))| {
                                    // check arg is valid
                                    let arg_type : ArgType = t.try_into()?;
                                    let base_ident = arg_type
                                        .base_ident()// resolve self for lookup
                                        .unwrap_or_else(|_| self.wrapped_type.as_str());

                                    // if the underlying ident is self, we shouldn't wrap it when printing it
                                    let wrapper_type : ArgWrapperType = arg_type.is_self().then(|| ArgWrapperType::None)
                                        .or_else(|| config.primitives.contains(base_ident).then_some(ArgWrapperType::Raw))
                                        .or_else(|| config.types.contains_key(base_ident).then_some(ArgWrapperType::Wrapped))
                                        .unwrap();

                                    if wrapper_type != ArgWrapperType::None {
                                        Ok(Arg::new(arg_type,wrapper_type).to_string())
                                    } else {
                                        Err("asd".to_owned())
                                    }
                                }).collect::<Result<Vec<_>,_>>()
                                .and_then(|v| Ok(v.join(&format!(" {} ",rep))))
                                .and_then(|expr| {
                                    // then provide return type
                                    // for these traits that's on associated types within the impl
                                    let out_type = impl_.items.iter().find_map(|v| {
                                        let item = self.source.index.get(v).unwrap();
                                        if let ItemEnum::AssocType { default, .. }= &item.inner{
                                            match item.name.as_ref().map(|v| v.as_str()) {
                                                Some("Output") => return Some(default.as_ref().unwrap()),
                                                _ => {}
                                            }
                                        }
                                        None
                                    }).ok_or_else(|| expr.clone())?;

                                    let arg_type : ArgType = out_type.try_into()?;
                                    let base_ident = arg_type.base_ident() // resolve self for lookup
                                        .unwrap_or_else(|_| self.wrapped_type.as_str());

                                    // if the underlying ident is self, we shouldn't wrap it when printing it
                                    let wrapper_type : ArgWrapperType = arg_type.is_self().then(|| ArgWrapperType::None)
                                        .or_else(|| config.primitives.contains(base_ident).then_some(ArgWrapperType::Raw))
                                        .or_else(|| config.types.contains_key(base_ident).then_some(ArgWrapperType::Wrapped))
                                        .unwrap();

                                    if wrapper_type == ArgWrapperType::None {
                                        return Err(arg_type.to_string())
                                    }

                                    let return_string = Arg::new(arg_type,wrapper_type).to_string();

                                    writer.write_no_newline(&expr);
                                    writer.write_inline(" -> ");
                                    writer.write_inline(&return_string);
                                    writer.write_inline(",");
                                    writer.newline();
                                    Ok(())
                                })
                        },
                        _ => panic!("Expected method")
                    };
                })
            });
        });
        writer.close_paren();

        static UNARY_OPS : [(&str,&str);1] = [("neg","Neg")];

        writer.write_line("+ UnaryOps");
        writer.open_paren();
        UNARY_OPS.into_iter().for_each(|(op,rep)|{
            self.impl_items.get(op).map(|items|{
                items.iter().for_each(|(_,_)|{
                    writer.write_line(&format!("{rep} self"));
                });
            });
        });
        writer.close_paren();

        self.config.derive_flags.iter().for_each(|flag| {
            writer.write_inline("+ ");
            flag.lines().for_each(|line| {
                writer.write_line(line);
            });
        });

    }


}
