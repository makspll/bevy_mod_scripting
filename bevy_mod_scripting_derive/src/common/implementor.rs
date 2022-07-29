

use proc_macro2::TokenStream;
use syn::{parse::Parse, spanned::Spanned,Result};
use quote::{quote_spanned, ToTokens};

use crate::{common::*};


/// A function on the wrapped type either wrapping an existing function or providing
/// additional functionality
pub(crate) trait WrapperFunction : Parse + ToTokens{

}


/// script-side API Implementation generator for a particular script language.
/// Helps avoid alot of boilerplate
pub(crate) trait WrapperImplementor : 'static {
    type Function : WrapperFunction;

    fn module_name() -> &'static str;

    /// Generates the type definition for the given newtype
    fn generate_newtype_definition(&mut self, new_type : &Newtype) -> Result<TokenStream>;

    /// Generates the necessary trait implementations for the given newtype exposing the given functionality
    fn generate_newtype_implementation<'a,I : Iterator<Item=&'a Self::Function>>(&mut self, new_type: &'a Newtype, functions : I) -> Result<TokenStream>;

    /// Generate methods from derive flags and newtype implementation blocks 
    fn generate_derive_flag_functions<'a, I : Iterator<Item=&'a DeriveFlag>>(&mut self, new_type : &'a Newtype, derive_flags : I) -> Result<Vec<Self::Function>>;
    
    /// Generate methods from newtype
    fn generate_newtype_functions(&mut self, new_type : &Newtype) -> Result<Vec<Self::Function>>;

    /// Turns newtype list into fully implemented wrapper types
    fn generate(&mut self, new_type : &Newtype) -> Result<TokenStream> {
        let definition = self.generate_newtype_definition(new_type)?;
    
        let mut functions = self.generate_derive_flag_functions(new_type,new_type.args.flags.iter())?;
        
        functions.extend(self.generate_newtype_functions(new_type)?);

        let implementation = self.generate_newtype_implementation(new_type, functions.iter())?;
        
        Ok(quote_spanned!{new_type.span()=>
            #definition
            #implementation
        })
    }

    // fn generate_all(&mut self, new_types: &NewtypeList) -> Result<TokenStream> {
    //     let mut methods_so_far = IndexMap::default();

    //     let newtypes : TokenStream = new_types.new_types.iter().map(|v| {
    //         self.generate(v, &mut methods_so_far)
    //     }).collect::<Result<_>>()?;

    //     let globals = self.generate_globals(new_types,methods_so_far)?;
    //     let module_name = format_ident!("{}",Self::module_name());
    //     let header = &new_types.module_headers;

    //     Ok(quote!{
    //         #[allow(unused_parens,unreachable_patterns,unused_variables)]
    //         pub mod #module_name {
    //             #header
    //             #newtypes
    //             #globals
    //         }
    //     })
    // }

}

