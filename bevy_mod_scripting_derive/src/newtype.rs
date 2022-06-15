use std::collections::{HashSet, HashMap};

use proc_macro2::{TokenStream, Span};
use syn::{*, punctuated::*, token::*, parse::{ParseStream, Parse}, spanned::Spanned};

use crate::{AdditionalImplBlock, lua_block::{LuaBlock, LuaMethod,LuaMethodType}, impls::DeriveFlag, utils::impl_parse_enum};
use quote::{quote, ToTokens, quote_spanned};


pub(crate)  struct NewtypeArgs {
    pub full_base_type: TypePath,
    pub type_colon : Token![:],
    pub short_base_type: TypePath,
    pub short_wrapper_type: TypePath,
    pub variation: NewtypeVariation,
    pub colon: Option<Token![:]>,
    pub flags: HashSet<DeriveFlag>
}


impl Parse for NewtypeArgs {
    fn parse(input: ParseStream) -> Result<Self>{
        let base_type : TypePath = input.parse()?;
        let short_base_type : String = base_type.path.segments.last().ok_or(input.error("Path does not have identifier"))?.ident.to_string();
        let short_wrapper_type : String = format!("Lua{}",short_base_type);
        let sbt_ident = Ident::new(&short_base_type,Span::call_site());
        let swt_ident = Ident::new(&short_wrapper_type, Span::call_site());
        let mut colon : Option<Token![:]> = None;
        Ok(Self {
            short_wrapper_type: parse_quote_spanned!{base_type.span()=>
                #swt_ident
            },
            short_base_type: parse_quote_spanned!{base_type.span()=>
                #sbt_ident
            },
            full_base_type: base_type,
            type_colon: input.parse()?,
            variation: input.parse()?,
            colon: if input.peek(Token![:]){colon = Some(input.parse()?); colon} else {None},
            flags: {
                if colon.is_some(){
                    Punctuated::<DeriveFlag, Token![+]>::parse_separated_nonempty(input)?
                    .into_iter()
                    .collect::<HashSet<DeriveFlag>>()
                } else {
                    HashSet::default()
                }
            },
        })
    }
}


impl_parse_enum!(input,ident:
pub(crate) enum NewtypeVariation {
    Full => {Ok(Self::Full {ident})},
    Primitive => {Ok(Self::Primitive{ident})},
    NonAssignable{
        braces: Brace,
        fields: Punctuated<Field,Token![,]>
    } => {
        let f;
        Ok(Self::NonAssignable{
            ident,
            braces: braced!(f in input), 
            fields: f.parse_terminated(Field::parse_named)?,
        })
    },
}
);

pub(crate) struct Newtype {
    pub braces: Brace,
    pub args: NewtypeArgs,
    pub additional_functions: Option<AdditionalImplBlock>,
    pub additional_lua_functions: Option<LuaBlock>
}

impl Newtype {
    pub fn to_from_lua_entry(&self) -> Option<TokenStream> {
        let k = &mut self.args.full_base_type.clone().into_token_stream().to_string();
        k.retain(|c| !c.is_whitespace());
        let wrapper_type = &self.args.short_wrapper_type;

        if self.args.variation.is_full() {
            Some(quote_spanned!(self.args.full_base_type.span()=>
                #k => |r,c,n| {
                    if let Value::UserData(v) = n {
                        let mut v = v.borrow_mut::<#wrapper_type>()?;
                        #wrapper_type::apply_self_to_base(v.deref_mut(),r);
                        Ok(())
                    } else {
                        Err(Error::RuntimeError("Invalid type".to_owned()))
                    }
                
            }))
        } else if self.args.variation.is_primitive() {
            self.additional_lua_functions.as_ref().map(|v| {
                let from = v.functions.iter().find(|f| {
                    if let LuaMethodType::Method(s) = &f.method_type {
                        if s.value() == "from"{
                            return true
                        } 
                    } 
                    false
                }).expect("").closure.to_applied_closure();
                quote_spanned!(self.args.full_base_type.span()=>
                    #k => #from
                )
            })
        } else {
            None
        }
    }

    pub fn to_to_lua_entry(&self) -> Option<TokenStream>{
        let k = &mut self.args.full_base_type.clone().into_token_stream().to_string();
        k.retain(|c| !c.is_whitespace());
        let wrapper_type = &self.args.short_wrapper_type;

        if self.args.variation.is_full() {
            Some(quote_spanned!(self.args.full_base_type.span()=>
                #k => |r,c| {
                    let usr = c.create_userdata(#wrapper_type::base_to_self(r)).unwrap();
                    Value::UserData(usr)
                }
            ))
        } else if self.args.variation.is_primitive() {
            self.additional_lua_functions.as_ref().map(|v| {
                let to = v.functions.iter().find(|f| {
                    if let LuaMethodType::Method(s) = &f.method_type {
                        if s.value() == "to"{
                            return true
                        } 
                    } 
                    false
                }).expect("").closure.to_applied_closure();
                quote_spanned!(self.args.full_base_type.span()=>
                    #k => #to
                )
            })
        } else {
            None
        }
    }
}


impl Parse for Newtype {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;

        Ok(Self {
            braces: braced!(f in input),
            args: f.parse()?,
            additional_lua_functions: if f.peek(Token![impl]) && !f.peek2(Token![fn]){
                    Some(f.parse()?)
                } else {
                    None
                },
            additional_functions:  if f.peek(Token![impl]) && f.peek2(Token![fn]) {
                    Some(f.parse()?)
                } else {
                    None
                },

        })
    }
}


impl Newtype {

    /// Applies all macros, fills in hashmap and returns entire newtype implementation
    pub fn to_applied_tokens(&self, all_methods:&mut HashMap<String,Vec<LuaMethod>> ) -> TokenStream {
        let name : &Ident = &self.args.short_wrapper_type.path.get_ident().unwrap();

        let functions = self.additional_functions.as_ref().map(|v| &v.functions);

        let trait_impls = self.args.flags.iter()
            .filter_map(|s| s.into_impl_block(&self.args));

        let mut our_functions : Vec<LuaMethod> = self.additional_lua_functions
            .as_ref()
            .map(|x|x.functions.iter().cloned().collect())
            .unwrap_or_default();

        our_functions.extend(self.args.flags.iter()
            .filter_map(|s| s.into_lua_block(&self.args,&all_methods))
            .flat_map(|m : LuaBlock| m.functions.iter().cloned().collect::<Vec<LuaMethod>>()));

        // generate call expressions
        let call_exprs : Punctuated<TokenStream,Token![;]> = our_functions.iter()
            .filter_map(|f| f.to_call_expr("methods"))
            .collect();

        // append all those functions to global map
        all_methods.insert(self.args.short_wrapper_type.clone().into_token_stream().to_string(), our_functions);

        
        let base_type = &self.args.full_base_type.path.segments.last().unwrap().ident;

        let struct_def = match &self.args.variation {
            NewtypeVariation::Full{..} => Some(quote!{
                pub type #name = crate::LuaWrapper<#base_type>;
            }),
            NewtypeVariation::NonAssignable { fields , ..} => Some(quote!{
                #[derive(Clone)]
                pub struct #name{#fields}
            }),
            NewtypeVariation::Primitive{..} => None
        };

        let impl_block = struct_def.as_ref()
            .and_then(|_| functions)
            .and_then(|f| 
                Some(quote!{
                    impl #name {
                        #f
                    }
                })
        );

        let lua_impl_block = struct_def.as_ref().map(|_| 
            quote!{
                impl rlua::UserData for #name {
                    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
                        #call_exprs;
                    }
                }
            }
        );

        quote!{
            #struct_def
            #impl_block
            #lua_impl_block
            #(#trait_impls)*
        }

    }
}



