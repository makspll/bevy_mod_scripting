use proc_macro2::Span;
use syn::{*, parse::{ParseStream, Parse}, token::Brace, punctuated::Punctuated};

pub(crate) trait ToLuaMethod {
    fn to_lua_method(self) -> LuaMethod;
}


pub(crate) struct LuaBlock {
    pub impl_token: Token![impl],
    pub impl_braces: Brace,
    pub functions: Punctuated<LuaMethod,Token![;]>
}


impl Parse for LuaBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            impl_token: input.parse()?,
            impl_braces: braced!(f in input),
            functions: f.parse_terminated(LuaMethod::parse)?,
        })
    }
}

pub(crate) struct LuaMethod {
    pub method_type: LuaMethodType,
    pub arrow: Token![=>],
    pub closure: ExprClosure
}

impl Parse for LuaMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            method_type: input.parse()?,
            arrow: input.parse()?,
            closure: input.parse()?,
        })
    }
}

pub(crate) enum LuaMethodType {
    MetaMethod(Path),
    MetaMethodMut(Path),
    Function(LitStr),
    FunctionMut(LitStr)
}

impl Parse for LuaMethodType {
    fn parse(input: ParseStream) -> Result<Self> {
        
        let mutable = input.peek(Token![mut]);

        if mutable {
            input.parse::<Token![mut]>()?;
        };

        Ok(match (input.peek(Ident),mutable) {
            (true, true) => Self::MetaMethodMut(input.parse()?),
            (true, false) => Self::MetaMethod(input.parse()?),
            (false, true) => Self::FunctionMut(input.parse()?),
            (false, false) => Self::Function(input.parse()?),
        })
    }
}

impl LuaMethod {
    pub fn to_call_expr(&self,receiver : &'static str) -> ExprMethodCall{
        let closure = &self.closure;
        let receiver = Ident::new(receiver,Span::call_site());

        match &self.method_type {
            LuaMethodType::MetaMethod(v) => parse_quote!{
                #receiver.add_meta_method(#v,#closure)
            },
            LuaMethodType::MetaMethodMut(v) => parse_quote!{
                #receiver.add_meta_method_mut(#v,#closure)
            },
            LuaMethodType::Function(v) => parse_quote!{
                #receiver.add_function(#v,#closure)
            },
            LuaMethodType::FunctionMut(v) => parse_quote!{
                #receiver.add_function_mut(#v,#closure)
            },
        }
    }
}


