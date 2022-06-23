
use proc_macro2::{Span, TokenStream};
use syn::{*, parse::{ParseStream, Parse}, token::{Brace, Paren}, punctuated::Punctuated, spanned::Spanned};
use quote::{quote, ToTokens, quote_spanned};
use convert_case::{Case, Casing};

pub(crate) trait ToLuaMethod {
    fn to_lua_method(self) -> LuaMethod;
}




#[derive(Debug)]
pub(crate) struct UserdataBlock {
    pub impl_token: Token![impl],
    pub impl_braces: Brace,
    pub functions: Punctuated<LuaMethod,Token![;]>
}

impl Parse for UserdataBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;

        Ok(Self{
            impl_token: input.parse()?,
            impl_braces: braced!(f in input),
            functions: f.parse_terminated(LuaMethod::parse)?,
        })
    }
}

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
pub(crate) struct MethodMacroArg {
    pub ident: Ident,
    pub equals: Token![=],
    pub replacement: TypePath,
}


impl Parse for MethodMacroArg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self{
            ident: input.parse()?,
            equals: input.parse()?,
            replacement: input.parse()?
        })
    }
}

impl ToTokens for MethodMacroArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = &self.ident;
        let rep = &self.replacement;
        tokens.extend(quote!{
            #id = #rep
        })
    }
}

#[derive(PartialEq,Eq,Clone,Hash,Debug)]
pub(crate) enum LuaMethodType {
    MetaMethod{
        paren: Paren,
        path: TypePath
    },
    MetaMethodMut{
        paren: Paren,
        path: TypePath
    },
    MetaFunction{
        paren: Paren,
        path: TypePath
    },
    MetaFunctionMut{
        paren: Paren,
        path: TypePath
    },
    Method(LitStr),
    MethodMut(LitStr),
    Global(LitStr),
}

impl Parse for LuaMethodType {
    fn parse(input: ParseStream) -> Result<Self> {


        let function_variant = input.peek(Token![fn]);
        if function_variant {
            input.parse::<Token![fn]>()?;
        }

        let mutable = input.peek(Token![mut]);

        let global = input.peek(Token![static]);

        if mutable {
            input.parse::<Token![mut]>()?;
        } else if global {
            input.parse::<Token![static]>()?;
            return Ok(Self::Global(input.parse()?));
        };

        Ok(match (function_variant,input.peek(Paren),mutable) {
            // fn mut metamethod
            (false,true, true) => {
                let f;
                Self::MetaMethodMut{ 
                    paren: parenthesized!(f in input), 
                    path: f.parse()?, 
                }
            },
            // mut metamethod
            (false,true, false) => {
                let f;
                Self::MetaMethod{
                    paren: parenthesized!(f in input),
                    path: f.parse()?
                }
            },
            // fn mut metamethod
            (true,true, true) => {
                let f;
                Self::MetaFunctionMut{ 
                    paren: parenthesized!(f in input), 
                    path: f.parse()?, 
                }
            },
            // fn metamethod
            (true,true, false) => {
                let f;
                Self::MetaFunction{
                    paren: parenthesized!(f in input),
                    path: f.parse()?
                }
            },
            // mut method
            (false,false, true) => Self::MethodMut(input.parse()?),
            // method
            (false,false, false) => Self::Method(input.parse()?),
            _ => panic!("Invalid lua closure type")
        })
    }
}

impl ToTokens for LuaMethodType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let q = match self {
            LuaMethodType::MetaMethod{path , ..} => quote!{(#path)},
            LuaMethodType::MetaMethodMut{path , ..} => quote!{mut (#path)},
            LuaMethodType::MetaFunction { path , ..} => quote!{fn (#path)},
            LuaMethodType::MetaFunctionMut { path, .. } => quote!{fn mut (#path)},
            LuaMethodType::Method(x) => quote!{#x},
            LuaMethodType::MethodMut(x) => quote!{mut #x},
            LuaMethodType::Global(x) => quote!{static #x},
        };

        tokens.extend(quote!{
            #q
        })
    }
}

#[derive(Clone,Debug)]
pub(crate) enum LuaClosure {
    MetaClosure{
        paren: Paren,
        args: Punctuated<MethodMacroArg,Token![,]>,
        arrow: Token![=>],
        brace: Brace,
        expr: TokenStream,
    },
    PureClosure{
        arrow: Token![=>],
        expr: ExprClosure
    },
}

impl Parse for LuaClosure {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(token::Paren){
            let f;
            let g;
            Ok(Self::MetaClosure{
                paren: parenthesized!(f in input),
                args: f.parse_terminated(MethodMacroArg::parse)?,
                arrow: input.parse()?,
                brace: braced!(g in input),
                expr: g.parse()?,
            })
        } else {
            
            Ok(Self::PureClosure{
                arrow: input.parse()?,
                expr: input.parse()?
            })
        }
    }
} 

impl LuaClosure {
    pub fn to_applied_closure(&self) -> TokenStream{
        match self {
            LuaClosure::MetaClosure { 
                paren, 
                args, 
                expr,
                .. } => {
                    
                quote!{
                    replace!{#args : #expr}
                }
            },
            LuaClosure::PureClosure { expr, .. } => {
                quote!{
                    #expr
                }
            },
        }
    }
}

impl ToTokens for LuaClosure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LuaClosure::MetaClosure { 
                paren, 
                args, 
                expr,
                .. } => {
                    
                tokens.extend(quote!{
                    (#args) => {#expr} 
                })
            },
            LuaClosure::PureClosure { expr, .. } => {
                tokens.extend(quote!{
                    => #expr
                })
            },
        }
    }
}

#[derive(Clone,Debug)]
pub(crate) struct Test {
    pub brace: Brace,
    pub ts: TokenStream
}

impl Parse for Test {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            brace: braced!(f in input),
            ts: f.parse()?,
        })
    }
}

impl ToTokens for Test {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = &self.ts;
        tokens.extend(quote!{
            {#ts}
        })
    }
}

#[derive(Clone,Debug)]
pub(crate) struct LuaMethod {
    pub method_type: LuaMethodType,
    pub closure: LuaClosure,
    pub test: Option<Test>
}


impl Parse for LuaMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {  
            method_type: input.parse()?,
            closure: input.parse()?,
            test: if input.peek(Token![=>]) {
                input.parse::<Token![=>]>()?;
                Some(input.parse()?)
            } else {None}
        })
    }
}

impl ToTokens for LuaMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mt = &self.method_type;
        let closure = &self.closure;
        let test = self.test.as_ref().map(|t| quote!{
            => #t
        });
        tokens.extend(quote!{
            #mt #closure #test
        })
    }
}

impl LuaMethod {


    pub fn gen_tests(&self, newtype_name : &str) -> Option<TokenStream>{
        self.test.as_ref().map(|v| {

            let fun = v.ts.clone();
            let test_ident = Ident::new(&format!{"{}",
                newtype_name.to_case(Case::Snake)
            },Span::call_site()); 

            match &self.closure{
                LuaClosure::MetaClosure { args, .. } => {
                    quote!{
                        replace!{#args : #fun}
                    }},
                LuaClosure::PureClosure {..} => {
                    fun.clone().into_token_stream()
                },
            }
        })
    }

    pub fn to_call_expr(&self,receiver : &'static str) -> Option<TokenStream>{
        let closure = &self.closure.to_applied_closure(); 
        let receiver = Ident::new(receiver,Span::call_site());
        match &self.method_type {
            LuaMethodType::MetaMethod{ path , ..} => Some(quote_spanned!{closure.span()=>
                #receiver.add_meta_method(#path,#closure);
            }),
            LuaMethodType::MetaMethodMut{path , ..} => Some(quote_spanned!{closure.span()=>
                #receiver.add_meta_method_mut(#path,#closure);
            }),
            LuaMethodType::MetaFunction{ path , ..} => Some(quote_spanned!{closure.span()=>
                #receiver.add_meta_function(#path,#closure);
            }),
            LuaMethodType::MetaFunctionMut{path , ..} => Some(quote_spanned!{closure.span()=>
                #receiver.add_meta_function_mut(#path,#closure);
            }),
            LuaMethodType::Method(v) => Some(quote_spanned!{closure.span()=>
                #receiver.add_method(#v,#closure);
            }),
            LuaMethodType::MethodMut(v) => Some(quote_spanned!{closure.span()=>
                #receiver.add_method_mut(#v,#closure);
            }),

            _ => None,
        }
    }

    pub fn rebind_macro_args<'a,I: Iterator<Item = &'a MethodMacroArg> + Clone>(&mut self, o : I) -> Result<()>{
        if let LuaClosure::MetaClosure { ref mut args, ..}  = self.closure {
            for other_arg in o{
                // validate the argument
                let corresponding = args.iter_mut()
                    .find(|oa| oa.ident.to_string() == other_arg.ident.to_string())
                    .ok_or(Error::new(Span::call_site(),format!("Invalid argument in macro invocation `{}`. No corresponding variable.",other_arg.ident)))?;

                *corresponding = other_arg.clone()
            }
            Ok(())
        } else {
            Err(Error::new(Span::call_site(),"Attempted to invoke macro args on non-meta closure"))
        }
    }

    pub fn to_create_static_expr(&self, global_receiver : &'static str, context_receiver: &'static str) -> Option<TokenStream> {

        let closure = &self.closure.to_applied_closure();
        let global_receiver = Ident::new(global_receiver,Span::call_site());
        let context_receiver = Ident::new(context_receiver, Span::call_site());

        match &self.method_type {
            LuaMethodType::Global(v) => Some(quote_spanned!{closure.span()=>
                 #global_receiver.set(#v, #context_receiver.create_function(#closure)?)?;
            }),
            _ => None,
        }
    }
}



