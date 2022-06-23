use proc_macro2::{Span,TokenStream};
use syn::{*,token::*,parse::*, punctuated::Punctuated};
use quote::{quote,quote_spanned,ToTokens};

use crate::utils::impl_parse_enum;


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
        Unm => {Ok(Self::Unm{ident})},
    }
    
    impl MathOpName {
        pub fn to_rlua_metamethod_path(&self) -> TokenStream {
            quote!{
                rlua::MetaMethod::#self 
            }
        }
        
        pub fn to_rust_method_ident(&self) -> TokenStream {
            
            match self {
                Self::Add{ident} => quote_spanned!{ident.span()=> add},
                Self::Sub{ident} => quote_spanned!{ident.span()=> sub},
                Self::Mul{ident} => quote_spanned!{ident.span()=> mul},
                Self::Div{ident} => quote_spanned!{ident.span()=> div},
                Self::Mod{ident} => quote_spanned!{ident.span()=> rem},
                Self::Unm{ident} => quote_spanned!{ident.span()=> neg},
            }
    
        }
    }
    );
    

#[derive(PartialEq,Eq,Hash)]
pub(crate) struct MathOutKind{
   pub kind : MathOpKind,
   pub paren: Paren,
   pub out_typ: TypePath,
}



impl Parse for MathOutKind {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self {
            kind: input.parse()?,
            paren: parenthesized!(f in input),
            out_typ: f.parse()?,
        })
    }
}


#[derive(PartialEq,Eq,Hash)]
pub(crate) struct MathUnaryOpArg {
    pub out_kind: Option<MathOutKind>
}


impl Parse for MathUnaryOpArg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self{
            out_kind: if input.peek(Token![->]) {input.parse::<Token![->]>()?;Some(input.parse()?)} else {None},
        })
    }
}

impl MathUnaryOpArg {
    pub fn gen_unary_call_expr(&self,default_out_type: &TypePath,math_op: &MathOpName, ud_receiver: &'static str) -> TokenStream{
        let ud_ident = Ident::new(ud_receiver,Span::call_site());

        let rust_op_name = math_op.to_rust_method_ident();

        let inner_expr = quote!{
                // ud on the left op on right
                #ud_ident.#rust_op_name()
        };

        // wrap in constructor
        let inner_expr = self.out_kind.as_ref()
            .map(|o| o.kind.constructor_wrap_val(&inner_expr,&o.out_typ))
            .unwrap_or(
                MathOpKind::LuaWrapper { ident: Ident::new("LuaWrapper", Span::call_site()) }.constructor_wrap_val(&inner_expr,default_out_type));

        quote!{
            return #ud_ident.val(|#ud_ident| {
                Ok(#inner_expr)
            })
        }
    }
}


#[derive(PartialEq,Eq,Hash)]
pub(crate) struct MathBinOpArg {
    pub ampersand_ud: Option<Token![&]>,
    pub self_token: Option<Token![self]>,
    pub paren: Paren,
    pub side: OpSide,
    pub colon: Token![:],
    pub ampersand_op: Option<Token![&]>,
    pub arg_type: TypePath,
    pub out_kind: Option<MathOutKind>,
}

impl Parse for MathBinOpArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            ampersand_ud: if input.peek(Token![&]) {Some(input.parse()?)} else {None},
            self_token: if input.peek(Token![self]) {Some(input.parse()?)} else {None},
            paren: parenthesized!(f in input),
            side: f.parse()?,
            colon: f.parse()?,
            ampersand_op: if f.peek(Token![&]) {Some(f.parse()?)} else {None},
            arg_type: f.parse()?,
            out_kind: if f.peek(Token![->]){f.parse::<Token![->]>()?;Some(f.parse()?)} else {None}
        })
    }
}

impl MathBinOpArg {
    pub fn gen_bin_call_expr(&self,wrap_in_value_enum: bool,default_out_type: &TypePath,math_op: &MathOpName,op_kind: &MathOpKind,ctx_receiver: &'static str, ud_receiver: &'static str, op_receiver: &'static str) -> proc_macro2::TokenStream{
        let ud_ident = Ident::new(ud_receiver,Span::call_site());
        let op_ident = Ident::new(op_receiver,Span::call_site());

        // whether or not we dereference op
        let star_op = self.ampersand_op
            .map(|_| quote!{})
            .unwrap_or(quote!{*});

        let star_ud = self.ampersand_ud
            .map(|_| quote!{})
            .unwrap_or(quote!{*});

        let rust_op_name = math_op.to_rust_method_ident();

        let inner_expr = match self.side {
            OpSide::Rhs{..} => {quote!{
                // ud on the left op on right
                (#star_ud #ud_ident).#rust_op_name(#star_op #op_ident)
            }},
            OpSide::Lhs{..} => {quote!{
                // ud on the right op on left
                (#star_op #op_ident).#rust_op_name(#star_ud #ud_ident)
            }},
        };



        // wrap in constructor
        let inner_expr = self.out_kind.as_ref()
            .map(|o| o.kind.constructor_wrap_val(&inner_expr,&o.out_typ))
            .unwrap_or(
                MathOpKind::LuaWrapper { ident: Ident::new("LuaWrapper", Span::call_site()) }.constructor_wrap_val(&inner_expr,default_out_type));
        
        // wrap in value if necessary
        let inner_expr = wrap_in_value_enum.then(|| 
            self.out_kind
                .as_ref()
                .map(|v| v.kind.wrap_in_value_enum(&inner_expr, ctx_receiver))
                .unwrap_or(MathOpKind::LuaWrapper{ident: Ident::new("LuaWrapper", Span::call_site())}.wrap_in_value_enum(&inner_expr, ctx_receiver))
        ).unwrap_or(inner_expr);

        // wrap in Ok
        let inner_expr = quote!{
            Ok(#inner_expr)
        };

        // wrap in val cal if op is userdata
        let inner_expr = if let MathOpKind::LuaWrapper{..} = op_kind { quote!{
            #op_ident.val(|#op_ident|  #inner_expr)
        }} else {inner_expr};

        quote!{
            return #ud_ident.val(|#ud_ident| {
                #inner_expr
            })
        }
    }
}



#[derive(PartialEq,Eq,Hash)]
pub(crate) struct MathBinOpGroup {
    pub kind: MathOpKind,
    pub bracket: Bracket,
    pub arg_sides: Punctuated<MathBinOpArg,Token![,]>,
}

impl MathBinOpGroup {
    pub fn gen_luawrapper_exprs(&self,wrap_in_value:bool,default_out_type: &TypePath, name: &MathOpName,side: &OpSide,ctx_receiver: &'static str, ud_receiver: &'static str, op_receiver: &'static str) -> proc_macro2::TokenStream{
        let mut generated = false;

        let op_ident = Ident::new(op_receiver,Span::call_site());

        let mut out = self.arg_sides.iter()
            .filter(|a| a.side.to_str() == side.to_str())
            .map(|a| {
                let x = a.gen_bin_call_expr(wrap_in_value,default_out_type,name, &self.kind,ctx_receiver, ud_receiver, op_receiver);
                let t = &a.arg_type;
                let else_tkn = if generated {quote!{else}} else {quote!{}};
                let out = quote!{
                    #else_tkn if let Ok(#op_ident) = #op_ident.borrow::<#t>() {
                        let #op_ident = &#op_ident;
                        #x;
                    }
                };

                generated = true;
                out
            })
            .collect();
        if generated {

            out = quote!{
                if let Value::UserData(#op_ident) = #op_ident {
                    #out;
                };
            };
        };

        out
    }

    pub fn gen_integer_exprs(&self,wrap_in_value:bool,default_out_type: &TypePath, name: &MathOpName,side: &OpSide,ctx_receiver: &'static str, ud_receiver: &'static str, op_receiver: &'static str) -> proc_macro2::TokenStream{
        let ctxt_ident = Ident::new(ctx_receiver,Span::call_site());
        let op_ident = Ident::new(op_receiver, Span::call_site());

        self.arg_sides.iter()
            // we can't have more than one numeric argument as then multiple valid function calls exist here
            .find(|a| a.side.to_str() == side.to_str())
            .map(|a| {
                let x = a.gen_bin_call_expr(wrap_in_value,default_out_type,name, &self.kind,ctx_receiver, ud_receiver, op_receiver);
                let t = &a.arg_type;
                quote!{
                    // we can clone here since on the good path, this is just a copy type
                    // on the bad path we error out anyway
                    if let Ok(Some(#op_ident)) = #ctxt_ident.coerce_integer(#op_ident.clone()) {
                        let #op_ident = &(#op_ident as #t);
                        #x;
                    };
                }
            }).unwrap_or(proc_macro2::TokenStream::new())
    }

    pub fn gen_number_exprs(&self,wrap_in_value:bool,default_out_type: &TypePath, name: &MathOpName,side: &OpSide,ctx_receiver: &'static str, ud_receiver: &'static str, op_receiver: &'static str) -> proc_macro2::TokenStream{
        let ctxt_ident = Ident::new(ctx_receiver,Span::call_site());
        let op_ident = Ident::new(op_receiver, Span::call_site());

        self.arg_sides.iter()
            // we can't have more than one numeric argument as then multiple valid function calls exist here
            .find(|a| a.side.to_str() == side.to_str())
            .map(|a| {
                let x = a.gen_bin_call_expr(wrap_in_value,default_out_type,name, &self.kind,ctx_receiver, ud_receiver, op_receiver);
                let t = &a.arg_type;
                quote!{
                    // we can clone here since on the good path, this is just a copy type
                    // on the bad path we error out anyway
                    if let Ok(Some(#op_ident)) = #ctxt_ident.coerce_number(#op_ident.clone()) {
                        let #op_ident = &(#op_ident as #t);
                        #x;
                    };
                }
            }).unwrap_or(proc_macro2::TokenStream::new())
        }

}

impl Parse for MathBinOpGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;

        Ok(Self {
            kind: input.parse()?,
            bracket: bracketed!(f in input),
            arg_sides: f.parse_terminated(MathBinOpArg::parse)?,
        })
    }
}

impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash,Clone)]
pub(crate) enum MathOpKind {
    Number => {Ok(Self::Number{ident})},
    Integer => {Ok(Self::Integer{ident})},
    LuaWrapper => {Ok(Self::LuaWrapper{ident})},
}

impl MathOpKind {
    pub fn constructor_wrap_val(&self, receiver: &TokenStream, typ: &TypePath) -> TokenStream{
        match self {
            Self::LuaWrapper{..} => {
            quote!{
                #typ::new(#receiver)
            }
            },
            _ => quote!{#receiver}
        }
    }

    pub fn wrap_in_value_enum(&self, receiver: &TokenStream, ctx_receiver:&'static str) -> TokenStream {
        let ctx_ident = Ident::new(ctx_receiver,Span::call_site());
        match self {
            MathOpKind::Number { .. } => quote!{
                Value::Number(#receiver as f64)
            },
            MathOpKind::Integer { .. } => quote!{
                Value::Integer(#receiver as i64)
            },
            MathOpKind::LuaWrapper { .. } => quote!{
                Value::UserData(#ctx_ident.create_userdata(#receiver)?)
            },
        }
    }
}
);


