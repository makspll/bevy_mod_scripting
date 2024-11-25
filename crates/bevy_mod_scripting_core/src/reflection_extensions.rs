use bevy::reflect::PartialReflect;

use crate::error::ScriptError;

/// Extension trait for [`PartialReflect`] providing additional functionality for working with specific types.
pub trait PartialReflectExt {
    /// Check if the represented type is from the given crate and has the given type identifier,
    /// returns false if not representing any type or if the type is not from the given crate or does not have the given type identifier.
    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool;

    /// Equivalent to [`PartialReflectExt::is_type`] but returns an appropriate error if the type is not the expected one.  
    fn expect_type(&self, crate_name: Option<&str>, type_ident: &str) -> Result<(), ScriptError>;

    /// If the type is an option, returns either the inner value or None if the option is None.
    /// Errors if the type is not an option.
    fn as_option(&self) -> Result<Option<&dyn PartialReflect>, ScriptError>;

    /// If the type is an iterable list-like type, returns an iterator over the elements.
    fn as_list(&self) -> Result<impl Iterator<Item = &dyn PartialReflect>, ScriptError>;
}

impl<T: PartialReflect + ?Sized> PartialReflectExt for T {
    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool {
        self.get_represented_type_info().is_some_and(|v| {
            let table = v.type_path_table();
            table.crate_name() == crate_name && table.ident() == Some(type_ident)
        })
    }

    fn expect_type(&self, crate_name: Option<&str>, type_ident: &str) -> Result<(), ScriptError> {
        if !self.is_type(crate_name, type_ident) {
            return Err(ScriptError::new_runtime_error(format!(
                "Expected type {type_ident}{}, but got {}",
                crate_name
                    .map(|s| format!(" from crate {s}"))
                    .unwrap_or_default(),
                self.get_represented_type_info()
                    .map(|ti| ti.type_path())
                    .unwrap_or_else(|| "dynamic type with no type information")
            )));
        }
        Ok(())
    }

    fn as_option(&self) -> Result<Option<&dyn PartialReflect>, ScriptError> {
        self.expect_type(Some("core"), "Option")?;

        if let bevy::reflect::ReflectRef::Enum(e) = self.reflect_ref() {
            if let Some(field) = e.field_at(0) {
                return Ok(Some(field));
            } else {
                return Ok(None);
            }
        }

        unreachable!("core::Option is an enum with a tuple variant")
    }

    fn as_list(&self) -> Result<impl Iterator<Item = &dyn PartialReflect>, ScriptError> {
        if let bevy::reflect::ReflectRef::List(l) = self.reflect_ref() {
            Ok(l.iter())
        } else {
            Err(ScriptError::new_runtime_error(format!(
                "Expected list-like type from crate core, but got {}",
                self.get_represented_type_info()
                    .map(|ti| ti.type_path())
                    .unwrap_or_else(|| "dynamic type with no type information")
            )))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_type_no_crate() {
        assert!(42.is_type(None, "i32"));
        assert!(42.expect_type(None, "i32").is_ok());
    }

    #[test]
    fn test_is_type_with_crate() {
        assert!(Some(42).is_type(Some("core"), "Option"));
        assert!(Some(42).expect_type(Some("core"), "Option").is_ok());
    }

    #[test]
    fn test_is_type_negative() {
        assert!(!Some(42).is_type(Some("std"), "Option"));
        assert_eq!(
            "Encountered Runtime Error error in a script: Expected type Option from crate std, but got core::option::Option<i32>",
            Some(42)
                .expect_type(Some("std"), "Option")
                .unwrap_err()
                .to_string()
        );
    }

    #[test]
    fn test_as_option_some() {
        assert_eq!(
            &42,
            Some(42)
                .as_option()
                .unwrap()
                .unwrap()
                .try_downcast_ref::<i32>()
                .unwrap()
        );
    }

    #[test]
    fn test_as_option_none() {
        assert!(None::<i32>.as_option().unwrap().is_none());
    }

    #[test]
    fn test_as_option_error() {
        assert_eq!(
            "Encountered Runtime Error error in a script: Expected type Option from crate core, but got i32",
            42.as_option().unwrap_err().to_string()
        );
    }
}
