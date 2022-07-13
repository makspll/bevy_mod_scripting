use indexmap::IndexMap;
use rustdoc_types::{Impl, Crate, Item, Id, ItemEnum};
use serde::Deserialize;

use crate::{Newtype, PrettyWriter,Args, Config,type_to_string,is_valid_parameter,is_valid_return_type, to_auto_method_argument, to_op_argument};

pub static WRAPPER_PREFIX : &str = "Lua";

#[derive(Deserialize, Debug,Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapperType {
    /// things which have pass by value semantics
    Value,
    /// things which have pass by reference semantics
    Ref,
    /// For primitives
    Primitive
}

impl ToString for WrapperType {
    fn to_string(&self) -> String {
        match self {
            WrapperType::Value => "Value".to_string(),
            WrapperType::Ref => "Ref".to_string(),
            WrapperType::Primitive => "Primitive".to_string(),
        }
    }
}

impl Default for WrapperType {
    fn default() -> Self {
        Self::Value
    }
}

#[derive(Debug)]
pub struct WrappedItem<'a> {
    pub wrapper_type : WrapperType,
    pub wrapper_name : String,
    pub wrapped_type: &'a String,
    pub path_components: &'a [String],
    pub source: &'a Crate,
    pub config: &'a Newtype,
    pub item : &'a Item,
    /// The items coming from all trait implementations
    pub impl_items: IndexMap<&'a str,Vec<(&'a Impl, &'a Item)>>, 
    pub self_impl: Option<&'a Impl>,
    pub crates : &'a [Crate],
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
    ///  AutoMethods(
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
    pub fn write_derive_flags_body(&self, config: &Config, writer: &mut PrettyWriter, args: &Args) {


        writer.write_line(": Fields");
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
                        let type_string = type_to_string(type_, &|b| {
                            to_auto_method_argument(b, &self.wrapped_type, config, false, WRAPPER_PREFIX)
                        }).ok()?;
                        

                        if is_valid_parameter(&type_string, config, WRAPPER_PREFIX) &&
                            is_valid_return_type(&type_string, config, WRAPPER_PREFIX)    
                        {
                            field_.docs.as_ref().map(|docs| {
                                writer.set_prefix("/// ".into());
                                docs.lines().for_each(|line|{
                                    writer.write_line(line);
                                });
                                writer.clear_prefix();
                            });
                            writer.write_no_newline(name);
                            writer.write_inline(": ");
                            writer.write_inline(&type_string);
                            writer.write_inline(",");
                            writer.newline();
                        }

                        Some(())
                    }).for_each(drop);
            },
            _ => {}
        };

        writer.close_paren();

        writer.write_line("+ AutoMethods");
        writer.open_paren();
        self.impl_items.iter()
            .flat_map(|(_,items)| items.iter())
            .for_each(|(impl_,v)| { 

                // only select trait methods are allowed
                if let Some(trait_) = &impl_.trait_ {
                    if self.config.traits.iter().find(|f| 
                        match type_to_string(trait_, &|s| Ok(s.to_string())).map(|s|&s == &f.name){
                            Ok(true) => true,
                            _ => false,
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
                    .for_each(|(i,(_,tp))| {
                        let type_ = 
                            type_to_string(tp, &|base_string : &String| 
                                to_auto_method_argument(base_string,&self.wrapped_type,config,i==0,WRAPPER_PREFIX));
                        if let Ok(type_) = type_ {
                            if !is_valid_parameter(&type_, config, WRAPPER_PREFIX){
                                inner_writer.write_inline(&format!("<invalid: {type_}>"));
                                errors.push(format!("Unsupported argument {}",type_));
                                return;
                            } else {
                                inner_writer.write_inline(&type_);
                            }
                            if i + 1 != decl.inputs.len() {
                                inner_writer.write_inline(",");
                            }
                        } else {
                            errors.push(format!("Unsupported argument {}",type_.unwrap_err()))
                        };
                });
                inner_writer.write_inline(")");
                
                decl.output
                    .as_ref()
                    .map(|tp| {
                        let type_ = type_to_string(tp, &|base_string : &String| 
                            to_auto_method_argument(base_string,&self.wrapped_type,config,false, WRAPPER_PREFIX));
                        if let Ok(type_) = type_ {
                            if !is_valid_return_type(&type_, config, WRAPPER_PREFIX){
                                errors.push(format!("Unsupported argument {}",type_));
                                inner_writer.write_inline(&format!("<invalid: {type_}>"));
                            } else {
                                inner_writer.write_inline(" -> ");
                                inner_writer.write_inline(&type_);
                            }
                        } else {
                            errors.push(format!("Unsupported argument {}",type_.unwrap_err()))
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
                .filter_map(|(impl_,item)| Some((impl_,item,type_to_string(&impl_.for_,&|s : &String| Ok(s.to_string())).ok()?)) )
                .filter(|(_,_, self_type)| 
                    (self_type == self.wrapped_type && config.types.contains_key(self_type)) 
                        || config.primitives.contains(self_type))
                .for_each(|(impl_,item, self_type)| {
                    let _ = match &item.inner {
                        ItemEnum::Method(m) => {
                            m.decl.inputs
                                .iter()
                                .enumerate()
                                .map(|(idx,(_,t))| {
                                    // check arg is valid
                                    let type_ = type_to_string(t, &|b: &String| 
                                        to_op_argument(b, &self_type, self, &config, idx == 0,false, WRAPPER_PREFIX));

                                    type_.and_then(|type_| {
                                        is_valid_parameter(&type_, config, WRAPPER_PREFIX)
                                            .then_some(type_.to_string())
                                            .ok_or(type_)
                                    })
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

                                    let return_string = type_to_string(out_type, &|b: &String| to_op_argument(b, &self_type, &self, &config, false,true, WRAPPER_PREFIX))?;
                                    
                                    if !is_valid_return_type(&return_string, config, WRAPPER_PREFIX){
                                        return Err(return_string)
                                    }

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
