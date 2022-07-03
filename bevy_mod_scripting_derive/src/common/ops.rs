use proc_macro2::{Span,TokenStream};
use syn::{*,Type,token::*,parse::*, punctuated::Punctuated, spanned::Spanned};
use quote::{quote_spanned,ToTokens};

use super::impl_parse_enum;



    

impl_parse_enum!(input,ident:
    #[derive(PartialEq,Eq,Hash,Clone,Debug)]
    pub(crate) enum OpName {
        Add => {Ok(Self::Add{ident})},
        Sub => {Ok(Self::Sub{ident})},
        Mul => {Ok(Self::Mul{ident})},
        Div => {Ok(Self::Div{ident})},
        Rem => {Ok(Self::Rem{ident})},
        Neg => {Ok(Self::Neg{ident})},

    }
    
    impl MathOpName {
        pub fn to_rlua_metamethod_path(&self) -> TokenStream {
            match self {
                OpName::Add { ident } => quote_spanned!(ident.span()=> Add),
                OpName::Sub { ident } => quote_spanned!(ident.span()=> Sub),
                OpName::Mul { ident } => quote_spanned!(ident.span()=> Mul),
                OpName::Div { ident } => quote_spanned!(ident.span()=> Div),
                OpName::Rem { ident } => quote_spanned!(ident.span()=> Mod),
                OpName::Neg { ident } => quote_spanned!(ident.span()=> Unm),

            }
        }
        
        pub fn to_rust_method_ident(&self) -> TokenStream {
            match self {
                Self::Add{ident} => quote_spanned!{ident.span()=> add},
                Self::Sub{ident} => quote_spanned!{ident.span()=> sub},
                Self::Mul{ident} => quote_spanned!{ident.span()=> mul},
                Self::Div{ident} => quote_spanned!{ident.span()=> div},
                Self::Rem{ident} => quote_spanned!{ident.span()=> rem},
                Self::Neg{ident} => quote_spanned!{ident.span()=> neg},

            }
        }
    }
    );
    

    


#[derive(PartialEq,Eq,Hash,Debug)]
pub(crate) enum  OpArg {
    Self_(Receiver),
    Arg(Type)
}

impl Parse for OpArg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(if Receiver::parse(&input.fork()).is_ok(){
            Self::Self_(input.parse()?)
        } else {
            Self::Arg(Type::parse(input)?)
        })
    }
}

impl ToTokens for OpArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            OpArg::Self_(s) => tokens.extend(quote::quote!{
                #s
            }),
            OpArg::Arg(s) => tokens.extend(quote::quote!{
                #s
            }),
        }
    }
}


#[derive(PartialEq,Eq,Hash,Debug)]
pub(crate) struct OpExpr {
    pub left : Option<OpArg>,
    pub op : OpName,
    pub right : OpArg,
    pub return_type : Option<(Token![->],Type)>
}

impl ToTokens for OpExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let left = &self.left;
        let op = &self.op;
        let right = &self.right;
        let return_ = self.return_type.as_ref().map(|(a,t)| quote::quote!{#a #t});

        tokens.extend(quote::quote!{
            #left #op #right #return_
        })  
    }
}


impl Parse for OpExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self{
            left: (&input.fork()).parse::<OpName>().is_err().then(|| Some(input.parse().unwrap())).unwrap_or(None),
            op: input.parse()?,
            right: input.parse()?,
            return_type: input.parse::<Token![->]>().and_then(|v| Ok((v,input.parse()?))).ok(),
        })
    }
}

impl OpExpr {
    pub fn is_binary(&self) -> bool {
        return self.left.is_some()
    }

    pub fn has_receiver_on_lhs(&self) -> bool {
        if let Some(OpArg::Self_(_)) = self.left {
           true 
        } else {
            false
        }
    }

    /// maps on return type if it is some otherwise returns none
    pub fn map_return_type<O,F : FnOnce(&Type) -> O>(&self, f : F) -> Option<O>{
        self.return_type.as_ref().map(|(_,v)| f(v))
    }

    /// Maps the return type and if none is present maps the given default instead
    pub fn map_return_type_with_default<O,F : FnOnce(&Type) -> O>(&self,def: Type,  f : F) -> O{
        f(self.return_type.as_ref().map(|(_,t)| t).unwrap_or(&def))
    }

    /// checks which side has type of Arg(Type) and maps over it or returns err
    pub fn map_type_side<O,F : FnOnce(&Type) -> O>(&self, f : F) -> Result<O>{
        if let Some(OpArg::Arg(v)) = &self.left {
            Ok(f(v))
        } else if let OpArg::Arg(v) = &self.right{
            Ok(f(v))
        } else {
            Err(Error::new(self.span(), "Invalid OpExpr, expected one side to have type"))
        }
    }

    /// maps both sides at the same time and combine into token stream preserving order, assumes this has 2 operators.
    /// The operator is converted to a rust operator call expression
    pub fn map_binary<T: FnOnce(&Type) -> TokenStream, S: FnOnce( &Receiver) -> TokenStream> (&self,t_f : T,s_f : S) -> Result<TokenStream>{
        let (l,r) = match (self.left.as_ref(),&self.right){
            (Some(OpArg::Self_(s)),OpArg::Arg(a)) => (s_f(s),t_f(a)),
            (Some(OpArg::Arg(s)),OpArg::Self_(a)) => (t_f(s),s_f(a)),
            _ => return Err(Error::new(self.span(), "Invalid OpExpr, expected binary expression"))
        };

        let operator = self.op.to_rust_method_ident();

        Ok(quote::quote!{
            #l.#operator(#r)
        })
    }

    pub fn map_unary<F: FnOnce(&Receiver) -> TokenStream>(&self, f : F) -> Result<TokenStream>{

        let l = match &self.right {
            OpArg::Self_(s) => f(s),
            _ => return Err(Error::new(self.span(), "Invalid OpExpr, expected unary expression"))
        };

        let operator = self.op.to_rust_method_ident();

        Ok(quote::quote!{
            #l.#operator()
        })
    }

}