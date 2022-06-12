
use std::collections::{HashSet, HashMap};

use proc_macro2::Span;
use syn::{*, parse::*, punctuated::*, token::*};

use crate::{lua_method::{LuaBlock, LuaMethodType, MethodMacroArg, LuaMethod, LuaClosure},TokenStream2, newtype::NewtypeArgs, utils::impl_parse_enum};
use paste::paste;


use quote::{ToTokens,quote, quote_spanned};



#[derive(PartialEq,Eq,Hash)]
pub(crate) struct MathOutKind{
    arrow: Token![->],
    kind : MathOpKind,
    paren: Paren,
    out_typ: TypePath,
}


impl Parse for MathOutKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self {
            arrow: input.parse()?,
            kind: input.parse()?,
            paren: parenthesized!(f in input),
            out_typ: f.parse()?,
        })
    }
}



#[derive(PartialEq,Eq,Hash)]
pub(crate) struct MathOpArg {
    paren: Paren,
    ampersand: Option<Token![&]>,
    side: OpSide,
    colon: Token![:],
    arg_type: TypePath,
    out_kind: Option<MathOutKind>,
}

impl MathOpArg {
    pub fn gen_bin_call_expr(&self,default_out_type: &TypePath,math_op: &MathOpName,op_kind: &MathOpKind, ud_receiver: &'static str, op_receiver: &'static str) -> proc_macro2::TokenStream{
        let ud_ident = Ident::new(ud_receiver,Span::call_site());
        let op_ident = Ident::new(op_receiver,Span::call_site());
        
        // whether or not we dereference op
        let star = self.ampersand
            .map(|_| quote!{})
            .unwrap_or(quote!{*});

        let rust_op_name = math_op.to_rust_method_ident();

        let inner_expr = match self.side {
            OpSide::Rhs{..} => {quote!{
                // ud on the left op on right
                #ud_ident.#rust_op_name(#star #op_ident)
            }},
            OpSide::Lhs{..} => {quote!{
                // ud on the right op on left
                #op_ident.#rust_op_name(#star #ud_ident)
            }},
        };



        // wrap in constructor
        let inner_expr = self.out_kind.as_ref()
            .map(|o| o.kind.constructor_wrap_val(&inner_expr,&o.out_typ))
            .unwrap_or(
                MathOpKind::LuaWrapper { ident: Ident::new("LuaWrapper", Span::call_site()) }.constructor_wrap_val(&inner_expr,default_out_type));

        // wrap in Ok
        let inner_expr = quote!{
            Ok(#inner_expr)
        };

        // wrap in val cal if op is userdata
        let inner_expr = if let MathOpKind::LuaWrapper{..} = op_kind { quote!{
            #op_ident.val(|#op_ident|  #inner_expr)
        }} else {inner_expr};

        quote!{
            #ud_ident.bin(#op_ident,|#ud_ident,#op_ident| {
                #inner_expr
            })
        }
    }
}

impl Parse for MathOpArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            paren: parenthesized!(f in input),
            ampersand: if f.peek(Token![&]) {Some(f.parse()?)} else {None},
            side: f.parse()?,
            colon: f.parse()?,
            arg_type: f.parse()?,
            out_kind: if f.peek(Token![->]){Some(f.parse()?)} else {None}
        })
    }
}

#[derive(PartialEq,Eq,Hash)]
pub(crate) struct MathOpGroup {
    kind: MathOpKind,
    bracket: Bracket,
    arg_sides: Punctuated<MathOpArg,Token![,]>,
}

impl MathOpGroup {
    pub fn gen_bin_call_exprs(&self,default_out_type: &TypePath, name: &MathOpName,side: &OpSide, ud_receiver: &'static str, op_receiver: &'static str) -> proc_macro2::TokenStream{
        let mut generated = false;
        let mut out = self.arg_sides.iter()
            .filter(|a| a.side.to_str() == side.to_str())
            .map(|a| {
                let x = a.gen_bin_call_expr(default_out_type,name, &self.kind, "ud", "op");
                let t = &a.arg_type;
                let else_tkn = if generated {quote!{else}} else {quote!{}};
                let out = quote!{
                    #else_tkn if let Ok(op) = op.borrow::<#t>() {
                        let op = &op;
                        return #x;
                    }
                };

                generated = true;
                out
            })
            .collect();
        if generated {
            out = quote!{
                if let Value::UserData(op) = op {
                    #out;
                };
            };
        };

        out
    }
}

impl Parse for MathOpGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;

        Ok(Self {
            kind: input.parse()?,
            bracket: bracketed!(f in input),
            arg_sides: f.parse_terminated(MathOpArg::parse)?,
        })
    }
}

impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash)]
pub(crate) enum MathOpKind {
    Number => {Ok(Self::Number{ident})},
    Integer => {Ok(Self::Integer{ident})},
    LuaWrapper => {Ok(Self::LuaWrapper{ident})},
}

impl MathOpKind {
    pub fn constructor_wrap_val(&self, receiver: &TokenStream2, typ: &TypePath) -> TokenStream2{
        match self {
            Self::LuaWrapper{..} => {
            quote!{
                #typ::new(#receiver)
            }
            },
            _ => quote!{#receiver}
        }

    }
}
);


impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash)]
pub(crate) enum OpSide{
    Lhs => {Ok(Self::Lhs{ident})},
    Rhs => {Ok(Self::Rhs{ident})} 
}
);

impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash)]
pub(crate) enum MathOpName {
    Add => {Ok(Self::Add{ident})},
    Sub => {Ok(Self::Sub{ident})},
    Mul => {Ok(Self::Mul{ident})},
    Div => {Ok(Self::Div{ident})},
    Mod => {Ok(Self::Mod{ident})},
}

impl MathOpName {
    pub fn to_rlua_metamethod_path(&self) -> TokenStream2 {
        quote!{
            rlua::MetaMethod::#self 
        }
    }
    
    pub fn to_rust_method_ident(&self) -> TokenStream2 {
        
        match self {
            Self::Add{ident} => quote_spanned!{ident.span()=> add},
            Self::Sub{ident} => quote_spanned!{ident.span()=> sub},
            Self::Mul{ident} => quote_spanned!{ident.span()=> mul},
            Self::Div{ident} => quote_spanned!{ident.span()=> div},
            Self::Mod{ident} => quote_spanned!{ident.span()=> rem},
        }

    }
}
);

#[derive(PartialEq,Eq,Hash,Debug)]
pub(crate) struct MethodMacroInvokation{
    target: TypePath,
    arrow: Token![->],
    identifier: LuaMethodType,
    paren: Paren,
    args : Punctuated<MethodMacroArg,Token![,]>
}

impl Parse for MethodMacroInvokation {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            target: input.parse()?,
            arrow: input.parse()?,
            identifier: input.parse()?,
            paren: parenthesized!(f in input),
            args: f.parse_terminated(MethodMacroArg::parse)?,
        })
    }
}

impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash)]
pub(crate) enum DeriveFlag {
    
    DebugToString => {Ok(Self::DebugToString{ident})},
    DisplayToString => {Ok(Self::DisplayToString{ident})},
    Copy{
        paren : Paren,
        invocations: Punctuated<MethodMacroInvokation,Token![,]>
    } => {
        let f;
        Ok(Self::Copy{
            ident,
            paren: parenthesized!(f in input),
            invocations: f.parse_terminated(MethodMacroInvokation::parse)?,
        })
    },
    MathBinOp {
        paren : Paren,
        name: MathOpName,
        colon: Token![:],
        ops: Punctuated<MathOpGroup,Token![,]>
    } => {
        let f;
        Ok(Self::MathBinOp{
            ident,
            paren: parenthesized!(f in input),
            name: f.parse()?,
            colon: f.parse()?, 
            ops: f.parse_terminated(MathOpGroup::parse)? 
        })
    }
}
);
    

    
impl DeriveFlag {
    pub fn into_impl_block(&self, newtype_args : &NewtypeArgs) -> Option<ItemImpl>{

        let wrapper_type = &newtype_args.short_wrapper_type;

        match self {
            _ => None
        }
    }

    pub fn into_lua_block(&self, newtype_args : &NewtypeArgs, all_methods: &HashMap<String,Vec<LuaMethod>>) -> Option<LuaBlock> {

        match self {
            DeriveFlag::DebugToString{ident} => Some(parse_quote_spanned!{ident.span()=>
                impl {
                    (rlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{:?}",s));
                }
            }),
            DeriveFlag::DisplayToString{ident} => Some(parse_quote_spanned!{ident.span()=>
                impl {
                    (rlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{}",s));
                }
            }),
            DeriveFlag::Copy { ident, paren, invocations } =>{ 
                let mut new_methods = Vec::default();
                for i in invocations{
                    let key = &i.target;
                    let key = quote!{#key}.to_string();

                    let methods = all_methods.get(&key).expect(&format!("Target lua wrapper type `{}` not found",key));

                    let mut found = false;
                    for m in methods {
                        if i.identifier == m.method_type {
                            found = true;
                            // hit apply replacements
                            let mut new_method = m.clone();

                            new_method.rebind_macro_args(i.args.iter()).unwrap();

                            new_methods.push(new_method);
                        }
                    }
                    if !found {
                        panic!("Could not find Method `{}` in target `{}`",i.identifier.to_token_stream(), i.target.to_token_stream());
                    }
                };
                let new_methods: Punctuated<LuaMethod,Token![;]> = new_methods.into_iter().collect();
                Some(parse_quote_spanned!{ident.span()=>
                    impl {
                        #new_methods
                    }
                })
            },
            DeriveFlag::MathBinOp {ident ,name , ops, .. } =>  {  
                let (lua_wrapper_group,rest)  = ops.into_iter()
                                                                        .partition::<Vec<_>,_>(|x| x.kind.is_lua_wrapper());
                let (numbers_group,integer_group) = rest.into_iter()
                                                .partition::<Vec<_>,_>(|x| x.kind.is_number());
                
                if lua_wrapper_group.len() > 1 || numbers_group.len() > 1 || integer_group.len() > 1 {
                    panic!("Expected at most one instance of LuaWrapper, Number and Integer argument groups")
                };

                let lua_wrapper_group = lua_wrapper_group.first();
                let numbers_group = numbers_group.first();
                let integer_group = integer_group.first();

                let newtype = &newtype_args.short_wrapper_type;

                let body_ud_borrow = quote_spanned! {ident.span()=>
                    // figure out which side is the newtype

                    let (ud,op_on_rhs) : (&rlua::AnyUserData,bool) = match (&lhs,&rhs) {
                        (Value::UserData(v),_) => (v,true),
                        (_,Value::UserData(v)) => (v,false),
                        _ => panic!("Something went wrong"),   
                    };

                    // borrow the newtype
                    let ud: &#newtype = &ud.borrow::<#newtype>().unwrap();
                };


                // we check in order of: userdata -> integer -> number in each case
                let (userdata_handlers_lhs,userdata_handlers_rhs) = lua_wrapper_group
                    .map(|x| {
                        let lhs = x.gen_bin_call_exprs(&newtype_args.short_wrapper_type,&name,&OpSide::Lhs { ident: Ident::new("Lhs",Span::call_site()) },"ud","op");
                        let rhs = x.gen_bin_call_exprs(&newtype_args.short_wrapper_type,&name,&OpSide::Rhs { ident: Ident::new("Rhs",Span::call_site()) },"ud","op");
                        (lhs,rhs)
                    })
                    .unwrap_or((proc_macro2::TokenStream::new(),proc_macro2::TokenStream::new()));

                let op_handling = quote_spanned!{ident.span()=>
                    // userdata handling
                    if op_on_rhs {
                        let op = &rhs;
                        #userdata_handlers_rhs
                    } else {
                        let op = &lhs;
                        #userdata_handlers_lhs
                    };

                    return Err(rlua::Error::RuntimeError("Attempted to perform invalid arithmetic with userdata".to_owned()))
                };

                Some(parse_quote_spanned! {ident.span()=>
                    impl {
                        fn (rlua::MetaMethod::#name) => |_,(lhs,rhs) :(Value,Value)| {
                            #body_ud_borrow
                            #op_handling
                        };
                    }
                })
                
            }
        }
    }
}





