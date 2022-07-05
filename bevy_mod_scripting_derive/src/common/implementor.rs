
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use syn::{parse::Parse, punctuated::Punctuated, Token, spanned::Spanned,Result};
use quote::{quote, quote_spanned, ToTokens, format_ident};

use crate::{common::*, NewtypeList};


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
    fn generate_derive_flag_functions<'a, I : Iterator<Item=&'a DeriveFlag>>(&mut self, new_type : &'a Newtype, derive_flags : I, functions_so_far : & IndexMap<String, Vec<Self::Function>>) -> Result<Vec<Self::Function>>;
    
    /// Generate methods from newtype
    fn generate_newtype_functions(&mut self, new_type : &Newtype) -> Result<Vec<Self::Function>>;

    /// generate implementations that require all newtypes to be generated to construct
    fn generate_globals(&mut self, new_types: &NewtypeList, all_functions : IndexMap<String, Vec<Self::Function>>) -> Result<TokenStream>;

    /// Turns newtype list into fully implemented wrapper types
    fn generate(&mut self, new_type : &Newtype, all_functions : &mut IndexMap<String, Vec<Self::Function>>) -> Result<TokenStream> {
        let definition = self.generate_newtype_definition(new_type)?;
    
        let mut functions = self.generate_derive_flag_functions(new_type,new_type.args.flags.iter(),all_functions)?;
        
        // panic!("{:?}",functions.len());
        functions.extend(self.generate_newtype_functions(new_type)?);

        let implementation = self.generate_newtype_implementation(new_type, functions.iter())?;
        
        all_functions.insert(new_type.args.wrapper_type.to_string(),functions);

        Ok(quote_spanned!{new_type.span()=>
            #definition
            #implementation
        })
    }

    fn generate_all(&mut self, new_types: &NewtypeList) -> Result<TokenStream> {
        let mut methods_so_far = IndexMap::default();

        let newtypes : TokenStream = new_types.new_types.iter().map(|v| {
            self.generate(v, &mut methods_so_far)
        }).collect::<Result<_>>()?;

        let globals = self.generate_globals(new_types,methods_so_far)?;
        let module_name = format_ident!("{}",Self::module_name());
        let header = &new_types.module_headers;

        Ok(quote!{
            pub mod #module_name {
                #header
                #newtypes
                #globals
            }
        })
    }

}

