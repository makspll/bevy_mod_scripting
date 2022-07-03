#![allow(unused_variables,unused_parens)]
use bevy::{prelude::*};
use bevy::math::*;
use std::sync::Weak;
use std::ops::*;
use phf::{phf_map, Map};
use crate::LuaFile;
use crate::LuaRefBase;
use crate::ReflectPtr;
use crate::Script;
use crate::ScriptCollection;
use crate::LuaRef;
use crate::get_type_data;
use crate::{LuaComponent,LuaResource,LuaWorld};
use std::sync::Arc;
use parking_lot::RwLock;
use crate::util::impl_tealr_type;
use num_traits::cast::ToPrimitive;
use bevy_mod_scripting_derive::{impl_lua_newtypes,replace};
use tealr::{mlu::{mlua,mlua::{prelude::*,Error,MetaMethod,Value}},create_union_mlua};

 impl_lua_newtypes!([LuaComponent,LuaResource,LuaWorld][

    {
            usize : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<usize>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_usize().ok_or_else(||Error::RuntimeError("Value not compatibile with usize".to_owned()))?)));
            }
    },
    {
            isize : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<isize>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_isize().ok_or_else(||Error::RuntimeError("Value not compatibile with isize".to_owned()))?)));
            }
    },
    {
            i128 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i128>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i128().ok_or_else(||Error::RuntimeError("Value not compatibile with i128".to_owned()))?)));
            }
    },
    {
            i64 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i64>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i64().ok_or_else(||Error::RuntimeError("Value not compatibile with i64".to_owned()))?)));
            }
    },
    {
            i32 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i32>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i32().ok_or_else(||Error::RuntimeError("Value not compatibile with i32".to_owned()))?)));
            }
    },
    {
            i16 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i16>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i16().ok_or_else(||Error::RuntimeError("Value not compatibile with i16".to_owned()))?)));
            }
    },
    {
            i8 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<i8>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_i8().ok_or_else(||Error::RuntimeError("Value not compatibile with i8".to_owned()))?)));
            }
    },
    {
            u128 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u128>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u128().ok_or_else(||Error::RuntimeError("Value not compatibile with u128".to_owned()))?)));
            }
    },
    {
            u64 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u64>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u64().ok_or_else(||Error::RuntimeError("Value not compatibile with u64".to_owned()))?)));
            }
    },
    {
            u32 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u32>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u32().ok_or_else(||Error::RuntimeError("Value not compatibile with u32".to_owned()))?)));
            }
    },
    {
            u16 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u16>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u16().ok_or_else(||Error::RuntimeError("Value not compatibile with u16".to_owned()))?)));
            }
    },
    {
            u8 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Integer(s.downcast_ref::<u8>().unwrap().to_i64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_integer(v)?.ok_or_else(||Error::RuntimeError("Not an integer".to_owned()))?.to_u8().ok_or_else(||Error::RuntimeError("Value not compatibile with u8".to_owned()))?)));
            }
    },
    {
            f32 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Number(s.downcast_ref::<f32>().unwrap().to_f64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_number(v)?.ok_or_else(||Error::RuntimeError("Not a number".to_owned()))?.to_f32().ok_or_else(||Error::RuntimeError("Value not compatibile with f32".to_owned()))?)));
            }
    },
    {
            f64 : Primitive
            impl {
            "to" => |r,_| r.get(|s,_| Value::Number(s.downcast_ref::<f64>().unwrap().to_f64().unwrap()));
            "from" =>   |r,c,v : Value| r.get_mut(|s,_| Ok(s.apply(&c.coerce_number(v)?.ok_or_else(||Error::RuntimeError("Not a number".to_owned()))?.to_f64().ok_or_else(||Error::RuntimeError("Value not compatibile with f64".to_owned()))?)));
            }
    },
    {
            alloc::string::String : Primitive
            impl {
            "to" => |r,c| r.get(|s,_| Value::String(c.create_string(s.downcast_ref::<String>().unwrap()).unwrap()));
            "from" =>   |r,c,v : Value| c.coerce_string(v)?.ok_or_else(||Error::RuntimeError("Not a string".to_owned())).and_then(|string| r.get_mut(|s,_| Ok(s.apply(&string.to_str()?.to_owned()))));                             //      
            }
    },
    {
    
	///Lightweight identifier of an [entity](crate::entity).
	///
	///The identifier is implemented using a [generational index]: a combination of an ID and a generation.
	///This allows fast insertion after data removal in an array while minimizing loss of spatial locality.
	///
	///[generational index]: https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594
	///
	///# Usage
	///
	///This data type is returned by iterating a `Query` that has `Entity` as part of its query fetch type parameter ([learn more]).
	///It can also be obtained by calling [`EntityCommands::id`] or [`EntityMut::id`].
	///
	///```
	///# use bevy_ecs::prelude::*;
	///#
	///fn setup(mut commands: Commands) {
	///    // Calling `spawn` returns `EntityCommands`.
	///    let entity = commands.spawn().id();
	///}
	///
	///fn exclusive_system(world: &mut World) {
	///    // Calling `spawn` returns `EntityMut`.
	///    let entity = world.spawn().id();
	///}
	///#
	///# bevy_ecs::system::assert_is_system(setup);
	///# bevy_ecs::system::IntoExclusiveSystem::exclusive_system(exclusive_system);
	///```
	///
	///It can be used to refer to a specific entity to apply [`EntityCommands`], or to call [`Query::get`] (or similar methods) to access its components.
	///
	///```
	///# use bevy_ecs::prelude::*;
	///#
	///# #[derive(Component)]
	///# struct Expired;
	///#
	///fn dispose_expired_food(mut commands: Commands, query: Query<Entity, With<Expired>>) {
	///    for food_entity in query.iter() {
	///        commands.entity(food_entity).despawn();
	///    }
	///}
	///#
	///# bevy_ecs::system::assert_is_system(dispose_expired_food);
	///```
	///
	///[learn more]: crate::system::Query#entity-id-access
	///[`EntityCommands::id`]: crate::system::EntityCommands::id
	///[`EntityMut::id`]: crate::world::EntityMut::id
	///[`EntityCommands`]: crate::system::EntityCommands
	///[`Query::get`]: crate::system::Query::get
    bevy_ecs::entity::Entity : Reflect:
        UnaryOps(
			
			) 
		+ BinOps(
			
			) 
		+ AutoMethods(
			
			///Creates a new entity reference with the specified `id` and a generation of 0.
			///
			///# Note
			///
			///Spawning a specific `entity` value is __rarely the right choice__. Most apps should favor
			///[`Commands::spawn`](crate::system::Commands::spawn). This method should generally
			///only be used for sharing entities across apps, and only when they have a scheme
			///worked out to share an ID space (which doesn't happen by default).
			///
			///In general, one should not try to synchronize the ECS by attempting to ensure that
			///`Entity` lines up between instances, but instead insert a secondary identifier as
			///a component.
			///
			///There are still some use cases where it might be appropriate to use this function
			///externally.
			///
			///## Examples
			///
			///Initializing a collection (e.g. `array` or `Vec`) with a known size:
			///
			///```no_run
			///# use bevy_ecs::prelude::*;
			///// Create a new array of size 10 and initialize it with (invalid) entities.
			///let mut entities: [Entity; 10] = [Entity::from_raw(0); 10];
			///
			///// ... replace the entities with valid ones.
			///```
			///
			///Deriving `Reflect` for a component that has an `Entity` field:
			///
			///```no_run
			///# use bevy_ecs::{prelude::*, component::*};
			///# use bevy_reflect::Reflect;
			///#[derive(Reflect, Component)]
			///#[reflect(Component)]
			///pub struct MyStruct {
			///    pub entity: Entity,
			///}
			///
			///impl FromWorld for MyStruct {
			///    fn from_world(_world: &mut World) -> Self {
			///        Self {
			///            entity: Entity::from_raw(u32::MAX),
			///        }
			///    }
			///}
			///```
			from_raw(u32) -> LuaEntity ,
			
			///Convert to a form convenient for passing outside of rust.
			///
			///Only useful for identifying entities within the same instance of an application. Do not use
			///for serialization between runs.
			///
			///No particular structure is guaranteed for the returned bits.
			to_bits(self) -> u64 ,
			
			///Reconstruct an `Entity` previously destructured with [`Entity::to_bits`].
			///
			///Only useful when applied to results from `to_bits` in the same instance of an application.
			from_bits(u64) -> LuaEntity ,
			
			///Return a transiently unique identifier.
			///
			///No two simultaneously-live entities share the same ID, but dead entities' IDs may collide
			///with both live and dead entities. Useful for compactly representing entities within a
			///specific snapshot of the world, such as when serializing.
			id(self) -> u32 ,
			
			///Returns the generation of this Entity's id. The generation is incremented each time an
			///entity with a given id is despawned. This serves as a "count" of the number of times a
			///given id has been reused (id, generation) pairs uniquely identify a given Entity.
			generation(self) -> u32 
		) 
},
{
    
	///A 2-dimensional vector.
    glam::vec2::Vec2 : Reflect:
        UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaVec2 -> LuaVec2,
			self Add f32 -> LuaVec2,
			f32 Add self -> LuaVec2,
			self Sub LuaVec2 -> LuaVec2,
			self Sub f32 -> LuaVec2,
			f32 Sub self -> LuaVec2,
			self Div LuaVec2 -> LuaVec2,
			self Div f32 -> LuaVec2,
			f32 Div self -> LuaVec2,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "Mat2", id: Id("0:7055:1570"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			self Mul LuaVec2 -> LuaVec2,
			self Mul f32 -> LuaVec2,
			f32 Mul self -> LuaVec2,
			self Rem LuaVec2 -> LuaVec2,
			self Rem f32 -> LuaVec2,
			f32 Rem self -> LuaVec2
			) 
		+ AutoMethods(
			
			///Creates a new vector.
			new(f32,f32) -> LuaVec2 ,
			
			///Creates a 3D vector from `self` and the given `z` value.
			extend(self,f32) -> LuaVec3 ,
		
//	
//			///`[x, y]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[f32;2]` in type: `Some("Vec2")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(f32) -> LuaVec2 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaVec2 
//			Error: Unsupported argument `BVec2` in type: `Some("Vec2")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaVec2) -> f32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaVec2) -> LuaVec2 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaVec2) -> LuaVec2 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaVec2,LuaVec2) -> LuaVec2 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> f32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> f32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("Vec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("Vec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("Vec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("Vec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("Vec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("Vec2")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaVec2 
//			Error: Unsupported argument `&[f32]` in type: `Some("Vec2")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [f32]` in type: `Some("Vec2")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaVec2 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaVec2 ,
			
			///Returns a vector that is equal to `self` rotated by 90 degrees.
			perp(self) -> LuaVec2 ,
			
			///The perpendicular dot product of `self` and `other`.
			///Also known as the wedge product, 2d cross product, and determinant.
			perp_dot(self,LuaVec2) -> f32 ,
			
			///Returns `true` if, and only if, all elements are finite.  If any element is either
			///`NaN`, positive or negative infinity, this will return `false`.
			is_finite(self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(self) -> bool ,
		
//	
//			///Performs `is_nan` on each element of self, returning a vector mask of the results.
//			///
//			///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
//			is_nan_mask(self) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("Vec2")`.,
			
			///Computes the length of `self`.
			length(self) -> f32 ,
			
			///Computes the squared length of `self`.
			///
			///This is faster than `length()` as it avoids a square root operation.
			length_squared(self) -> f32 ,
			
			///Computes `1.0 / length()`.
			///
			///For valid results, `self` must _not_ be of length zero.
			length_recip(self) -> f32 ,
			
			///Computes the Euclidean distance between two points in space.
			distance(self,LuaVec2) -> f32 ,
			
			///Compute the squared euclidean distance between two points in space.
			distance_squared(self,LuaVec2) -> f32 ,
			
			///Returns `self` normalized to length 1.0.
			///
			///For valid results, `self` must _not_ be of length zero, nor very close to zero.
			///
			///See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
			///
			///Panics
			///
			///Will panic if `self` is zero length when `glam_assert` is enabled.
			normalize(self) -> LuaVec2 ,
		
//	
//			///Returns `self` normalized to length 1.0 if possible, else returns `None`.
//			///
//			///In particular, if the input is zero (or very close to zero), or non-finite,
//			///the result of this operation will be `None`.
//			///
//			///See also [`Self::normalize_or_zero`].
//			try_normalize(self) ->  
//			Error: Unsupported return type `Option` in type: `Some("Vec2")`.,
			
			///Returns `self` normalized to length 1.0 if possible, else returns zero.
			///
			///In particular, if the input is zero (or very close to zero), or non-finite,
			///the result of this operation will be zero.
			///
			///See also [`Self::try_normalize`].
			normalize_or_zero(self) -> LuaVec2 ,
			
			///Returns whether `self` is length `1.0` or not.
			///
			///Uses a precision threshold of `1e-6`.
			is_normalized(self) -> bool ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` is zero length when `glam_assert` is enabled.
			project_onto(self,LuaVec2) -> LuaVec2 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` has a length of zero when `glam_assert` is enabled.
			reject_from(self,LuaVec2) -> LuaVec2 ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			project_onto_normalized(self,LuaVec2) -> LuaVec2 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			reject_from_normalized(self,LuaVec2) -> LuaVec2 ,
			
			///Returns a vector containing the nearest integer to a number for each element of `self`.
			///Round half-way cases away from 0.0.
			round(self) -> LuaVec2 ,
			
			///Returns a vector containing the largest integer less than or equal to a number for each
			///element of `self`.
			floor(self) -> LuaVec2 ,
			
			///Returns a vector containing the smallest integer greater than or equal to a number for
			///each element of `self`.
			ceil(self) -> LuaVec2 ,
			
			///Returns a vector containing the fractional part of the vector, e.g. `self -
			///self.floor()`.
			///
			///Note that this is fast but not precise for large numbers.
			fract(self) -> LuaVec2 ,
			
			///Returns a vector containing `e^self` (the exponential function) for each element of
			///`self`.
			exp(self) -> LuaVec2 ,
			
			///Returns a vector containing each element of `self` raised to the power of `n`.
			powf(self,f32) -> LuaVec2 ,
			
			///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
			recip(self) -> LuaVec2 ,
			
			///Performs a linear interpolation between `self` and `other` based on the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
			///will be equal to `other`. When `s` is outside of range [0,1], the result is linearly
			///extrapolated.
			lerp(self,LuaVec2,f32) -> LuaVec2 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other` is
			///less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two vectors contain similar elements. It works best when
			///comparing with a known value. The `max_abs_diff` that should be used used depends on
			///the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(self,LuaVec2,f32) -> bool ,
			
			///Returns a vector with a length no less than `min` and no more than `max`
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp_length(self,f32,f32) -> LuaVec2 ,
			
			///Returns a vector with a length no more than `max`
			clamp_length_max(self,f32) -> LuaVec2 ,
			
			///Returns a vector with a length no less than `min`
			clamp_length_min(self,f32) -> LuaVec2 ,
			
			///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
			///error, yielding a more accurate result than an unfused multiply-add.
			///
			///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
			///architecture has a dedicated fma CPU instruction. However, this is not always true,
			///and will be heavily dependant on designing algorithms with specific target hardware in
			///mind.
			mul_add(self,LuaVec2,LuaVec2) -> LuaVec2 ,
			
			///Returns the angle (in radians) between `self` and `other`.
			///
			///The input vectors do not need to be unit length however they must be non-zero.
			angle_between(self,LuaVec2) -> f32 ,
			
			///Casts all elements of `self` to `f64`.
			as_dvec2(&self) -> LuaDVec2 ,
			
			///Casts all elements of `self` to `i32`.
			as_ivec2(&self) -> LuaIVec2 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec2(&self) -> LuaUVec2 
		) 
	impl {

			(MetaMethod::Index) (s=LuaVec2)=> {|_,s,idx: usize| {Ok(s.inner()[idx-1])}};
			mut (MetaMethod::NewIndex) (n=f32) => {|_,s,(idx,val): (usize,($n))| {Ok(s.val_mut(|s| s[idx-1] = val))}};}
},
{
    
	///A 3-dimensional vector without SIMD support.
    glam::vec3::Vec3 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaVec3),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=f32))
    
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaVec3 -> LuaVec3,
			self Add f32 -> LuaVec3,
			f32 Add self -> LuaVec3,
			self Sub LuaVec3 -> LuaVec3,
			self Sub f32 -> LuaVec3,
			f32 Sub self -> LuaVec3,
			self Div LuaVec3 -> LuaVec3,
			self Div f32 -> LuaVec3,
			f32 Div self -> LuaVec3,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "Mat3", id: Id("0:7273:1572"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "Mat3A", id: Id("0:7387:1573"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "Quat", id: Id("0:7889:1575"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			self Mul LuaVec3 -> LuaVec3,
			self Mul f32 -> LuaVec3,
			f32 Mul self -> LuaVec3,
			self Rem LuaVec3 -> LuaVec3,
			self Rem f32 -> LuaVec3,
			f32 Rem self -> LuaVec3
			) 
		+ AutoMethods(
			
			///Creates a new 3D vector.
			new(f32,f32,f32) -> LuaVec3 ,
			
			///Creates a 4D vector from `self` and the given `w` value.
			extend(self,f32) -> LuaVec4 ,
			
			///Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
			///
			///Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
			truncate(self) -> LuaVec2 ,
			
			///Computes the cross product of `self` and `other`.
			cross(self,LuaVec3) -> LuaVec3 ,
		
//	
//			///`[x, y, z]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[f32;3]` in type: `Some("Vec3")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(f32) -> LuaVec3 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaVec3 
//			Error: Unsupported argument `BVec3` in type: `Some("Vec3")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaVec3) -> f32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaVec3) -> LuaVec3 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaVec3) -> LuaVec3 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaVec3,LuaVec3) -> LuaVec3 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> f32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> f32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("Vec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("Vec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("Vec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("Vec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("Vec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("Vec3")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaVec3 
//			Error: Unsupported argument `&[f32]` in type: `Some("Vec3")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [f32]` in type: `Some("Vec3")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaVec3 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaVec3 ,
			
			///Returns `true` if, and only if, all elements are finite.  If any element is either
			///`NaN`, positive or negative infinity, this will return `false`.
			is_finite(self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(self) -> bool ,
		
//	
//			///Performs `is_nan` on each element of self, returning a vector mask of the results.
//			///
//			///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
//			is_nan_mask(self) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("Vec3")`.,
			
			///Computes the length of `self`.
			length(self) -> f32 ,
			
			///Computes the squared length of `self`.
			///
			///This is faster than `length()` as it avoids a square root operation.
			length_squared(self) -> f32 ,
			
			///Computes `1.0 / length()`.
			///
			///For valid results, `self` must _not_ be of length zero.
			length_recip(self) -> f32 ,
			
			///Computes the Euclidean distance between two points in space.
			distance(self,LuaVec3) -> f32 ,
			
			///Compute the squared euclidean distance between two points in space.
			distance_squared(self,LuaVec3) -> f32 ,
			
			///Returns `self` normalized to length 1.0.
			///
			///For valid results, `self` must _not_ be of length zero, nor very close to zero.
			///
			///See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
			///
			///Panics
			///
			///Will panic if `self` is zero length when `glam_assert` is enabled.
			normalize(self) -> LuaVec3 ,
		
//	
//			///Returns `self` normalized to length 1.0 if possible, else returns `None`.
//			///
//			///In particular, if the input is zero (or very close to zero), or non-finite,
//			///the result of this operation will be `None`.
//			///
//			///See also [`Self::normalize_or_zero`].
//			try_normalize(self) ->  
//			Error: Unsupported return type `Option` in type: `Some("Vec3")`.,
			
			///Returns `self` normalized to length 1.0 if possible, else returns zero.
			///
			///In particular, if the input is zero (or very close to zero), or non-finite,
			///the result of this operation will be zero.
			///
			///See also [`Self::try_normalize`].
			normalize_or_zero(self) -> LuaVec3 ,
			
			///Returns whether `self` is length `1.0` or not.
			///
			///Uses a precision threshold of `1e-6`.
			is_normalized(self) -> bool ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` is zero length when `glam_assert` is enabled.
			project_onto(self,LuaVec3) -> LuaVec3 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` has a length of zero when `glam_assert` is enabled.
			reject_from(self,LuaVec3) -> LuaVec3 ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			project_onto_normalized(self,LuaVec3) -> LuaVec3 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			reject_from_normalized(self,LuaVec3) -> LuaVec3 ,
			
			///Returns a vector containing the nearest integer to a number for each element of `self`.
			///Round half-way cases away from 0.0.
			round(self) -> LuaVec3 ,
			
			///Returns a vector containing the largest integer less than or equal to a number for each
			///element of `self`.
			floor(self) -> LuaVec3 ,
			
			///Returns a vector containing the smallest integer greater than or equal to a number for
			///each element of `self`.
			ceil(self) -> LuaVec3 ,
			
			///Returns a vector containing the fractional part of the vector, e.g. `self -
			///self.floor()`.
			///
			///Note that this is fast but not precise for large numbers.
			fract(self) -> LuaVec3 ,
			
			///Returns a vector containing `e^self` (the exponential function) for each element of
			///`self`.
			exp(self) -> LuaVec3 ,
			
			///Returns a vector containing each element of `self` raised to the power of `n`.
			powf(self,f32) -> LuaVec3 ,
			
			///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
			recip(self) -> LuaVec3 ,
			
			///Performs a linear interpolation between `self` and `other` based on the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
			///will be equal to `other`. When `s` is outside of range [0,1], the result is linearly
			///extrapolated.
			lerp(self,LuaVec3,f32) -> LuaVec3 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other` is
			///less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two vectors contain similar elements. It works best when
			///comparing with a known value. The `max_abs_diff` that should be used used depends on
			///the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(self,LuaVec3,f32) -> bool ,
			
			///Returns a vector with a length no less than `min` and no more than `max`
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp_length(self,f32,f32) -> LuaVec3 ,
			
			///Returns a vector with a length no more than `max`
			clamp_length_max(self,f32) -> LuaVec3 ,
			
			///Returns a vector with a length no less than `min`
			clamp_length_min(self,f32) -> LuaVec3 ,
			
			///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
			///error, yielding a more accurate result than an unfused multiply-add.
			///
			///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
			///architecture has a dedicated fma CPU instruction. However, this is not always true,
			///and will be heavily dependant on designing algorithms with specific target hardware in
			///mind.
			mul_add(self,LuaVec3,LuaVec3) -> LuaVec3 ,
			
			///Returns the angle (in radians) between two vectors.
			///
			///The input vectors do not need to be unit length however they must be non-zero.
			angle_between(self,LuaVec3) -> f32 ,
			
			///Returns some vector that is orthogonal to the given one.
			///
			///The input vector must be finite and non-zero.
			///
			///The output vector is not necessarily unit-length.
			///For that use [`Self::any_orthonormal_vector`] instead.
			any_orthogonal_vector(&self) -> LuaVec3 ,
			
			///Returns any unit-length vector that is orthogonal to the given one.
			///The input vector must be finite and non-zero.
			///
			///# Panics
			///
			///Will panic if `self` is not normalized when `glam_assert` is enabled.
			any_orthonormal_vector(&self) -> LuaVec3 ,
		
//	
//			///Given a unit-length vector return two other vectors that together form an orthonormal
//			///basis.  That is, all three vectors are orthogonal to each other and are normalized.
//			///
//			///# Panics
//			///
//			///Will panic if `self` is not normalized when `glam_assert` is enabled.
//			any_orthonormal_pair(&self) ->  
//			Error: Unsupported return type `(LuaVec3,LuaVec3)` in type: `Some("Vec3")`.,
			
			///Casts all elements of `self` to `f64`.
			as_dvec3(&self) -> LuaDVec3 ,
			
			///Casts all elements of `self` to `i32`.
			as_ivec3(&self) -> LuaIVec3 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec3(&self) -> LuaUVec3 
		) 
},
{
    
	///A 4-dimensional vector.
	///
	///This type uses 16 byte aligned SIMD vector type for storage on supported platforms.
    glam::vec4::Vec4 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaVec4),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=f32))
    
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaVec4 -> LuaVec4,
			self Add f32 -> LuaVec4,
			f32 Add self -> LuaVec4,
			self Sub LuaVec4 -> LuaVec4,
			self Sub f32 -> LuaVec4,
			f32 Sub self -> LuaVec4,
			self Div LuaVec4 -> LuaVec4,
			self Div f32 -> LuaVec4,
			f32 Div self -> LuaVec4,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "Mat4", id: Id("0:7623:1574"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			self Mul LuaVec4 -> LuaVec4,
			self Mul f32 -> LuaVec4,
			f32 Mul self -> LuaVec4,
			self Rem LuaVec4 -> LuaVec4,
			self Rem f32 -> LuaVec4,
			f32 Rem self -> LuaVec4
			) 
		+ AutoMethods(
			
			///Creates a new 4D vector.
			new(f32,f32,f32,f32) -> LuaVec4 ,
			
			///Creates a `Vec3` from the `x`, `y` and `z` elements of `self`, discarding `w`.
			///
			///Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
			///
			///To truncate to `Vec3A` use `Vec3A::from()`.
			truncate(self) -> LuaVec3 ,
		
//	
//			///`[x, y, z, w]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[f32;4]` in type: `Some("Vec4")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(f32) -> LuaVec4 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaVec4 
//			Error: Unsupported argument `BVec4A` in type: `Some("Vec4")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaVec4) -> f32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaVec4) -> LuaVec4 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaVec4) -> LuaVec4 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaVec4,LuaVec4) -> LuaVec4 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> f32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> f32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaVec4) ->  
//			Error: Unsupported return type `BVec4A` in type: `Some("Vec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaVec4) ->  
//			Error: Unsupported return type `BVec4A` in type: `Some("Vec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaVec4) ->  
//			Error: Unsupported return type `BVec4A` in type: `Some("Vec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaVec4) ->  
//			Error: Unsupported return type `BVec4A` in type: `Some("Vec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaVec4) ->  
//			Error: Unsupported return type `BVec4A` in type: `Some("Vec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaVec4) ->  
//			Error: Unsupported return type `BVec4A` in type: `Some("Vec4")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaVec4 
//			Error: Unsupported argument `&[f32]` in type: `Some("Vec4")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [f32]` in type: `Some("Vec4")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaVec4 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaVec4 ,
			
			///Returns `true` if, and only if, all elements are finite.  If any element is either
			///`NaN`, positive or negative infinity, this will return `false`.
			is_finite(self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(self) -> bool ,
		
//	
//			///Performs `is_nan` on each element of self, returning a vector mask of the results.
//			///
//			///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
//			is_nan_mask(self) ->  
//			Error: Unsupported return type `BVec4A` in type: `Some("Vec4")`.,
			
			///Computes the length of `self`.
			length(self) -> f32 ,
			
			///Computes the squared length of `self`.
			///
			///This is faster than `length()` as it avoids a square root operation.
			length_squared(self) -> f32 ,
			
			///Computes `1.0 / length()`.
			///
			///For valid results, `self` must _not_ be of length zero.
			length_recip(self) -> f32 ,
			
			///Computes the Euclidean distance between two points in space.
			distance(self,LuaVec4) -> f32 ,
			
			///Compute the squared euclidean distance between two points in space.
			distance_squared(self,LuaVec4) -> f32 ,
			
			///Returns `self` normalized to length 1.0.
			///
			///For valid results, `self` must _not_ be of length zero, nor very close to zero.
			///
			///See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
			///
			///Panics
			///
			///Will panic if `self` is zero length when `glam_assert` is enabled.
			normalize(self) -> LuaVec4 ,
		
//	
//			///Returns `self` normalized to length 1.0 if possible, else returns `None`.
//			///
//			///In particular, if the input is zero (or very close to zero), or non-finite,
//			///the result of this operation will be `None`.
//			///
//			///See also [`Self::normalize_or_zero`].
//			try_normalize(self) ->  
//			Error: Unsupported return type `Option` in type: `Some("Vec4")`.,
			
			///Returns `self` normalized to length 1.0 if possible, else returns zero.
			///
			///In particular, if the input is zero (or very close to zero), or non-finite,
			///the result of this operation will be zero.
			///
			///See also [`Self::try_normalize`].
			normalize_or_zero(self) -> LuaVec4 ,
			
			///Returns whether `self` is length `1.0` or not.
			///
			///Uses a precision threshold of `1e-6`.
			is_normalized(self) -> bool ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` is zero length when `glam_assert` is enabled.
			project_onto(self,LuaVec4) -> LuaVec4 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` has a length of zero when `glam_assert` is enabled.
			reject_from(self,LuaVec4) -> LuaVec4 ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			project_onto_normalized(self,LuaVec4) -> LuaVec4 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			reject_from_normalized(self,LuaVec4) -> LuaVec4 ,
			
			///Returns a vector containing the nearest integer to a number for each element of `self`.
			///Round half-way cases away from 0.0.
			round(self) -> LuaVec4 ,
			
			///Returns a vector containing the largest integer less than or equal to a number for each
			///element of `self`.
			floor(self) -> LuaVec4 ,
			
			///Returns a vector containing the smallest integer greater than or equal to a number for
			///each element of `self`.
			ceil(self) -> LuaVec4 ,
			
			///Returns a vector containing the fractional part of the vector, e.g. `self -
			///self.floor()`.
			///
			///Note that this is fast but not precise for large numbers.
			fract(self) -> LuaVec4 ,
			
			///Returns a vector containing `e^self` (the exponential function) for each element of
			///`self`.
			exp(self) -> LuaVec4 ,
			
			///Returns a vector containing each element of `self` raised to the power of `n`.
			powf(self,f32) -> LuaVec4 ,
			
			///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
			recip(self) -> LuaVec4 ,
			
			///Performs a linear interpolation between `self` and `other` based on the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
			///will be equal to `other`. When `s` is outside of range [0,1], the result is linearly
			///extrapolated.
			lerp(self,LuaVec4,f32) -> LuaVec4 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other` is
			///less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two vectors contain similar elements. It works best when
			///comparing with a known value. The `max_abs_diff` that should be used used depends on
			///the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(self,LuaVec4,f32) -> bool ,
			
			///Returns a vector with a length no less than `min` and no more than `max`
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp_length(self,f32,f32) -> LuaVec4 ,
			
			///Returns a vector with a length no more than `max`
			clamp_length_max(self,f32) -> LuaVec4 ,
			
			///Returns a vector with a length no less than `min`
			clamp_length_min(self,f32) -> LuaVec4 ,
			
			///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
			///error, yielding a more accurate result than an unfused multiply-add.
			///
			///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
			///architecture has a dedicated fma CPU instruction. However, this is not always true,
			///and will be heavily dependant on designing algorithms with specific target hardware in
			///mind.
			mul_add(self,LuaVec4,LuaVec4) -> LuaVec4 ,
			
			///Casts all elements of `self` to `f64`.
			as_dvec4(&self) -> LuaDVec4 ,
			
			///Casts all elements of `self` to `i32`.
			as_ivec4(&self) -> LuaIVec4 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec4(&self) -> LuaUVec4 
		) 
},
{
    
	///A 2-dimensional vector.
    glam::vec2::DVec2 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaDVec2),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64))
    
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaDVec2 -> LuaDVec2,
			self Add f64 -> LuaDVec2,
			f64 Add self -> LuaDVec2,
			self Sub LuaDVec2 -> LuaDVec2,
			self Sub f64 -> LuaDVec2,
			f64 Sub self -> LuaDVec2,
			self Div LuaDVec2 -> LuaDVec2,
			self Div f64 -> LuaDVec2,
			f64 Div self -> LuaDVec2,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "DMat2", id: Id("0:7158:1587"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			self Mul LuaDVec2 -> LuaDVec2,
			self Mul f64 -> LuaDVec2,
			f64 Mul self -> LuaDVec2,
			self Rem LuaDVec2 -> LuaDVec2,
			self Rem f64 -> LuaDVec2,
			f64 Rem self -> LuaDVec2
			) 
		+ AutoMethods(
			
			///Creates a new vector.
			new(f64,f64) -> LuaDVec2 ,
			
			///Creates a 3D vector from `self` and the given `z` value.
			extend(self,f64) -> LuaDVec3 ,
		
//	
//			///`[x, y]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[f64;2]` in type: `Some("DVec2")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(f64) -> LuaDVec2 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaDVec2 
//			Error: Unsupported argument `BVec2` in type: `Some("DVec2")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaDVec2) -> f64 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaDVec2) -> LuaDVec2 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaDVec2) -> LuaDVec2 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaDVec2,LuaDVec2) -> LuaDVec2 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> f64 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> f64 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaDVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("DVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaDVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("DVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaDVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("DVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaDVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("DVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaDVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("DVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaDVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("DVec2")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaDVec2 
//			Error: Unsupported argument `&[f64]` in type: `Some("DVec2")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [f64]` in type: `Some("DVec2")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaDVec2 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaDVec2 ,
			
			///Returns a vector that is equal to `self` rotated by 90 degrees.
			perp(self) -> LuaDVec2 ,
			
			///The perpendicular dot product of `self` and `other`.
			///Also known as the wedge product, 2d cross product, and determinant.
			perp_dot(self,LuaDVec2) -> f64 ,
			
			///Returns `true` if, and only if, all elements are finite.  If any element is either
			///`NaN`, positive or negative infinity, this will return `false`.
			is_finite(self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(self) -> bool ,
		
//	
//			///Performs `is_nan` on each element of self, returning a vector mask of the results.
//			///
//			///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
//			is_nan_mask(self) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("DVec2")`.,
			
			///Computes the length of `self`.
			length(self) -> f64 ,
			
			///Computes the squared length of `self`.
			///
			///This is faster than `length()` as it avoids a square root operation.
			length_squared(self) -> f64 ,
			
			///Computes `1.0 / length()`.
			///
			///For valid results, `self` must _not_ be of length zero.
			length_recip(self) -> f64 ,
			
			///Computes the Euclidean distance between two points in space.
			distance(self,LuaDVec2) -> f64 ,
			
			///Compute the squared euclidean distance between two points in space.
			distance_squared(self,LuaDVec2) -> f64 ,
			
			///Returns `self` normalized to length 1.0.
			///
			///For valid results, `self` must _not_ be of length zero, nor very close to zero.
			///
			///See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
			///
			///Panics
			///
			///Will panic if `self` is zero length when `glam_assert` is enabled.
			normalize(self) -> LuaDVec2 ,
		
//	
//			///Returns `self` normalized to length 1.0 if possible, else returns `None`.
//			///
//			///In particular, if the input is zero (or very close to zero), or non-finite,
//			///the result of this operation will be `None`.
//			///
//			///See also [`Self::normalize_or_zero`].
//			try_normalize(self) ->  
//			Error: Unsupported return type `Option` in type: `Some("DVec2")`.,
			
			///Returns `self` normalized to length 1.0 if possible, else returns zero.
			///
			///In particular, if the input is zero (or very close to zero), or non-finite,
			///the result of this operation will be zero.
			///
			///See also [`Self::try_normalize`].
			normalize_or_zero(self) -> LuaDVec2 ,
			
			///Returns whether `self` is length `1.0` or not.
			///
			///Uses a precision threshold of `1e-6`.
			is_normalized(self) -> bool ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` is zero length when `glam_assert` is enabled.
			project_onto(self,LuaDVec2) -> LuaDVec2 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` has a length of zero when `glam_assert` is enabled.
			reject_from(self,LuaDVec2) -> LuaDVec2 ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			project_onto_normalized(self,LuaDVec2) -> LuaDVec2 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			reject_from_normalized(self,LuaDVec2) -> LuaDVec2 ,
			
			///Returns a vector containing the nearest integer to a number for each element of `self`.
			///Round half-way cases away from 0.0.
			round(self) -> LuaDVec2 ,
			
			///Returns a vector containing the largest integer less than or equal to a number for each
			///element of `self`.
			floor(self) -> LuaDVec2 ,
			
			///Returns a vector containing the smallest integer greater than or equal to a number for
			///each element of `self`.
			ceil(self) -> LuaDVec2 ,
			
			///Returns a vector containing the fractional part of the vector, e.g. `self -
			///self.floor()`.
			///
			///Note that this is fast but not precise for large numbers.
			fract(self) -> LuaDVec2 ,
			
			///Returns a vector containing `e^self` (the exponential function) for each element of
			///`self`.
			exp(self) -> LuaDVec2 ,
			
			///Returns a vector containing each element of `self` raised to the power of `n`.
			powf(self,f64) -> LuaDVec2 ,
			
			///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
			recip(self) -> LuaDVec2 ,
			
			///Performs a linear interpolation between `self` and `other` based on the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
			///will be equal to `other`. When `s` is outside of range [0,1], the result is linearly
			///extrapolated.
			lerp(self,LuaDVec2,f64) -> LuaDVec2 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other` is
			///less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two vectors contain similar elements. It works best when
			///comparing with a known value. The `max_abs_diff` that should be used used depends on
			///the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(self,LuaDVec2,f64) -> bool ,
			
			///Returns a vector with a length no less than `min` and no more than `max`
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp_length(self,f64,f64) -> LuaDVec2 ,
			
			///Returns a vector with a length no more than `max`
			clamp_length_max(self,f64) -> LuaDVec2 ,
			
			///Returns a vector with a length no less than `min`
			clamp_length_min(self,f64) -> LuaDVec2 ,
			
			///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
			///error, yielding a more accurate result than an unfused multiply-add.
			///
			///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
			///architecture has a dedicated fma CPU instruction. However, this is not always true,
			///and will be heavily dependant on designing algorithms with specific target hardware in
			///mind.
			mul_add(self,LuaDVec2,LuaDVec2) -> LuaDVec2 ,
			
			///Returns the angle (in radians) between `self` and `other`.
			///
			///The input vectors do not need to be unit length however they must be non-zero.
			angle_between(self,LuaDVec2) -> f64 ,
			
			///Casts all elements of `self` to `f32`.
			as_vec2(&self) -> LuaVec2 ,
			
			///Casts all elements of `self` to `i32`.
			as_ivec2(&self) -> LuaIVec2 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec2(&self) -> LuaUVec2 
		) 
},
{
    
	///A 3-dimensional vector.
    glam::vec3::DVec3 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaDVec3),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64))
    
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaDVec3 -> LuaDVec3,
			self Add f64 -> LuaDVec3,
			f64 Add self -> LuaDVec3,
			self Sub LuaDVec3 -> LuaDVec3,
			self Sub f64 -> LuaDVec3,
			f64 Sub self -> LuaDVec3,
			self Div LuaDVec3 -> LuaDVec3,
			self Div f64 -> LuaDVec3,
			f64 Div self -> LuaDVec3,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "DMat3", id: Id("0:7491:1589"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "DQuat", id: Id("0:8014:1593"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			self Mul LuaDVec3 -> LuaDVec3,
			self Mul f64 -> LuaDVec3,
			f64 Mul self -> LuaDVec3,
			self Rem LuaDVec3 -> LuaDVec3,
			self Rem f64 -> LuaDVec3,
			f64 Rem self -> LuaDVec3
			) 
		+ AutoMethods(
			
			///Creates a new 3D vector.
			new(f64,f64,f64) -> LuaDVec3 ,
			
			///Creates a 4D vector from `self` and the given `w` value.
			extend(self,f64) -> LuaDVec4 ,
			
			///Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
			///
			///Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
			truncate(self) -> LuaDVec2 ,
			
			///Computes the cross product of `self` and `other`.
			cross(self,LuaDVec3) -> LuaDVec3 ,
		
//	
//			///`[x, y, z]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[f64;3]` in type: `Some("DVec3")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(f64) -> LuaDVec3 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaDVec3 
//			Error: Unsupported argument `BVec3` in type: `Some("DVec3")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaDVec3) -> f64 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaDVec3) -> LuaDVec3 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaDVec3) -> LuaDVec3 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaDVec3,LuaDVec3) -> LuaDVec3 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> f64 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> f64 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaDVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("DVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaDVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("DVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaDVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("DVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaDVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("DVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaDVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("DVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaDVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("DVec3")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaDVec3 
//			Error: Unsupported argument `&[f64]` in type: `Some("DVec3")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [f64]` in type: `Some("DVec3")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaDVec3 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaDVec3 ,
			
			///Returns `true` if, and only if, all elements are finite.  If any element is either
			///`NaN`, positive or negative infinity, this will return `false`.
			is_finite(self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(self) -> bool ,
		
//	
//			///Performs `is_nan` on each element of self, returning a vector mask of the results.
//			///
//			///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
//			is_nan_mask(self) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("DVec3")`.,
			
			///Computes the length of `self`.
			length(self) -> f64 ,
			
			///Computes the squared length of `self`.
			///
			///This is faster than `length()` as it avoids a square root operation.
			length_squared(self) -> f64 ,
			
			///Computes `1.0 / length()`.
			///
			///For valid results, `self` must _not_ be of length zero.
			length_recip(self) -> f64 ,
			
			///Computes the Euclidean distance between two points in space.
			distance(self,LuaDVec3) -> f64 ,
			
			///Compute the squared euclidean distance between two points in space.
			distance_squared(self,LuaDVec3) -> f64 ,
			
			///Returns `self` normalized to length 1.0.
			///
			///For valid results, `self` must _not_ be of length zero, nor very close to zero.
			///
			///See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
			///
			///Panics
			///
			///Will panic if `self` is zero length when `glam_assert` is enabled.
			normalize(self) -> LuaDVec3 ,
		
//	
//			///Returns `self` normalized to length 1.0 if possible, else returns `None`.
//			///
//			///In particular, if the input is zero (or very close to zero), or non-finite,
//			///the result of this operation will be `None`.
//			///
//			///See also [`Self::normalize_or_zero`].
//			try_normalize(self) ->  
//			Error: Unsupported return type `Option` in type: `Some("DVec3")`.,
			
			///Returns `self` normalized to length 1.0 if possible, else returns zero.
			///
			///In particular, if the input is zero (or very close to zero), or non-finite,
			///the result of this operation will be zero.
			///
			///See also [`Self::try_normalize`].
			normalize_or_zero(self) -> LuaDVec3 ,
			
			///Returns whether `self` is length `1.0` or not.
			///
			///Uses a precision threshold of `1e-6`.
			is_normalized(self) -> bool ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` is zero length when `glam_assert` is enabled.
			project_onto(self,LuaDVec3) -> LuaDVec3 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` has a length of zero when `glam_assert` is enabled.
			reject_from(self,LuaDVec3) -> LuaDVec3 ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			project_onto_normalized(self,LuaDVec3) -> LuaDVec3 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			reject_from_normalized(self,LuaDVec3) -> LuaDVec3 ,
			
			///Returns a vector containing the nearest integer to a number for each element of `self`.
			///Round half-way cases away from 0.0.
			round(self) -> LuaDVec3 ,
			
			///Returns a vector containing the largest integer less than or equal to a number for each
			///element of `self`.
			floor(self) -> LuaDVec3 ,
			
			///Returns a vector containing the smallest integer greater than or equal to a number for
			///each element of `self`.
			ceil(self) -> LuaDVec3 ,
			
			///Returns a vector containing the fractional part of the vector, e.g. `self -
			///self.floor()`.
			///
			///Note that this is fast but not precise for large numbers.
			fract(self) -> LuaDVec3 ,
			
			///Returns a vector containing `e^self` (the exponential function) for each element of
			///`self`.
			exp(self) -> LuaDVec3 ,
			
			///Returns a vector containing each element of `self` raised to the power of `n`.
			powf(self,f64) -> LuaDVec3 ,
			
			///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
			recip(self) -> LuaDVec3 ,
			
			///Performs a linear interpolation between `self` and `other` based on the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
			///will be equal to `other`. When `s` is outside of range [0,1], the result is linearly
			///extrapolated.
			lerp(self,LuaDVec3,f64) -> LuaDVec3 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other` is
			///less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two vectors contain similar elements. It works best when
			///comparing with a known value. The `max_abs_diff` that should be used used depends on
			///the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(self,LuaDVec3,f64) -> bool ,
			
			///Returns a vector with a length no less than `min` and no more than `max`
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp_length(self,f64,f64) -> LuaDVec3 ,
			
			///Returns a vector with a length no more than `max`
			clamp_length_max(self,f64) -> LuaDVec3 ,
			
			///Returns a vector with a length no less than `min`
			clamp_length_min(self,f64) -> LuaDVec3 ,
			
			///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
			///error, yielding a more accurate result than an unfused multiply-add.
			///
			///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
			///architecture has a dedicated fma CPU instruction. However, this is not always true,
			///and will be heavily dependant on designing algorithms with specific target hardware in
			///mind.
			mul_add(self,LuaDVec3,LuaDVec3) -> LuaDVec3 ,
			
			///Returns the angle (in radians) between two vectors.
			///
			///The input vectors do not need to be unit length however they must be non-zero.
			angle_between(self,LuaDVec3) -> f64 ,
			
			///Returns some vector that is orthogonal to the given one.
			///
			///The input vector must be finite and non-zero.
			///
			///The output vector is not necessarily unit-length.
			///For that use [`Self::any_orthonormal_vector`] instead.
			any_orthogonal_vector(&self) -> LuaDVec3 ,
			
			///Returns any unit-length vector that is orthogonal to the given one.
			///The input vector must be finite and non-zero.
			///
			///# Panics
			///
			///Will panic if `self` is not normalized when `glam_assert` is enabled.
			any_orthonormal_vector(&self) -> LuaDVec3 ,
		
//	
//			///Given a unit-length vector return two other vectors that together form an orthonormal
//			///basis.  That is, all three vectors are orthogonal to each other and are normalized.
//			///
//			///# Panics
//			///
//			///Will panic if `self` is not normalized when `glam_assert` is enabled.
//			any_orthonormal_pair(&self) ->  
//			Error: Unsupported return type `(LuaDVec3,LuaDVec3)` in type: `Some("DVec3")`.,
			
			///Casts all elements of `self` to `f32`.
			as_vec3(&self) -> LuaVec3 ,
		
//	
//			///Casts all elements of `self` to `f32`.
//			as_vec3a(&self) ->  
//			Error: Unsupported return type `Vec3A` in type: `Some("DVec3")`.,
			
			///Casts all elements of `self` to `i32`.
			as_ivec3(&self) -> LuaIVec3 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec3(&self) -> LuaUVec3 
		) 
},
{
    
	///A 4-dimensional vector.
    glam::vec4::DVec4 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaDVec4),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=f64))
    
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaDVec4 -> LuaDVec4,
			self Add f64 -> LuaDVec4,
			f64 Add self -> LuaDVec4,
			self Sub LuaDVec4 -> LuaDVec4,
			self Sub f64 -> LuaDVec4,
			f64 Sub self -> LuaDVec4,
			self Div LuaDVec4 -> LuaDVec4,
			self Div f64 -> LuaDVec4,
			f64 Div self -> LuaDVec4,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "DMat4", id: Id("0:7750:1591"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			self Mul LuaDVec4 -> LuaDVec4,
			self Mul f64 -> LuaDVec4,
			f64 Mul self -> LuaDVec4,
			self Rem LuaDVec4 -> LuaDVec4,
			self Rem f64 -> LuaDVec4,
			f64 Rem self -> LuaDVec4
			) 
		+ AutoMethods(
			
			///Creates a new 4D vector.
			new(f64,f64,f64,f64) -> LuaDVec4 ,
			
			///Creates a `Vec3` from the `x`, `y` and `z` elements of `self`, discarding `w`.
			///
			///Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
			///
			///To truncate to `Vec3A` use `Vec3A::from()`.
			truncate(self) -> LuaDVec3 ,
		
//	
//			///`[x, y, z, w]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[f64;4]` in type: `Some("DVec4")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(f64) -> LuaDVec4 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaDVec4 
//			Error: Unsupported argument `BVec4` in type: `Some("DVec4")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaDVec4) -> f64 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaDVec4) -> LuaDVec4 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaDVec4) -> LuaDVec4 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaDVec4,LuaDVec4) -> LuaDVec4 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> f64 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> f64 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaDVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("DVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaDVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("DVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaDVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("DVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaDVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("DVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaDVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("DVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaDVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("DVec4")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaDVec4 
//			Error: Unsupported argument `&[f64]` in type: `Some("DVec4")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [f64]` in type: `Some("DVec4")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaDVec4 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaDVec4 ,
			
			///Returns `true` if, and only if, all elements are finite.  If any element is either
			///`NaN`, positive or negative infinity, this will return `false`.
			is_finite(self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(self) -> bool ,
		
//	
//			///Performs `is_nan` on each element of self, returning a vector mask of the results.
//			///
//			///In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
//			is_nan_mask(self) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("DVec4")`.,
			
			///Computes the length of `self`.
			length(self) -> f64 ,
			
			///Computes the squared length of `self`.
			///
			///This is faster than `length()` as it avoids a square root operation.
			length_squared(self) -> f64 ,
			
			///Computes `1.0 / length()`.
			///
			///For valid results, `self` must _not_ be of length zero.
			length_recip(self) -> f64 ,
			
			///Computes the Euclidean distance between two points in space.
			distance(self,LuaDVec4) -> f64 ,
			
			///Compute the squared euclidean distance between two points in space.
			distance_squared(self,LuaDVec4) -> f64 ,
			
			///Returns `self` normalized to length 1.0.
			///
			///For valid results, `self` must _not_ be of length zero, nor very close to zero.
			///
			///See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
			///
			///Panics
			///
			///Will panic if `self` is zero length when `glam_assert` is enabled.
			normalize(self) -> LuaDVec4 ,
		
//	
//			///Returns `self` normalized to length 1.0 if possible, else returns `None`.
//			///
//			///In particular, if the input is zero (or very close to zero), or non-finite,
//			///the result of this operation will be `None`.
//			///
//			///See also [`Self::normalize_or_zero`].
//			try_normalize(self) ->  
//			Error: Unsupported return type `Option` in type: `Some("DVec4")`.,
			
			///Returns `self` normalized to length 1.0 if possible, else returns zero.
			///
			///In particular, if the input is zero (or very close to zero), or non-finite,
			///the result of this operation will be zero.
			///
			///See also [`Self::try_normalize`].
			normalize_or_zero(self) -> LuaDVec4 ,
			
			///Returns whether `self` is length `1.0` or not.
			///
			///Uses a precision threshold of `1e-6`.
			is_normalized(self) -> bool ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` is zero length when `glam_assert` is enabled.
			project_onto(self,LuaDVec4) -> LuaDVec4 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be of non-zero length.
			///
			///# Panics
			///
			///Will panic if `other` has a length of zero when `glam_assert` is enabled.
			reject_from(self,LuaDVec4) -> LuaDVec4 ,
			
			///Returns the vector projection of `self` onto `other`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			project_onto_normalized(self,LuaDVec4) -> LuaDVec4 ,
			
			///Returns the vector rejection of `self` from `other`.
			///
			///The vector rejection is the vector perpendicular to the projection of `self` onto
			///`other`, in other words the result of `self - self.project_onto(other)`.
			///
			///`other` must be normalized.
			///
			///# Panics
			///
			///Will panic if `other` is not normalized when `glam_assert` is enabled.
			reject_from_normalized(self,LuaDVec4) -> LuaDVec4 ,
			
			///Returns a vector containing the nearest integer to a number for each element of `self`.
			///Round half-way cases away from 0.0.
			round(self) -> LuaDVec4 ,
			
			///Returns a vector containing the largest integer less than or equal to a number for each
			///element of `self`.
			floor(self) -> LuaDVec4 ,
			
			///Returns a vector containing the smallest integer greater than or equal to a number for
			///each element of `self`.
			ceil(self) -> LuaDVec4 ,
			
			///Returns a vector containing the fractional part of the vector, e.g. `self -
			///self.floor()`.
			///
			///Note that this is fast but not precise for large numbers.
			fract(self) -> LuaDVec4 ,
			
			///Returns a vector containing `e^self` (the exponential function) for each element of
			///`self`.
			exp(self) -> LuaDVec4 ,
			
			///Returns a vector containing each element of `self` raised to the power of `n`.
			powf(self,f64) -> LuaDVec4 ,
			
			///Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
			recip(self) -> LuaDVec4 ,
			
			///Performs a linear interpolation between `self` and `other` based on the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
			///will be equal to `other`. When `s` is outside of range [0,1], the result is linearly
			///extrapolated.
			lerp(self,LuaDVec4,f64) -> LuaDVec4 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other` is
			///less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two vectors contain similar elements. It works best when
			///comparing with a known value. The `max_abs_diff` that should be used used depends on
			///the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(self,LuaDVec4,f64) -> bool ,
			
			///Returns a vector with a length no less than `min` and no more than `max`
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp_length(self,f64,f64) -> LuaDVec4 ,
			
			///Returns a vector with a length no more than `max`
			clamp_length_max(self,f64) -> LuaDVec4 ,
			
			///Returns a vector with a length no less than `min`
			clamp_length_min(self,f64) -> LuaDVec4 ,
			
			///Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
			///error, yielding a more accurate result than an unfused multiply-add.
			///
			///Using `mul_add` *may* be more performant than an unfused multiply-add if the target
			///architecture has a dedicated fma CPU instruction. However, this is not always true,
			///and will be heavily dependant on designing algorithms with specific target hardware in
			///mind.
			mul_add(self,LuaDVec4,LuaDVec4) -> LuaDVec4 ,
			
			///Casts all elements of `self` to `f32`.
			as_vec4(&self) -> LuaVec4 ,
			
			///Casts all elements of `self` to `i32`.
			as_ivec4(&self) -> LuaIVec4 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec4(&self) -> LuaUVec4 
		) 
},
{
    
	///A 2-dimensional vector.
    glam::vec2::IVec2 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaIVec2),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32))
    
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaIVec2 -> LuaIVec2,
			self Add i32 -> LuaIVec2,
			i32 Add self -> LuaIVec2,
			self Sub LuaIVec2 -> LuaIVec2,
			self Sub i32 -> LuaIVec2,
			i32 Sub self -> LuaIVec2,
			self Div LuaIVec2 -> LuaIVec2,
			self Div i32 -> LuaIVec2,
			i32 Div self -> LuaIVec2,
			self Mul LuaIVec2 -> LuaIVec2,
			self Mul i32 -> LuaIVec2,
			i32 Mul self -> LuaIVec2,
			self Rem LuaIVec2 -> LuaIVec2,
			self Rem i32 -> LuaIVec2,
			i32 Rem self -> LuaIVec2
			) 
		+ AutoMethods(
			
			///Creates a new vector.
			new(i32,i32) -> LuaIVec2 ,
			
			///Creates a 3D vector from `self` and the given `z` value.
			extend(self,i32) -> LuaIVec3 ,
		
//	
//			///`[x, y]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[i32;2]` in type: `Some("IVec2")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(i32) -> LuaIVec2 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaIVec2 
//			Error: Unsupported argument `BVec2` in type: `Some("IVec2")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaIVec2) -> i32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaIVec2) -> LuaIVec2 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaIVec2) -> LuaIVec2 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaIVec2,LuaIVec2) -> LuaIVec2 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> i32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> i32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaIVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("IVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaIVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("IVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaIVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("IVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaIVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("IVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaIVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("IVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaIVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("IVec2")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaIVec2 
//			Error: Unsupported argument `&[i32]` in type: `Some("IVec2")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [i32]` in type: `Some("IVec2")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaIVec2 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaIVec2 ,
			
			///Returns a vector that is equal to `self` rotated by 90 degrees.
			perp(self) -> LuaIVec2 ,
			
			///The perpendicular dot product of `self` and `other`.
			///Also known as the wedge product, 2d cross product, and determinant.
			perp_dot(self,LuaIVec2) -> i32 ,
			
			///Casts all elements of `self` to `f32`.
			as_vec2(&self) -> LuaVec2 ,
			
			///Casts all elements of `self` to `f64`.
			as_dvec2(&self) -> LuaDVec2 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec2(&self) -> LuaUVec2 
		) 
},
{
    
	///A 3-dimensional vector.
    glam::vec3::IVec3 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaIVec3),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32))
    
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaIVec3 -> LuaIVec3,
			self Add i32 -> LuaIVec3,
			i32 Add self -> LuaIVec3,
			self Sub LuaIVec3 -> LuaIVec3,
			self Sub i32 -> LuaIVec3,
			i32 Sub self -> LuaIVec3,
			self Div LuaIVec3 -> LuaIVec3,
			self Div i32 -> LuaIVec3,
			i32 Div self -> LuaIVec3,
			self Mul LuaIVec3 -> LuaIVec3,
			self Mul i32 -> LuaIVec3,
			i32 Mul self -> LuaIVec3,
			self Rem LuaIVec3 -> LuaIVec3,
			self Rem i32 -> LuaIVec3,
			i32 Rem self -> LuaIVec3
			) 
		+ AutoMethods(
			
			///Creates a new 3D vector.
			new(i32,i32,i32) -> LuaIVec3 ,
			
			///Creates a 4D vector from `self` and the given `w` value.
			extend(self,i32) -> LuaIVec4 ,
			
			///Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
			///
			///Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
			truncate(self) -> LuaIVec2 ,
			
			///Computes the cross product of `self` and `other`.
			cross(self,LuaIVec3) -> LuaIVec3 ,
		
//	
//			///`[x, y, z]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[i32;3]` in type: `Some("IVec3")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(i32) -> LuaIVec3 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaIVec3 
//			Error: Unsupported argument `BVec3` in type: `Some("IVec3")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaIVec3) -> i32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaIVec3) -> LuaIVec3 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaIVec3) -> LuaIVec3 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaIVec3,LuaIVec3) -> LuaIVec3 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> i32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> i32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaIVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("IVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaIVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("IVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaIVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("IVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaIVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("IVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaIVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("IVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaIVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("IVec3")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaIVec3 
//			Error: Unsupported argument `&[i32]` in type: `Some("IVec3")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [i32]` in type: `Some("IVec3")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaIVec3 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaIVec3 ,
			
			///Casts all elements of `self` to `f32`.
			as_vec3(&self) -> LuaVec3 ,
		
//	
//			///Casts all elements of `self` to `f32`.
//			as_vec3a(&self) ->  
//			Error: Unsupported return type `Vec3A` in type: `Some("IVec3")`.,
			
			///Casts all elements of `self` to `f64`.
			as_dvec3(&self) -> LuaDVec3 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec3(&self) -> LuaUVec3 
		) 
},
{
    
	///A 4-dimensional vector.
    glam::vec4::IVec4 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaIVec4),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=i32))
    
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaIVec4 -> LuaIVec4,
			self Add i32 -> LuaIVec4,
			i32 Add self -> LuaIVec4,
			self Sub LuaIVec4 -> LuaIVec4,
			self Sub i32 -> LuaIVec4,
			i32 Sub self -> LuaIVec4,
			self Div LuaIVec4 -> LuaIVec4,
			self Div i32 -> LuaIVec4,
			i32 Div self -> LuaIVec4,
			self Mul LuaIVec4 -> LuaIVec4,
			self Mul i32 -> LuaIVec4,
			i32 Mul self -> LuaIVec4,
			self Rem LuaIVec4 -> LuaIVec4,
			self Rem i32 -> LuaIVec4,
			i32 Rem self -> LuaIVec4
			) 
		+ AutoMethods(
			
			///Creates a new 4D vector.
			new(i32,i32,i32,i32) -> LuaIVec4 ,
			
			///Creates a `Vec3` from the `x`, `y` and `z` elements of `self`, discarding `w`.
			///
			///Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
			///
			///To truncate to `Vec3A` use `Vec3A::from()`.
			truncate(self) -> LuaIVec3 ,
		
//	
//			///`[x, y, z, w]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[i32;4]` in type: `Some("IVec4")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(i32) -> LuaIVec4 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaIVec4 
//			Error: Unsupported argument `BVec4` in type: `Some("IVec4")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaIVec4) -> i32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaIVec4) -> LuaIVec4 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaIVec4) -> LuaIVec4 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaIVec4,LuaIVec4) -> LuaIVec4 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> i32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> i32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaIVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("IVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaIVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("IVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaIVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("IVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaIVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("IVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaIVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("IVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaIVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("IVec4")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaIVec4 
//			Error: Unsupported argument `&[i32]` in type: `Some("IVec4")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [i32]` in type: `Some("IVec4")`.,
			
			///Returns a vector containing the absolute value of each element of `self`.
			abs(self) -> LuaIVec4 ,
			
			///Returns a vector with elements representing the sign of `self`.
			///
			///- `1.0` if the number is positive, `+0.0` or `INFINITY`
			///- `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
			///- `NAN` if the number is `NAN`
			signum(self) -> LuaIVec4 ,
			
			///Casts all elements of `self` to `f32`.
			as_vec4(&self) -> LuaVec4 ,
			
			///Casts all elements of `self` to `f64`.
			as_dvec4(&self) -> LuaDVec4 ,
			
			///Casts all elements of `self` to `u32`.
			as_uvec4(&self) -> LuaUVec4 
		) 
},
{
    
	///A 2-dimensional vector.
    glam::vec2::UVec2 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaUVec2),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32))
    
		+ UnaryOps(
			
			) 
		+ BinOps(
			self Add LuaUVec2 -> LuaUVec2,
			self Add u32 -> LuaUVec2,
			u32 Add self -> LuaUVec2,
			self Sub LuaUVec2 -> LuaUVec2,
			self Sub u32 -> LuaUVec2,
			u32 Sub self -> LuaUVec2,
			self Div LuaUVec2 -> LuaUVec2,
			self Div u32 -> LuaUVec2,
			u32 Div self -> LuaUVec2,
			self Mul LuaUVec2 -> LuaUVec2,
			self Mul u32 -> LuaUVec2,
			u32 Mul self -> LuaUVec2,
			self Rem LuaUVec2 -> LuaUVec2,
			self Rem u32 -> LuaUVec2,
			u32 Rem self -> LuaUVec2
			) 
		+ AutoMethods(
			
			///Creates a new vector.
			new(u32,u32) -> LuaUVec2 ,
			
			///Creates a 3D vector from `self` and the given `z` value.
			extend(self,u32) -> LuaUVec3 ,
		
//	
//			///`[x, y]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[u32;2]` in type: `Some("UVec2")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(u32) -> LuaUVec2 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaUVec2 
//			Error: Unsupported argument `BVec2` in type: `Some("UVec2")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaUVec2) -> u32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaUVec2) -> LuaUVec2 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaUVec2) -> LuaUVec2 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaUVec2,LuaUVec2) -> LuaUVec2 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> u32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> u32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaUVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("UVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaUVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("UVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaUVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("UVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaUVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("UVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaUVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("UVec2")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaUVec2) ->  
//			Error: Unsupported return type `BVec2` in type: `Some("UVec2")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaUVec2 
//			Error: Unsupported argument `&[u32]` in type: `Some("UVec2")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [u32]` in type: `Some("UVec2")`.,
			
			///Casts all elements of `self` to `f32`.
			as_vec2(&self) -> LuaVec2 ,
			
			///Casts all elements of `self` to `f64`.
			as_dvec2(&self) -> LuaDVec2 ,
			
			///Casts all elements of `self` to `i32`.
			as_ivec2(&self) -> LuaIVec2 
		) 
},
{
    
	///A 3-dimensional vector.
    glam::vec3::UVec3 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaUVec3),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32))
    
		+ UnaryOps(
			
			) 
		+ BinOps(
			self Add LuaUVec3 -> LuaUVec3,
			self Add u32 -> LuaUVec3,
			u32 Add self -> LuaUVec3,
			self Sub LuaUVec3 -> LuaUVec3,
			self Sub u32 -> LuaUVec3,
			u32 Sub self -> LuaUVec3,
			self Div LuaUVec3 -> LuaUVec3,
			self Div u32 -> LuaUVec3,
			u32 Div self -> LuaUVec3,
			self Mul LuaUVec3 -> LuaUVec3,
			self Mul u32 -> LuaUVec3,
			u32 Mul self -> LuaUVec3,
			self Rem LuaUVec3 -> LuaUVec3,
			self Rem u32 -> LuaUVec3,
			u32 Rem self -> LuaUVec3
			) 
		+ AutoMethods(
			
			///Creates a new 3D vector.
			new(u32,u32,u32) -> LuaUVec3 ,
			
			///Creates a 4D vector from `self` and the given `w` value.
			extend(self,u32) -> LuaUVec4 ,
			
			///Creates a `Vec2` from the `x` and `y` elements of `self`, discarding `z`.
			///
			///Truncation may also be performed by using `self.xy()` or `Vec2::from()`.
			truncate(self) -> LuaUVec2 ,
			
			///Computes the cross product of `self` and `other`.
			cross(self,LuaUVec3) -> LuaUVec3 ,
		
//	
//			///`[x, y, z]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[u32;3]` in type: `Some("UVec3")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(u32) -> LuaUVec3 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaUVec3 
//			Error: Unsupported argument `BVec3` in type: `Some("UVec3")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaUVec3) -> u32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaUVec3) -> LuaUVec3 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaUVec3) -> LuaUVec3 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaUVec3,LuaUVec3) -> LuaUVec3 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> u32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> u32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaUVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("UVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaUVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("UVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaUVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("UVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaUVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("UVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaUVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("UVec3")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaUVec3) ->  
//			Error: Unsupported return type `BVec3` in type: `Some("UVec3")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaUVec3 
//			Error: Unsupported argument `&[u32]` in type: `Some("UVec3")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [u32]` in type: `Some("UVec3")`.,
			
			///Casts all elements of `self` to `f32`.
			as_vec3(&self) -> LuaVec3 ,
		
//	
//			///Casts all elements of `self` to `f32`.
//			as_vec3a(&self) ->  
//			Error: Unsupported return type `Vec3A` in type: `Some("UVec3")`.,
			
			///Casts all elements of `self` to `f64`.
			as_dvec3(&self) -> LuaDVec3 ,
			
			///Casts all elements of `self` to `i32`.
			as_ivec3(&self) -> LuaIVec3 
		) 
},
{
    
	///A 4-dimensional vector.
    glam::vec4::UVec4 : Reflect:
            Copy(LuaVec2 -> (MetaMethod::Index) (s=LuaUVec4),
        LuaVec2 -> mut (MetaMethod::NewIndex) (n=u32))
    
		+ UnaryOps(
			
			) 
		+ BinOps(
			self Add LuaUVec4 -> LuaUVec4,
			self Add u32 -> LuaUVec4,
			u32 Add self -> LuaUVec4,
			self Sub LuaUVec4 -> LuaUVec4,
			self Sub u32 -> LuaUVec4,
			u32 Sub self -> LuaUVec4,
			self Div LuaUVec4 -> LuaUVec4,
			self Div u32 -> LuaUVec4,
			u32 Div self -> LuaUVec4,
			self Mul LuaUVec4 -> LuaUVec4,
			self Mul u32 -> LuaUVec4,
			u32 Mul self -> LuaUVec4,
			self Rem LuaUVec4 -> LuaUVec4,
			self Rem u32 -> LuaUVec4,
			u32 Rem self -> LuaUVec4
			) 
		+ AutoMethods(
			
			///Creates a new 4D vector.
			new(u32,u32,u32,u32) -> LuaUVec4 ,
			
			///Creates a `Vec3` from the `x`, `y` and `z` elements of `self`, discarding `w`.
			///
			///Truncation to `Vec3` may also be performed by using `self.xyz()` or `Vec3::from()`.
			///
			///To truncate to `Vec3A` use `Vec3A::from()`.
			truncate(self) -> LuaUVec3 ,
		
//	
//			///`[x, y, z, w]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[u32;4]` in type: `Some("UVec4")`.,
			
			///Creates a vector with all elements set to `v`.
			splat(u32) -> LuaUVec4 ,
		
//	
//			///Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
//			///for each element of `self`.
//			///
//			///A true element in the mask uses the corresponding element from `if_true`, and false
//			///uses the element from `if_false`.
//			select() -> LuaUVec4 
//			Error: Unsupported argument `BVec4` in type: `Some("UVec4")`.,
			
			///Computes the dot product of `self` and `other`.
			dot(self,LuaUVec4) -> u32 ,
			
			///Returns a vector containing the minimum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.min(other.x), self.y.min(other.y), ..]`.
			min(self,LuaUVec4) -> LuaUVec4 ,
			
			///Returns a vector containing the maximum values for each element of `self` and `other`.
			///
			///In other words this computes `[self.x.max(other.x), self.y.max(other.y), ..]`.
			max(self,LuaUVec4) -> LuaUVec4 ,
			
			///Component-wise clamping of values, similar to [`f32::clamp`].
			///
			///Each element in `min` must be less-or-equal to the corresponding element in `max`.
			///
			///# Panics
			///
			///Will panic if `min` is greater than `max` when `glam_assert` is enabled.
			clamp(self,LuaUVec4,LuaUVec4) -> LuaUVec4 ,
			
			///Returns the horizontal minimum of `self`.
			///
			///In other words this computes `min(x, y, ..)`.
			min_element(self) -> u32 ,
			
			///Returns the horizontal maximum of `self`.
			///
			///In other words this computes `max(x, y, ..)`.
			max_element(self) -> u32 ,
		
//	
//			///Returns a vector mask containing the result of a `==` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words, this computes `[self.x == other.x, self.y == other.y, ..]` for all
//			///elements.
//			cmpeq(self,LuaUVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("UVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `!=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x != other.x, self.y != other.y, ..]` for all
//			///elements.
//			cmpne(self,LuaUVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("UVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `>=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x >= other.x, self.y >= other.y, ..]` for all
//			///elements.
//			cmpge(self,LuaUVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("UVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `>` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x > other.x, self.y > other.y, ..]` for all
//			///elements.
//			cmpgt(self,LuaUVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("UVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `<=` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x <= other.x, self.y <= other.y, ..]` for all
//			///elements.
//			cmple(self,LuaUVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("UVec4")`.,
		
//	
//			///Returns a vector mask containing the result of a `<` comparison for each element of
//			///`self` and `other`.
//			///
//			///In other words this computes `[self.x < other.x, self.y < other.y, ..]` for all
//			///elements.
//			cmplt(self,LuaUVec4) ->  
//			Error: Unsupported return type `BVec4` in type: `Some("UVec4")`.,
		
//	
//			///Creates a vector from the first N values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			from_slice() -> LuaUVec4 
//			Error: Unsupported argument `&[u32]` in type: `Some("UVec4")`.,
		
//	
//			///Writes the elements of `self` to the first N elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than N elements long.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [u32]` in type: `Some("UVec4")`.,
			
			///Casts all elements of `self` to `f32`.
			as_vec4(&self) -> LuaVec4 ,
			
			///Casts all elements of `self` to `f64`.
			as_dvec4(&self) -> LuaDVec4 ,
			
			///Casts all elements of `self` to `i32`.
			as_ivec4(&self) -> LuaIVec4 
		) 
},
{
    
	///A 3x3 column major matrix.
	///
	///This 3x3 matrix type features convenience methods for creating and using linear and
	///affine transformations. If you are primarily dealing with 2D affine transformations the
	///[`Affine2`](crate::Affine2) type is much faster and more space efficient than using a
	///3x3 matrix.
	///
	///Linear transformations including 3D rotation and scale can be created using methods
	///such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
	///[`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
	///[`Self::from_rotation_z()`].
	///
	///The resulting matrices can be use to transform 3D vectors using regular vector
	///multiplication.
	///
	///Affine transformations including 2D translation, rotation and scale can be created
	///using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
	///[`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
	///
	///The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
	///are provided for performing affine transforms on 2D vectors and points. These multiply
	///2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
	///vectors respectively. These methods assume that `Self` contains a valid affine
	///transform.
    glam::mat3::Mat3 : Reflect:
        UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaMat3 -> LuaMat3,
			self Sub LuaMat3 -> LuaMat3,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "Affine2", id: Id("0:5922:1568"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			// Error: unsupported type in `Method { decl: FnDecl { inputs: [("self", Generic("Self")), ("other", ResolvedPath { name: "Affine2", id: Id("0:5922:1568"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] })], output: Some(QualifiedPath { name: "Output", args: AngleBracketed { args: [], bindings: [] }, self_type: Generic("Self"), trait_: ResolvedPath { name: "", id: Id("2:3257:1919"), args: None, param_names: [] } }), c_variadic: false }, generics: Generics { params: [], where_predicates: [] }, header: Header { const_: false, unsafe_: false, async_: false, abi: Rust }, has_body: true }`,
			self Mul LuaMat3 -> LuaMat3,
			self Mul LuaVec3 -> LuaVec3,
			f32 Mul self -> LuaMat3,
			self Mul f32 -> LuaMat3,
			// Error: unsupported type in `Method { decl: FnDecl { inputs: [("self", Generic("Self")), ("other", ResolvedPath { name: "Vec3A", id: Id("0:9222:1579"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] })], output: Some(ResolvedPath { name: "Vec3A", id: Id("0:9222:1579"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }), c_variadic: false }, generics: Generics { params: [], where_predicates: [] }, header: Header { const_: false, unsafe_: false, async_: false, abi: Rust }, has_body: true }`
			) 
		+ AutoMethods(
			
			///Creates a 3x3 matrix from three column vectors.
			from_cols(LuaVec3,LuaVec3,LuaVec3) -> LuaMat3 ,
		
//	
//			///Creates a 3x3 matrix from a `[S; 9]` array stored in column major order.
//			///If your data is stored in row major you will need to `transpose` the returned
//			///matrix.
//			from_cols_array() -> LuaMat3 
//			Error: Unsupported argument `&[f32;9]` in type: `Some("Mat3")`.,
		
//	
//			///Creates a `[S; 9]` array storing data in column major order.
//			///If you require data in row major order `transpose` the matrix first.
//			to_cols_array(&self) ->  
//			Error: Unsupported return type `[f32;9]` in type: `Some("Mat3")`.,
		
//	
//			///Creates a 3x3 matrix from a `[[S; 3]; 3]` 2D array stored in column major order.
//			///If your data is in row major order you will need to `transpose` the returned
//			///matrix.
//			from_cols_array_2d() -> LuaMat3 
//			Error: Unsupported argument `&[[f32;3];3]` in type: `Some("Mat3")`.,
		
//	
//			///Creates a `[[S; 3]; 3]` 2D array storing data in column major order.
//			///If you require data in row major order `transpose` the matrix first.
//			to_cols_array_2d(&self) ->  
//			Error: Unsupported return type `[[f32;3];3]` in type: `Some("Mat3")`.,
			
			///Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
			///The resulting matrix is a 3D scale transfom.
			from_diagonal(LuaVec3) -> LuaMat3 ,
			
			///Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
			from_mat4(LuaMat4) -> LuaMat3 ,
			
			///Creates a 3D rotation matrix from the given quaternion.
			///
			///# Panics
			///
			///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
			from_quat(LuaQuat) -> LuaMat3 ,
			
			///Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
			///radians).
			///
			///# Panics
			///
			///Will panic if `axis` is not normalized when `glam_assert` is enabled.
			from_axis_angle(LuaVec3,f32) -> LuaMat3 ,
			
			///Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
			///radians).
			from_euler(LuaEulerRot,f32,f32,f32) -> LuaMat3 ,
			
			///Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
			from_rotation_x(f32) -> LuaMat3 ,
			
			///Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
			from_rotation_y(f32) -> LuaMat3 ,
			
			///Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
			from_rotation_z(f32) -> LuaMat3 ,
			
			///Creates an affine transformation matrix from the given 2D `translation`.
			///
			///The resulting matrix can be used to transform 2D points and vectors. See
			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
			from_translation(LuaVec2) -> LuaMat3 ,
			
			///Creates an affine transformation matrix from the given 2D rotation `angle` (in
			///radians).
			///
			///The resulting matrix can be used to transform 2D points and vectors. See
			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
			from_angle(f32) -> LuaMat3 ,
			
			///Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
			///radians) and `translation`.
			///
			///The resulting matrix can be used to transform 2D points and vectors. See
			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
			from_scale_angle_translation(LuaVec2,f32,LuaVec2) -> LuaMat3 ,
			
			///Creates an affine transformation matrix from the given non-uniform 2D `scale`.
			///
			///The resulting matrix can be used to transform 2D points and vectors. See
			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
			///
			///# Panics
			///
			///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
			from_scale(LuaVec2) -> LuaMat3 ,
		
//	
//			///Creates an affine transformation matrix from the given 2x2 matrix.
//			///
//			///The resulting matrix can be used to transform 2D points and vectors. See
//			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
//			from_mat2() -> LuaMat3 
//			Error: Unsupported argument `Mat2` in type: `Some("Mat3")`.,
		
//	
//			///Creates a 3x3 matrix from the first 9 values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than 9 elements long.
//			from_cols_slice() -> LuaMat3 
//			Error: Unsupported argument `&[f32]` in type: `Some("Mat3")`.,
		
//	
//			///Writes the columns of `self` to the first 9 elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than 9 elements long.
//			write_cols_to_slice()  
//			Error: Unsupported argument `&mut [f32]` in type: `Some("Mat3")`.,
			
			///Returns the matrix column for the given `index`.
			///
			///# Panics
			///
			///Panics if `index` is greater than 2.
			col(&self,usize) -> LuaVec3 ,
		
//	
//			///Returns a mutable reference to the matrix column for the given `index`.
//			///
//			///# Panics
//			///
//			///Panics if `index` is greater than 2.
//			col_mut() ->  
//			Error: Unsupported argument `&mut self` in type: `Some("Mat3")`.,
			
			///Returns the matrix row for the given `index`.
			///
			///# Panics
			///
			///Panics if `index` is greater than 2.
			row(&self,usize) -> LuaVec3 ,
			
			///Returns `true` if, and only if, all elements are finite.
			///If any element is either `NaN`, positive or negative infinity, this will return `false`.
			is_finite(&self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(&self) -> bool ,
			
			///Returns the transpose of `self`.
			transpose(&self) -> LuaMat3 ,
			
			///Returns the determinant of `self`.
			determinant(&self) -> f32 ,
			
			///Returns the inverse of `self`.
			///
			///If the matrix is not invertible the returned matrix will be invalid.
			///
			///# Panics
			///
			///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
			inverse(&self) -> LuaMat3 ,
			
			///Transforms a 3D vector.
			mul_vec3(&self,LuaVec3) -> LuaVec3 ,
		
//	
//			///Multiplies two 3x3 matrices.
//			mul_mat3() -> LuaMat3 
//			Error: Unsupported argument `&LuaMat3` in type: `Some("Mat3")`.,
		
//	
//			///Adds two 3x3 matrices.
//			add_mat3() -> LuaMat3 
//			Error: Unsupported argument `&LuaMat3` in type: `Some("Mat3")`.,
		
//	
//			///Subtracts two 3x3 matrices.
//			sub_mat3() -> LuaMat3 
//			Error: Unsupported argument `&LuaMat3` in type: `Some("Mat3")`.,
			
			///Multiplies a 3x3 matrix by a scalar.
			mul_scalar(&self,f32) -> LuaMat3 ,
			
			///Transforms the given 2D vector as a point.
			///
			///This is the equivalent of multiplying `other` as a 3D vector where `z` is `1`.
			///
			///This method assumes that `self` contains a valid affine transform.
			transform_point2(&self,LuaVec2) -> LuaVec2 ,
			
			///Rotates the given 2D vector.
			///
			///This is the equivalent of multiplying `other` as a 3D vector where `z` is `0`.
			///
			///This method assumes that `self` contains a valid affine transform.
			transform_vector2(&self,LuaVec2) -> LuaVec2 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other`
			///is less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two matrices contain similar elements. It works best
			///when comparing with a known value. The `max_abs_diff` that should be used used
			///depends on the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(&self,LuaMat3,f32) -> bool ,
		
//	
//			///Transforms a `Vec3A`.
//			mul_vec3a() ->  
//			Error: Unsupported argument `Vec3A` in type: `Some("Mat3")`.,
			
			as_dmat3(&self) -> LuaDMat3 
		) 
	impl {

			    
mut (MetaMethod::Index) (s=LuaMat3,b=Mat3,v=LuaVec3) => {|_,s,idx : usize| {
    match s {
        ($s)::Owned(ref mut v, ref valid) => {
            Ok(($v)::Ref(LuaRef{
                root: LuaRefBase::LuaOwned{valid: Arc::downgrade((valid))},
                r: ReflectPtr::Mut(v.col_mut(idx-1)),
                path: None
            }))
        },
        ($s)::Ref(ref mut r) => {
            r.get_mut(|s,r| {
                Ok(($v)::Ref(LuaRef{
                    root: r.root.clone(),
                    r: ReflectPtr::Mut(s.downcast_mut::<($b)>().unwrap().col_mut(idx-1)),
                    path: None
                })) 
            })
        }
    }
}}
;}
},
{
    
	///A 4x4 column major matrix.
	///
	///This 4x4 matrix type features convenience methods for creating and using affine transforms and
	///perspective projections. If you are primarily dealing with 3D affine transformations
	///considering using [`Affine3A`](crate::Affine3A) which is faster than a 4x4 matrix for some
	///affine operations.
	///
	///Affine transformations including 3D translation, rotation and scale can be created
	///using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
	///[`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
	///
	///Othographic projections can be created using the methods [`Self::orthographic_lh()`] for
	///left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
	///systems. The resulting matrix is also an affine transformation.
	///
	///The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
	///are provided for performing affine transformations on 3D vectors and points. These
	///multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
	///for vectors respectively. These methods assume that `Self` contains a valid affine
	///transform.
	///
	///Perspective projections can be created using methods such as
	///[`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
	///[`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
	///[`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
	///[`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
	///
	///The resulting perspective project can be use to transform 3D vectors as points with
	///perspective correction using the [`Self::project_point3()`] convenience method.
    glam::mat4::Mat4 : Reflect:
        Copy(LuaMat3 -> mut (MetaMethod::Index) (s=LuaMat4,b=Mat4,v=LuaVec4))
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaMat4 -> LuaMat4,
			self Sub LuaMat4 -> LuaMat4,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "Affine3A", id: Id("0:6090:1569"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			// Error: unsupported type in `Method { decl: FnDecl { inputs: [("self", Generic("Self")), ("rhs", ResolvedPath { name: "Affine3A", id: Id("0:6090:1569"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] })], output: Some(QualifiedPath { name: "Output", args: AngleBracketed { args: [], bindings: [] }, self_type: Generic("Self"), trait_: ResolvedPath { name: "", id: Id("2:3257:1919"), args: None, param_names: [] } }), c_variadic: false }, generics: Generics { params: [], where_predicates: [] }, header: Header { const_: false, unsafe_: false, async_: false, abi: Rust }, has_body: true }`,
			self Mul LuaMat4 -> LuaMat4,
			self Mul LuaVec4 -> LuaVec4,
			f32 Mul self -> LuaMat4,
			self Mul f32 -> LuaMat4
			) 
		+ AutoMethods(
			
			///Creates a 4x4 matrix from four column vectors.
			from_cols(LuaVec4,LuaVec4,LuaVec4,LuaVec4) -> LuaMat4 ,
		
//	
//			///Creates a 4x4 matrix from a `[S; 16]` array stored in column major order.
//			///If your data is stored in row major you will need to `transpose` the returned
//			///matrix.
//			from_cols_array() -> LuaMat4 
//			Error: Unsupported argument `&[f32;16]` in type: `Some("Mat4")`.,
		
//	
//			///Creates a `[S; 16]` array storing data in column major order.
//			///If you require data in row major order `transpose` the matrix first.
//			to_cols_array(&self) ->  
//			Error: Unsupported return type `[f32;16]` in type: `Some("Mat4")`.,
		
//	
//			///Creates a 4x4 matrix from a `[[S; 4]; 4]` 2D array stored in column major order.
//			///If your data is in row major order you will need to `transpose` the returned
//			///matrix.
//			from_cols_array_2d() -> LuaMat4 
//			Error: Unsupported argument `&[[f32;4];4]` in type: `Some("Mat4")`.,
		
//	
//			///Creates a `[[S; 4]; 4]` 2D array storing data in column major order.
//			///If you require data in row major order `transpose` the matrix first.
//			to_cols_array_2d(&self) ->  
//			Error: Unsupported return type `[[f32;4];4]` in type: `Some("Mat4")`.,
			
			///Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
			from_diagonal(LuaVec4) -> LuaMat4 ,
			
			///Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
			///`translation`.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
			from_scale_rotation_translation(LuaVec3,LuaQuat,LuaVec3) -> LuaMat4 ,
			
			///Creates an affine transformation matrix from the given 3D `translation`.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
			from_rotation_translation(LuaQuat,LuaVec3) -> LuaMat4 ,
		
//	
//			///Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is
//			///expected to be a 3D affine transformation matrix otherwise the output will be invalid.
//			///
//			///# Panics
//			///
//			///Will panic if the determinant of `self` is zero or if the resulting scale vector
//			///contains any zero elements when `glam_assert` is enabled.
//			to_scale_rotation_translation(&self) ->  
//			Error: Unsupported return type `(LuaVec3,LuaQuat,LuaVec3)` in type: `Some("Mat4")`.,
			
			///Creates an affine transformation matrix from the given `rotation` quaternion.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
			from_quat(LuaQuat) -> LuaMat4 ,
			
			///Creates an affine transformation matrix from the given 3x3 linear transformation
			///matrix.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_mat3(LuaMat3) -> LuaMat4 ,
			
			///Creates an affine transformation matrix from the given 3D `translation`.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_translation(LuaVec3) -> LuaMat4 ,
			
			///Creates an affine transformation matrix containing a 3D rotation around a normalized
			///rotation `axis` of `angle` (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if `axis` is not normalized when `glam_assert` is enabled.
			from_axis_angle(LuaVec3,f32) -> LuaMat4 ,
			
			///Creates a affine transformation matrix containing a rotation from the given euler
			///rotation sequence and angles (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_euler(LuaEulerRot,f32,f32,f32) -> LuaMat4 ,
			
			///Creates an affine transformation matrix containing a 3D rotation around the x axis of
			///`angle` (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_rotation_x(f32) -> LuaMat4 ,
			
			///Creates an affine transformation matrix containing a 3D rotation around the y axis of
			///`angle` (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_rotation_y(f32) -> LuaMat4 ,
			
			///Creates an affine transformation matrix containing a 3D rotation around the z axis of
			///`angle` (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_rotation_z(f32) -> LuaMat4 ,
			
			///Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
			from_scale(LuaVec3) -> LuaMat4 ,
		
//	
//			///Creates a 4x4 matrix from the first 16 values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than 16 elements long.
//			from_cols_slice() -> LuaMat4 
//			Error: Unsupported argument `&[f32]` in type: `Some("Mat4")`.,
		
//	
//			///Writes the columns of `self` to the first 16 elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than 16 elements long.
//			write_cols_to_slice()  
//			Error: Unsupported argument `&mut [f32]` in type: `Some("Mat4")`.,
			
			///Returns the matrix column for the given `index`.
			///
			///# Panics
			///
			///Panics if `index` is greater than 3.
			col(&self,usize) -> LuaVec4 ,
		
//	
//			///Returns a mutable reference to the matrix column for the given `index`.
//			///
//			///# Panics
//			///
//			///Panics if `index` is greater than 3.
//			col_mut() ->  
//			Error: Unsupported argument `&mut self` in type: `Some("Mat4")`.,
			
			///Returns the matrix row for the given `index`.
			///
			///# Panics
			///
			///Panics if `index` is greater than 3.
			row(&self,usize) -> LuaVec4 ,
			
			///Returns `true` if, and only if, all elements are finite.
			///If any element is either `NaN`, positive or negative infinity, this will return `false`.
			is_finite(&self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(&self) -> bool ,
			
			///Returns the transpose of `self`.
			transpose(&self) -> LuaMat4 ,
			
			///Returns the determinant of `self`.
			determinant(&self) -> f32 ,
			
			///Returns the inverse of `self`.
			///
			///If the matrix is not invertible the returned matrix will be invalid.
			///
			///# Panics
			///
			///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
			inverse(&self) -> LuaMat4 ,
			
			///Creates a left-handed view matrix using a camera position, an up direction, and a focal
			///point.
			///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
			///
			///# Panics
			///
			///Will panic if `up` is not normalized when `glam_assert` is enabled.
			look_at_lh(LuaVec3,LuaVec3,LuaVec3) -> LuaMat4 ,
			
			///Creates a right-handed view matrix using a camera position, an up direction, and a focal
			///point.
			///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
			///
			///# Panics
			///
			///Will panic if `up` is not normalized when `glam_assert` is enabled.
			look_at_rh(LuaVec3,LuaVec3,LuaVec3) -> LuaMat4 ,
			
			///Creates a right-handed perspective projection matrix with [-1,1] depth range.
			///This is the same as the OpenGL `gluPerspective` function.
			///See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
			perspective_rh_gl(f32,f32,f32,f32) -> LuaMat4 ,
			
			///Creates a left-handed perspective projection matrix with `[0,1]` depth range.
			///
			///# Panics
			///
			///Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
			///enabled.
			perspective_lh(f32,f32,f32,f32) -> LuaMat4 ,
			
			///Creates a right-handed perspective projection matrix with `[0,1]` depth range.
			///
			///# Panics
			///
			///Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
			///enabled.
			perspective_rh(f32,f32,f32,f32) -> LuaMat4 ,
			
			///Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
			///
			///# Panics
			///
			///Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
			perspective_infinite_lh(f32,f32,f32) -> LuaMat4 ,
			
			///Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
			///
			///# Panics
			///
			///Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
			perspective_infinite_reverse_lh(f32,f32,f32) -> LuaMat4 ,
			
			///Creates an infinite right-handed perspective projection matrix with
			///`[0,1]` depth range.
			perspective_infinite_rh(f32,f32,f32) -> LuaMat4 ,
			
			///Creates an infinite reverse right-handed perspective projection matrix
			///with `[0,1]` depth range.
			perspective_infinite_reverse_rh(f32,f32,f32) -> LuaMat4 ,
			
			///Creates a right-handed orthographic projection matrix with `[-1,1]` depth
			///range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
			///See
			///<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
			orthographic_rh_gl(f32,f32,f32,f32,f32,f32) -> LuaMat4 ,
			
			///Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
			orthographic_lh(f32,f32,f32,f32,f32,f32) -> LuaMat4 ,
			
			///Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
			orthographic_rh(f32,f32,f32,f32,f32,f32) -> LuaMat4 ,
			
			///Transforms a 4D vector.
			mul_vec4(&self,LuaVec4) -> LuaVec4 ,
		
//	
//			///Multiplies two 4x4 matrices.
//			mul_mat4() -> LuaMat4 
//			Error: Unsupported argument `&LuaMat4` in type: `Some("Mat4")`.,
		
//	
//			///Adds two 4x4 matrices.
//			add_mat4() -> LuaMat4 
//			Error: Unsupported argument `&LuaMat4` in type: `Some("Mat4")`.,
		
//	
//			///Subtracts two 4x4 matrices.
//			sub_mat4() -> LuaMat4 
//			Error: Unsupported argument `&LuaMat4` in type: `Some("Mat4")`.,
			
			///Multiplies this matrix by a scalar value.
			mul_scalar(&self,f32) -> LuaMat4 ,
			
			///Transforms the given 3D vector as a point, applying perspective correction.
			///
			///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
			///The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
			///
			///This method assumes that `self` contains a projective transform.
			project_point3(&self,LuaVec3) -> LuaVec3 ,
			
			///Transforms the given 3D vector as a point.
			///
			///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
			///`1.0`.
			///
			///This method assumes that `self` contains a valid affine transform. It does not perform
			///a persective divide, if `self` contains a perspective transform, or if you are unsure,
			///the [`Self::project_point3()`] method should be used instead.
			///
			///# Panics
			///
			///Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
			transform_point3(&self,LuaVec3) -> LuaVec3 ,
			
			///Transforms the give 3D vector as a direction.
			///
			///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
			///`0.0`.
			///
			///This method assumes that `self` contains a valid affine transform.
			///
			///# Panics
			///
			///Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
			transform_vector3(&self,LuaVec3) -> LuaVec3 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other`
			///is less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two 4x4 matrices contain similar elements. It works
			///best when comparing with a known value. The `max_abs_diff` that should be used used
			///depends on the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(&self,LuaMat4,f32) -> bool ,
		
//	
//			///Transforms the given `Vec3A` as 3D point.
//			///
//			///This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `1.0`.
//			transform_point3a() ->  
//			Error: Unsupported argument `Vec3A` in type: `Some("Mat4")`.,
		
//	
//			///Transforms the give `Vec3A` as 3D vector.
//			///
//			///This is the equivalent of multiplying the `Vec3A` as a 4D vector where `w` is `0.0`.
//			transform_vector3a() ->  
//			Error: Unsupported argument `Vec3A` in type: `Some("Mat4")`.,
			
			as_dmat4(&self) -> LuaDMat4 
		) 
},
{
    
	///A 3x3 column major matrix.
	///
	///This 3x3 matrix type features convenience methods for creating and using linear and
	///affine transformations. If you are primarily dealing with 2D affine transformations the
	///[`Affine2`](crate::Affine2) type is much faster and more space efficient than using a
	///3x3 matrix.
	///
	///Linear transformations including 3D rotation and scale can be created using methods
	///such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
	///[`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
	///[`Self::from_rotation_z()`].
	///
	///The resulting matrices can be use to transform 3D vectors using regular vector
	///multiplication.
	///
	///Affine transformations including 2D translation, rotation and scale can be created
	///using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
	///[`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
	///
	///The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
	///are provided for performing affine transforms on 2D vectors and points. These multiply
	///2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
	///vectors respectively. These methods assume that `Self` contains a valid affine
	///transform.
    glam::mat3::DMat3 : Reflect:
        Copy(LuaMat3 -> mut (MetaMethod::Index) (s=LuaDMat3,b=DMat3,v=LuaDVec3))
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaDMat3 -> LuaDMat3,
			self Sub LuaDMat3 -> LuaDMat3,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "DAffine2", id: Id("0:6000:1584"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			// Error: unsupported type in `Method { decl: FnDecl { inputs: [("self", Generic("Self")), ("other", ResolvedPath { name: "DAffine2", id: Id("0:6000:1584"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] })], output: Some(QualifiedPath { name: "Output", args: AngleBracketed { args: [], bindings: [] }, self_type: Generic("Self"), trait_: ResolvedPath { name: "", id: Id("2:3257:1919"), args: None, param_names: [] } }), c_variadic: false }, generics: Generics { params: [], where_predicates: [] }, header: Header { const_: false, unsafe_: false, async_: false, abi: Rust }, has_body: true }`,
			self Mul LuaDMat3 -> LuaDMat3,
			self Mul LuaDVec3 -> LuaDVec3,
			f64 Mul self -> LuaDMat3,
			self Mul f64 -> LuaDMat3
			) 
		+ AutoMethods(
			
			///Creates a 3x3 matrix from three column vectors.
			from_cols(LuaDVec3,LuaDVec3,LuaDVec3) -> LuaDMat3 ,
		
//	
//			///Creates a 3x3 matrix from a `[S; 9]` array stored in column major order.
//			///If your data is stored in row major you will need to `transpose` the returned
//			///matrix.
//			from_cols_array() -> LuaDMat3 
//			Error: Unsupported argument `&[f64;9]` in type: `Some("DMat3")`.,
		
//	
//			///Creates a `[S; 9]` array storing data in column major order.
//			///If you require data in row major order `transpose` the matrix first.
//			to_cols_array(&self) ->  
//			Error: Unsupported return type `[f64;9]` in type: `Some("DMat3")`.,
		
//	
//			///Creates a 3x3 matrix from a `[[S; 3]; 3]` 2D array stored in column major order.
//			///If your data is in row major order you will need to `transpose` the returned
//			///matrix.
//			from_cols_array_2d() -> LuaDMat3 
//			Error: Unsupported argument `&[[f64;3];3]` in type: `Some("DMat3")`.,
		
//	
//			///Creates a `[[S; 3]; 3]` 2D array storing data in column major order.
//			///If you require data in row major order `transpose` the matrix first.
//			to_cols_array_2d(&self) ->  
//			Error: Unsupported return type `[[f64;3];3]` in type: `Some("DMat3")`.,
			
			///Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.
			///The resulting matrix is a 3D scale transfom.
			from_diagonal(LuaDVec3) -> LuaDMat3 ,
			
			///Creates a 3x3 matrix from a 4x4 matrix, discarding the 3rd row and column.
			from_mat4(LuaDMat4) -> LuaDMat3 ,
			
			///Creates a 3D rotation matrix from the given quaternion.
			///
			///# Panics
			///
			///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
			from_quat(LuaDQuat) -> LuaDMat3 ,
			
			///Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
			///radians).
			///
			///# Panics
			///
			///Will panic if `axis` is not normalized when `glam_assert` is enabled.
			from_axis_angle(LuaDVec3,f64) -> LuaDMat3 ,
			
			///Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
			///radians).
			from_euler(LuaEulerRot,f64,f64,f64) -> LuaDMat3 ,
			
			///Creates a 3D rotation matrix from `angle` (in radians) around the x axis.
			from_rotation_x(f64) -> LuaDMat3 ,
			
			///Creates a 3D rotation matrix from `angle` (in radians) around the y axis.
			from_rotation_y(f64) -> LuaDMat3 ,
			
			///Creates a 3D rotation matrix from `angle` (in radians) around the z axis.
			from_rotation_z(f64) -> LuaDMat3 ,
			
			///Creates an affine transformation matrix from the given 2D `translation`.
			///
			///The resulting matrix can be used to transform 2D points and vectors. See
			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
			from_translation(LuaDVec2) -> LuaDMat3 ,
			
			///Creates an affine transformation matrix from the given 2D rotation `angle` (in
			///radians).
			///
			///The resulting matrix can be used to transform 2D points and vectors. See
			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
			from_angle(f64) -> LuaDMat3 ,
			
			///Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
			///radians) and `translation`.
			///
			///The resulting matrix can be used to transform 2D points and vectors. See
			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
			from_scale_angle_translation(LuaDVec2,f64,LuaDVec2) -> LuaDMat3 ,
			
			///Creates an affine transformation matrix from the given non-uniform 2D `scale`.
			///
			///The resulting matrix can be used to transform 2D points and vectors. See
			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
			///
			///# Panics
			///
			///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
			from_scale(LuaDVec2) -> LuaDMat3 ,
		
//	
//			///Creates an affine transformation matrix from the given 2x2 matrix.
//			///
//			///The resulting matrix can be used to transform 2D points and vectors. See
//			///[`Self::transform_point2()`] and [`Self::transform_vector2()`].
//			from_mat2() -> LuaDMat3 
//			Error: Unsupported argument `DMat2` in type: `Some("DMat3")`.,
		
//	
//			///Creates a 3x3 matrix from the first 9 values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than 9 elements long.
//			from_cols_slice() -> LuaDMat3 
//			Error: Unsupported argument `&[f64]` in type: `Some("DMat3")`.,
		
//	
//			///Writes the columns of `self` to the first 9 elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than 9 elements long.
//			write_cols_to_slice()  
//			Error: Unsupported argument `&mut [f64]` in type: `Some("DMat3")`.,
			
			///Returns the matrix column for the given `index`.
			///
			///# Panics
			///
			///Panics if `index` is greater than 2.
			col(&self,usize) -> LuaDVec3 ,
		
//	
//			///Returns a mutable reference to the matrix column for the given `index`.
//			///
//			///# Panics
//			///
//			///Panics if `index` is greater than 2.
//			col_mut() ->  
//			Error: Unsupported argument `&mut self` in type: `Some("DMat3")`.,
			
			///Returns the matrix row for the given `index`.
			///
			///# Panics
			///
			///Panics if `index` is greater than 2.
			row(&self,usize) -> LuaDVec3 ,
			
			///Returns `true` if, and only if, all elements are finite.
			///If any element is either `NaN`, positive or negative infinity, this will return `false`.
			is_finite(&self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(&self) -> bool ,
			
			///Returns the transpose of `self`.
			transpose(&self) -> LuaDMat3 ,
			
			///Returns the determinant of `self`.
			determinant(&self) -> f64 ,
			
			///Returns the inverse of `self`.
			///
			///If the matrix is not invertible the returned matrix will be invalid.
			///
			///# Panics
			///
			///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
			inverse(&self) -> LuaDMat3 ,
			
			///Transforms a 3D vector.
			mul_vec3(&self,LuaDVec3) -> LuaDVec3 ,
		
//	
//			///Multiplies two 3x3 matrices.
//			mul_mat3() -> LuaDMat3 
//			Error: Unsupported argument `&LuaDMat3` in type: `Some("DMat3")`.,
		
//	
//			///Adds two 3x3 matrices.
//			add_mat3() -> LuaDMat3 
//			Error: Unsupported argument `&LuaDMat3` in type: `Some("DMat3")`.,
		
//	
//			///Subtracts two 3x3 matrices.
//			sub_mat3() -> LuaDMat3 
//			Error: Unsupported argument `&LuaDMat3` in type: `Some("DMat3")`.,
			
			///Multiplies a 3x3 matrix by a scalar.
			mul_scalar(&self,f64) -> LuaDMat3 ,
			
			///Transforms the given 2D vector as a point.
			///
			///This is the equivalent of multiplying `other` as a 3D vector where `z` is `1`.
			///
			///This method assumes that `self` contains a valid affine transform.
			transform_point2(&self,LuaDVec2) -> LuaDVec2 ,
			
			///Rotates the given 2D vector.
			///
			///This is the equivalent of multiplying `other` as a 3D vector where `z` is `0`.
			///
			///This method assumes that `self` contains a valid affine transform.
			transform_vector2(&self,LuaDVec2) -> LuaDVec2 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other`
			///is less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two matrices contain similar elements. It works best
			///when comparing with a known value. The `max_abs_diff` that should be used used
			///depends on the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(&self,LuaDMat3,f64) -> bool ,
			
			as_mat3(&self) -> LuaMat3 
		) 
},
{
    
	///A 4x4 column major matrix.
	///
	///This 4x4 matrix type features convenience methods for creating and using affine transforms and
	///perspective projections. If you are primarily dealing with 3D affine transformations
	///considering using [`DAffine3`](crate::DAffine3) which is faster than a 4x4 matrix for some
	///affine operations.
	///
	///Affine transformations including 3D translation, rotation and scale can be created
	///using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
	///[`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
	///
	///Othographic projections can be created using the methods [`Self::orthographic_lh()`] for
	///left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
	///systems. The resulting matrix is also an affine transformation.
	///
	///The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
	///are provided for performing affine transformations on 3D vectors and points. These
	///multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
	///for vectors respectively. These methods assume that `Self` contains a valid affine
	///transform.
	///
	///Perspective projections can be created using methods such as
	///[`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
	///[`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
	///[`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
	///[`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
	///
	///The resulting perspective project can be use to transform 3D vectors as points with
	///perspective correction using the [`Self::project_point3()`] convenience method.
    glam::mat4::DMat4 : Reflect:
        Copy(LuaMat3 -> mut (MetaMethod::Index) (s=LuaDMat4,b=DMat4,v=LuaDVec4))
		+ UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaDMat4 -> LuaDMat4,
			self Sub LuaDMat4 -> LuaDMat4,
			
//			 Error: unsupported lhs operator `ResolvedPath { name: "DAffine3", id: Id("0:6176:1585"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] }` in `Mul`,
			// Error: unsupported type in `Method { decl: FnDecl { inputs: [("self", Generic("Self")), ("rhs", ResolvedPath { name: "DAffine3", id: Id("0:6176:1585"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] })], output: Some(QualifiedPath { name: "Output", args: AngleBracketed { args: [], bindings: [] }, self_type: Generic("Self"), trait_: ResolvedPath { name: "", id: Id("2:3257:1919"), args: None, param_names: [] } }), c_variadic: false }, generics: Generics { params: [], where_predicates: [] }, header: Header { const_: false, unsafe_: false, async_: false, abi: Rust }, has_body: true }`,
			self Mul LuaDMat4 -> LuaDMat4,
			self Mul LuaDVec4 -> LuaDVec4,
			f64 Mul self -> LuaDMat4,
			self Mul f64 -> LuaDMat4
			) 
		+ AutoMethods(
			
			///Creates a 4x4 matrix from four column vectors.
			from_cols(LuaDVec4,LuaDVec4,LuaDVec4,LuaDVec4) -> LuaDMat4 ,
		
//	
//			///Creates a 4x4 matrix from a `[S; 16]` array stored in column major order.
//			///If your data is stored in row major you will need to `transpose` the returned
//			///matrix.
//			from_cols_array() -> LuaDMat4 
//			Error: Unsupported argument `&[f64;16]` in type: `Some("DMat4")`.,
		
//	
//			///Creates a `[S; 16]` array storing data in column major order.
//			///If you require data in row major order `transpose` the matrix first.
//			to_cols_array(&self) ->  
//			Error: Unsupported return type `[f64;16]` in type: `Some("DMat4")`.,
		
//	
//			///Creates a 4x4 matrix from a `[[S; 4]; 4]` 2D array stored in column major order.
//			///If your data is in row major order you will need to `transpose` the returned
//			///matrix.
//			from_cols_array_2d() -> LuaDMat4 
//			Error: Unsupported argument `&[[f64;4];4]` in type: `Some("DMat4")`.,
		
//	
//			///Creates a `[[S; 4]; 4]` 2D array storing data in column major order.
//			///If you require data in row major order `transpose` the matrix first.
//			to_cols_array_2d(&self) ->  
//			Error: Unsupported return type `[[f64;4];4]` in type: `Some("DMat4")`.,
			
			///Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.
			from_diagonal(LuaDVec4) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
			///`translation`.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
			from_scale_rotation_translation(LuaDVec3,LuaDQuat,LuaDVec3) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix from the given 3D `translation`.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
			from_rotation_translation(LuaDQuat,LuaDVec3) -> LuaDMat4 ,
		
//	
//			///Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is
//			///expected to be a 3D affine transformation matrix otherwise the output will be invalid.
//			///
//			///# Panics
//			///
//			///Will panic if the determinant of `self` is zero or if the resulting scale vector
//			///contains any zero elements when `glam_assert` is enabled.
//			to_scale_rotation_translation(&self) ->  
//			Error: Unsupported return type `(LuaDVec3,LuaDQuat,LuaDVec3)` in type: `Some("DMat4")`.,
			
			///Creates an affine transformation matrix from the given `rotation` quaternion.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if `rotation` is not normalized when `glam_assert` is enabled.
			from_quat(LuaDQuat) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix from the given 3x3 linear transformation
			///matrix.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_mat3(LuaDMat3) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix from the given 3D `translation`.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_translation(LuaDVec3) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix containing a 3D rotation around a normalized
			///rotation `axis` of `angle` (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if `axis` is not normalized when `glam_assert` is enabled.
			from_axis_angle(LuaDVec3,f64) -> LuaDMat4 ,
			
			///Creates a affine transformation matrix containing a rotation from the given euler
			///rotation sequence and angles (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_euler(LuaEulerRot,f64,f64,f64) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix containing a 3D rotation around the x axis of
			///`angle` (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_rotation_x(f64) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix containing a 3D rotation around the y axis of
			///`angle` (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_rotation_y(f64) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix containing a 3D rotation around the z axis of
			///`angle` (in radians).
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			from_rotation_z(f64) -> LuaDMat4 ,
			
			///Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
			///
			///The resulting matrix can be used to transform 3D points and vectors. See
			///[`Self::transform_point3()`] and [`Self::transform_vector3()`].
			///
			///# Panics
			///
			///Will panic if all elements of `scale` are zero when `glam_assert` is enabled.
			from_scale(LuaDVec3) -> LuaDMat4 ,
		
//	
//			///Creates a 4x4 matrix from the first 16 values in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than 16 elements long.
//			from_cols_slice() -> LuaDMat4 
//			Error: Unsupported argument `&[f64]` in type: `Some("DMat4")`.,
		
//	
//			///Writes the columns of `self` to the first 16 elements in `slice`.
//			///
//			///# Panics
//			///
//			///Panics if `slice` is less than 16 elements long.
//			write_cols_to_slice()  
//			Error: Unsupported argument `&mut [f64]` in type: `Some("DMat4")`.,
			
			///Returns the matrix column for the given `index`.
			///
			///# Panics
			///
			///Panics if `index` is greater than 3.
			col(&self,usize) -> LuaDVec4 ,
		
//	
//			///Returns a mutable reference to the matrix column for the given `index`.
//			///
//			///# Panics
//			///
//			///Panics if `index` is greater than 3.
//			col_mut() ->  
//			Error: Unsupported argument `&mut self` in type: `Some("DMat4")`.,
			
			///Returns the matrix row for the given `index`.
			///
			///# Panics
			///
			///Panics if `index` is greater than 3.
			row(&self,usize) -> LuaDVec4 ,
			
			///Returns `true` if, and only if, all elements are finite.
			///If any element is either `NaN`, positive or negative infinity, this will return `false`.
			is_finite(&self) -> bool ,
			
			///Returns `true` if any elements are `NaN`.
			is_nan(&self) -> bool ,
			
			///Returns the transpose of `self`.
			transpose(&self) -> LuaDMat4 ,
			
			///Returns the determinant of `self`.
			determinant(&self) -> f64 ,
			
			///Returns the inverse of `self`.
			///
			///If the matrix is not invertible the returned matrix will be invalid.
			///
			///# Panics
			///
			///Will panic if the determinant of `self` is zero when `glam_assert` is enabled.
			inverse(&self) -> LuaDMat4 ,
			
			///Creates a left-handed view matrix using a camera position, an up direction, and a focal
			///point.
			///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
			///
			///# Panics
			///
			///Will panic if `up` is not normalized when `glam_assert` is enabled.
			look_at_lh(LuaDVec3,LuaDVec3,LuaDVec3) -> LuaDMat4 ,
			
			///Creates a right-handed view matrix using a camera position, an up direction, and a focal
			///point.
			///For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
			///
			///# Panics
			///
			///Will panic if `up` is not normalized when `glam_assert` is enabled.
			look_at_rh(LuaDVec3,LuaDVec3,LuaDVec3) -> LuaDMat4 ,
			
			///Creates a right-handed perspective projection matrix with [-1,1] depth range.
			///This is the same as the OpenGL `gluPerspective` function.
			///See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>
			perspective_rh_gl(f64,f64,f64,f64) -> LuaDMat4 ,
			
			///Creates a left-handed perspective projection matrix with `[0,1]` depth range.
			///
			///# Panics
			///
			///Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
			///enabled.
			perspective_lh(f64,f64,f64,f64) -> LuaDMat4 ,
			
			///Creates a right-handed perspective projection matrix with `[0,1]` depth range.
			///
			///# Panics
			///
			///Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
			///enabled.
			perspective_rh(f64,f64,f64,f64) -> LuaDMat4 ,
			
			///Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
			///
			///# Panics
			///
			///Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
			perspective_infinite_lh(f64,f64,f64) -> LuaDMat4 ,
			
			///Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
			///
			///# Panics
			///
			///Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.
			perspective_infinite_reverse_lh(f64,f64,f64) -> LuaDMat4 ,
			
			///Creates an infinite right-handed perspective projection matrix with
			///`[0,1]` depth range.
			perspective_infinite_rh(f64,f64,f64) -> LuaDMat4 ,
			
			///Creates an infinite reverse right-handed perspective projection matrix
			///with `[0,1]` depth range.
			perspective_infinite_reverse_rh(f64,f64,f64) -> LuaDMat4 ,
			
			///Creates a right-handed orthographic projection matrix with `[-1,1]` depth
			///range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
			///See
			///<https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>
			orthographic_rh_gl(f64,f64,f64,f64,f64,f64) -> LuaDMat4 ,
			
			///Creates a left-handed orthographic projection matrix with `[0,1]` depth range.
			orthographic_lh(f64,f64,f64,f64,f64,f64) -> LuaDMat4 ,
			
			///Creates a right-handed orthographic projection matrix with `[0,1]` depth range.
			orthographic_rh(f64,f64,f64,f64,f64,f64) -> LuaDMat4 ,
			
			///Transforms a 4D vector.
			mul_vec4(&self,LuaDVec4) -> LuaDVec4 ,
		
//	
//			///Multiplies two 4x4 matrices.
//			mul_mat4() -> LuaDMat4 
//			Error: Unsupported argument `&LuaDMat4` in type: `Some("DMat4")`.,
		
//	
//			///Adds two 4x4 matrices.
//			add_mat4() -> LuaDMat4 
//			Error: Unsupported argument `&LuaDMat4` in type: `Some("DMat4")`.,
		
//	
//			///Subtracts two 4x4 matrices.
//			sub_mat4() -> LuaDMat4 
//			Error: Unsupported argument `&LuaDMat4` in type: `Some("DMat4")`.,
			
			///Multiplies this matrix by a scalar value.
			mul_scalar(&self,f64) -> LuaDMat4 ,
			
			///Transforms the given 3D vector as a point, applying perspective correction.
			///
			///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
			///The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
			///
			///This method assumes that `self` contains a projective transform.
			project_point3(&self,LuaDVec3) -> LuaDVec3 ,
			
			///Transforms the given 3D vector as a point.
			///
			///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
			///`1.0`.
			///
			///This method assumes that `self` contains a valid affine transform. It does not perform
			///a persective divide, if `self` contains a perspective transform, or if you are unsure,
			///the [`Self::project_point3()`] method should be used instead.
			///
			///# Panics
			///
			///Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
			transform_point3(&self,LuaDVec3) -> LuaDVec3 ,
			
			///Transforms the give 3D vector as a direction.
			///
			///This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
			///`0.0`.
			///
			///This method assumes that `self` contains a valid affine transform.
			///
			///# Panics
			///
			///Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.
			transform_vector3(&self,LuaDVec3) -> LuaDVec3 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other`
			///is less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two 4x4 matrices contain similar elements. It works
			///best when comparing with a known value. The `max_abs_diff` that should be used used
			///depends on the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(&self,LuaDMat4,f64) -> bool ,
			
			as_mat4(&self) -> LuaMat4 
		) 
},
{
    
	///A quaternion representing an orientation.
	///
	///This quaternion is intended to be of unit length but may denormalize due to
	///floating point "error creep" which can occur when successive quaternion
	///operations are applied.
	///
	///This type is 16 byte aligned.
    glam::quat::Quat : Reflect:
        UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaQuat -> LuaQuat,
			self Sub LuaQuat -> LuaQuat,
			self Div f32 -> LuaQuat,
			self Mul f32 -> LuaQuat,
			self Mul LuaQuat -> LuaQuat,
			self Mul LuaVec3 -> LuaVec3,
			// Error: unsupported type in `Method { decl: FnDecl { inputs: [("self", Generic("Self")), ("other", ResolvedPath { name: "Vec3A", id: Id("0:9222:1579"), args: Some(AngleBracketed { args: [], bindings: [] }), param_names: [] })], output: Some(QualifiedPath { name: "Output", args: AngleBracketed { args: [], bindings: [] }, self_type: Generic("Self"), trait_: ResolvedPath { name: "", id: Id("2:3257:1919"), args: None, param_names: [] } }), c_variadic: false }, generics: Generics { params: [], where_predicates: [] }, header: Header { const_: false, unsafe_: false, async_: false, abi: Rust }, has_body: true }`
			) 
		+ AutoMethods(
			
			///Creates a new rotation quaternion.
			///
			///This should generally not be called manually unless you know what you are doing.
			///Use one of the other constructors instead such as `identity` or `from_axis_angle`.
			///
			///`from_xyzw` is mostly used by unit tests and `serde` deserialization.
			///
			///# Preconditions
			///
			///This function does not check if the input is normalized, it is up to the user to
			///provide normalized input or to normalized the resulting quaternion.
			from_xyzw(f32,f32,f32,f32) -> LuaQuat ,
		
//	
//			///Creates a rotation quaternion from an array.
//			///
//			///# Preconditions
//			///
//			///This function does not check if the input is normalized, it is up to the user to
//			///provide normalized input or to normalized the resulting quaternion.
//			from_array() -> LuaQuat 
//			Error: Unsupported argument `[f32;4]` in type: `Some("Quat")`.,
			
			///Creates a new rotation quaternion from a 4D vector.
			///
			///# Preconditions
			///
			///This function does not check if the input is normalized, it is up to the user to
			///provide normalized input or to normalized the resulting quaternion.
			from_vec4(LuaVec4) -> LuaQuat ,
		
//	
//			///Creates a rotation quaternion from a slice.
//			///
//			///# Preconditions
//			///
//			///This function does not check if the input is normalized, it is up to the user to
//			///provide normalized input or to normalized the resulting quaternion.
//			///
//			///# Panics
//			///
//			///Panics if `slice` length is less than 4.
//			from_slice() -> LuaQuat 
//			Error: Unsupported argument `&[f32]` in type: `Some("Quat")`.,
		
//	
//			///Writes the quaternion to an unaligned slice.
//			///
//			///# Panics
//			///
//			///Panics if `slice` length is less than 4.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [f32]` in type: `Some("Quat")`.,
			
			///Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
			///The axis must be normalized (unit-length).
			///
			///# Panics
			///
			///Will panic if `axis` is not normalized when `glam_assert` is enabled.
			from_axis_angle(LuaVec3,f32) -> LuaQuat ,
			
			///Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
			///
			///`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
			from_scaled_axis(LuaVec3) -> LuaQuat ,
			
			///Creates a quaternion from the `angle` (in radians) around the x axis.
			from_rotation_x(f32) -> LuaQuat ,
			
			///Creates a quaternion from the `angle` (in radians) around the y axis.
			from_rotation_y(f32) -> LuaQuat ,
			
			///Creates a quaternion from the `angle` (in radians) around the z axis.
			from_rotation_z(f32) -> LuaQuat ,
			
			///Creates a quaternion from the given euler rotation sequence and the angles (in radians).
			from_euler(LuaEulerRot,f32,f32,f32) -> LuaQuat ,
		
//	
//			///Creates a quaternion from a 3x3 rotation matrix.
//			from_mat3() -> LuaQuat 
//			Error: Unsupported argument `&LuaMat3` in type: `Some("Quat")`.,
		
//	
//			///Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
//			from_mat4() -> LuaQuat 
//			Error: Unsupported argument `&LuaMat4` in type: `Some("Quat")`.,
			
			///Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
			///plane spanned by the two vectors.  Will rotate at most 180 degrees.
			///
			///The input vectors must be normalized (unit-length).
			///
			///`from_rotation_arc(from, to) * from ≈ to`.
			///
			///For near-singular cases (from≈to and from≈-to) the current implementation
			///is only accurate to about 0.001 (for `f32`).
			///
			///# Panics
			///
			///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
			from_rotation_arc(LuaVec3,LuaVec3) -> LuaQuat ,
			
			///Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
			///that the resulting quaternion will rotate `from` so that it is colinear with `to`.
			///
			///The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
			///degrees.
			///
			///The input vectors must be normalized (unit-length).
			///
			///`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
			///
			///# Panics
			///
			///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
			from_rotation_arc_colinear(LuaVec3,LuaVec3) -> LuaQuat ,
			
			///Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
			///around the z axis. Will rotate at most 180 degrees.
			///
			///The input vectors must be normalized (unit-length).
			///
			///`from_rotation_arc_2d(from, to) * from ≈ to`.
			///
			///For near-singular cases (from≈to and from≈-to) the current implementation
			///is only accurate to about 0.001 (for `f32`).
			///
			///# Panics
			///
			///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
			from_rotation_arc_2d(LuaVec2,LuaVec2) -> LuaQuat ,
		
//	
//			///Returns the rotation axis and angle (in radians) of `self`.
//			to_axis_angle(self) ->  
//			Error: Unsupported return type `(LuaVec3,f32)` in type: `Some("Quat")`.,
			
			///Returns the rotation axis scaled by the rotation in radians.
			to_scaled_axis(self) -> LuaVec3 ,
		
//	
//			///Returns the rotation angles for the given euler rotation sequence.
//			to_euler(self,LuaEulerRot) ->  
//			Error: Unsupported return type `(f32,f32,f32)` in type: `Some("Quat")`.,
		
//	
//			///`[x, y, z, w]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[f32;4]` in type: `Some("Quat")`.,
			
			///Returns the vector part of the quaternion.
			xyz(self) -> LuaVec3 ,
			
			///Returns the quaternion conjugate of `self`. For a unit quaternion the
			///conjugate is also the inverse.
			conjugate(self) -> LuaQuat ,
			
			///Returns the inverse of a normalized quaternion.
			///
			///Typically quaternion inverse returns the conjugate of a normalized quaternion.
			///Because `self` is assumed to already be unit length this method *does not* normalize
			///before returning the conjugate.
			///
			///# Panics
			///
			///Will panic if `self` is not normalized when `glam_assert` is enabled.
			inverse(self) -> LuaQuat ,
			
			///Computes the dot product of `self` and `other`. The dot product is
			///equal to the the cosine of the angle between two quaternion rotations.
			dot(self,LuaQuat) -> f32 ,
			
			///Computes the length of `self`.
			length(self) -> f32 ,
			
			///Computes the squared length of `self`.
			///
			///This is generally faster than `length()` as it avoids a square
			///root operation.
			length_squared(self) -> f32 ,
			
			///Computes `1.0 / length()`.
			///
			///For valid results, `self` must _not_ be of length zero.
			length_recip(self) -> f32 ,
			
			///Returns `self` normalized to length 1.0.
			///
			///For valid results, `self` must _not_ be of length zero.
			///
			///Panics
			///
			///Will panic if `self` is zero length when `glam_assert` is enabled.
			normalize(self) -> LuaQuat ,
			
			///Returns `true` if, and only if, all elements are finite.
			///If any element is either `NaN`, positive or negative infinity, this will return `false`.
			is_finite(self) -> bool ,
			
			is_nan(self) -> bool ,
			
			///Returns whether `self` of length `1.0` or not.
			///
			///Uses a precision threshold of `1e-6`.
			is_normalized(self) -> bool ,
			
			is_near_identity(self) -> bool ,
			
			///Returns the angle (in radians) for the minimal rotation
			///for transforming this quaternion into another.
			///
			///Both quaternions must be normalized.
			///
			///# Panics
			///
			///Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
			angle_between(self,LuaQuat) -> f32 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other`
			///is less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two quaternions contain similar elements. It works
			///best when comparing with a known value. The `max_abs_diff` that should be used used
			///depends on the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(self,LuaQuat,f32) -> bool ,
			
			///Performs a linear interpolation between `self` and `other` based on
			///the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s`
			///is `1.0`, the result will be equal to `other`.
			///
			///# Panics
			///
			///Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
			lerp(self,LuaQuat,f32) -> LuaQuat ,
			
			///Performs a spherical linear interpolation between `self` and `end`
			///based on the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s`
			///is `1.0`, the result will be equal to `end`.
			///
			///# Panics
			///
			///Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
			slerp(self,LuaQuat,f32) -> LuaQuat ,
			
			///Multiplies a quaternion and a 3D vector, returning the rotated vector.
			///
			///# Panics
			///
			///Will panic if `self` is not normalized when `glam_assert` is enabled.
			mul_vec3(self,LuaVec3) -> LuaVec3 ,
			
			///Multiplies two quaternions. If they each represent a rotation, the result will
			///represent the combined rotation.
			///
			///Note that due to floating point rounding the result may not be perfectly normalized.
			///
			///# Panics
			///
			///Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
			mul_quat(self,LuaQuat) -> LuaQuat ,
		
//	
//			///Multiplies a quaternion and a 3D vector, returning the rotated vector.
//			mul_vec3a() ->  
//			Error: Unsupported argument `Vec3A` in type: `Some("Quat")`.,
			
			as_f64(self) -> LuaDQuat ,
		
//	
//			///Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
//			from_affine3() -> LuaQuat 
//			Error: Unsupported argument `crate::Affine3A` in type: `Some("Quat")`.
		) 
},
{
    
	///A quaternion representing an orientation.
	///
	///This quaternion is intended to be of unit length but may denormalize due to
	///floating point "error creep" which can occur when successive quaternion
	///operations are applied.
    glam::quat::DQuat : Reflect:
        UnaryOps(
			Neg self
			) 
		+ BinOps(
			self Add LuaDQuat -> LuaDQuat,
			self Sub LuaDQuat -> LuaDQuat,
			self Div f64 -> LuaDQuat,
			self Mul f64 -> LuaDQuat,
			self Mul LuaDQuat -> LuaDQuat,
			self Mul LuaDVec3 -> LuaDVec3
			) 
		+ AutoMethods(
			
			///Creates a new rotation quaternion.
			///
			///This should generally not be called manually unless you know what you are doing.
			///Use one of the other constructors instead such as `identity` or `from_axis_angle`.
			///
			///`from_xyzw` is mostly used by unit tests and `serde` deserialization.
			///
			///# Preconditions
			///
			///This function does not check if the input is normalized, it is up to the user to
			///provide normalized input or to normalized the resulting quaternion.
			from_xyzw(f64,f64,f64,f64) -> LuaDQuat ,
		
//	
//			///Creates a rotation quaternion from an array.
//			///
//			///# Preconditions
//			///
//			///This function does not check if the input is normalized, it is up to the user to
//			///provide normalized input or to normalized the resulting quaternion.
//			from_array() -> LuaDQuat 
//			Error: Unsupported argument `[f64;4]` in type: `Some("DQuat")`.,
			
			///Creates a new rotation quaternion from a 4D vector.
			///
			///# Preconditions
			///
			///This function does not check if the input is normalized, it is up to the user to
			///provide normalized input or to normalized the resulting quaternion.
			from_vec4(LuaDVec4) -> LuaDQuat ,
		
//	
//			///Creates a rotation quaternion from a slice.
//			///
//			///# Preconditions
//			///
//			///This function does not check if the input is normalized, it is up to the user to
//			///provide normalized input or to normalized the resulting quaternion.
//			///
//			///# Panics
//			///
//			///Panics if `slice` length is less than 4.
//			from_slice() -> LuaDQuat 
//			Error: Unsupported argument `&[f64]` in type: `Some("DQuat")`.,
		
//	
//			///Writes the quaternion to an unaligned slice.
//			///
//			///# Panics
//			///
//			///Panics if `slice` length is less than 4.
//			write_to_slice()  
//			Error: Unsupported argument `&mut [f64]` in type: `Some("DQuat")`.,
			
			///Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
			///The axis must be normalized (unit-length).
			///
			///# Panics
			///
			///Will panic if `axis` is not normalized when `glam_assert` is enabled.
			from_axis_angle(LuaDVec3,f64) -> LuaDQuat ,
			
			///Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
			///
			///`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
			from_scaled_axis(LuaDVec3) -> LuaDQuat ,
			
			///Creates a quaternion from the `angle` (in radians) around the x axis.
			from_rotation_x(f64) -> LuaDQuat ,
			
			///Creates a quaternion from the `angle` (in radians) around the y axis.
			from_rotation_y(f64) -> LuaDQuat ,
			
			///Creates a quaternion from the `angle` (in radians) around the z axis.
			from_rotation_z(f64) -> LuaDQuat ,
			
			///Creates a quaternion from the given euler rotation sequence and the angles (in radians).
			from_euler(LuaEulerRot,f64,f64,f64) -> LuaDQuat ,
		
//	
//			///Creates a quaternion from a 3x3 rotation matrix.
//			from_mat3() -> LuaDQuat 
//			Error: Unsupported argument `&LuaDMat3` in type: `Some("DQuat")`.,
		
//	
//			///Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
//			from_mat4() -> LuaDQuat 
//			Error: Unsupported argument `&LuaDMat4` in type: `Some("DQuat")`.,
			
			///Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
			///plane spanned by the two vectors.  Will rotate at most 180 degrees.
			///
			///The input vectors must be normalized (unit-length).
			///
			///`from_rotation_arc(from, to) * from ≈ to`.
			///
			///For near-singular cases (from≈to and from≈-to) the current implementation
			///is only accurate to about 0.001 (for `f32`).
			///
			///# Panics
			///
			///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
			from_rotation_arc(LuaDVec3,LuaDVec3) -> LuaDQuat ,
			
			///Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
			///that the resulting quaternion will rotate `from` so that it is colinear with `to`.
			///
			///The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
			///degrees.
			///
			///The input vectors must be normalized (unit-length).
			///
			///`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
			///
			///# Panics
			///
			///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
			from_rotation_arc_colinear(LuaDVec3,LuaDVec3) -> LuaDQuat ,
			
			///Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
			///around the z axis. Will rotate at most 180 degrees.
			///
			///The input vectors must be normalized (unit-length).
			///
			///`from_rotation_arc_2d(from, to) * from ≈ to`.
			///
			///For near-singular cases (from≈to and from≈-to) the current implementation
			///is only accurate to about 0.001 (for `f32`).
			///
			///# Panics
			///
			///Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
			from_rotation_arc_2d(LuaDVec2,LuaDVec2) -> LuaDQuat ,
		
//	
//			///Returns the rotation axis and angle (in radians) of `self`.
//			to_axis_angle(self) ->  
//			Error: Unsupported return type `(LuaDVec3,f64)` in type: `Some("DQuat")`.,
			
			///Returns the rotation axis scaled by the rotation in radians.
			to_scaled_axis(self) -> LuaDVec3 ,
		
//	
//			///Returns the rotation angles for the given euler rotation sequence.
//			to_euler(self,LuaEulerRot) ->  
//			Error: Unsupported return type `(f64,f64,f64)` in type: `Some("DQuat")`.,
		
//	
//			///`[x, y, z, w]`
//			to_array(&self) ->  
//			Error: Unsupported return type `[f64;4]` in type: `Some("DQuat")`.,
			
			///Returns the vector part of the quaternion.
			xyz(self) -> LuaDVec3 ,
			
			///Returns the quaternion conjugate of `self`. For a unit quaternion the
			///conjugate is also the inverse.
			conjugate(self) -> LuaDQuat ,
			
			///Returns the inverse of a normalized quaternion.
			///
			///Typically quaternion inverse returns the conjugate of a normalized quaternion.
			///Because `self` is assumed to already be unit length this method *does not* normalize
			///before returning the conjugate.
			///
			///# Panics
			///
			///Will panic if `self` is not normalized when `glam_assert` is enabled.
			inverse(self) -> LuaDQuat ,
			
			///Computes the dot product of `self` and `other`. The dot product is
			///equal to the the cosine of the angle between two quaternion rotations.
			dot(self,LuaDQuat) -> f64 ,
			
			///Computes the length of `self`.
			length(self) -> f64 ,
			
			///Computes the squared length of `self`.
			///
			///This is generally faster than `length()` as it avoids a square
			///root operation.
			length_squared(self) -> f64 ,
			
			///Computes `1.0 / length()`.
			///
			///For valid results, `self` must _not_ be of length zero.
			length_recip(self) -> f64 ,
			
			///Returns `self` normalized to length 1.0.
			///
			///For valid results, `self` must _not_ be of length zero.
			///
			///Panics
			///
			///Will panic if `self` is zero length when `glam_assert` is enabled.
			normalize(self) -> LuaDQuat ,
			
			///Returns `true` if, and only if, all elements are finite.
			///If any element is either `NaN`, positive or negative infinity, this will return `false`.
			is_finite(self) -> bool ,
			
			is_nan(self) -> bool ,
			
			///Returns whether `self` of length `1.0` or not.
			///
			///Uses a precision threshold of `1e-6`.
			is_normalized(self) -> bool ,
			
			is_near_identity(self) -> bool ,
			
			///Returns the angle (in radians) for the minimal rotation
			///for transforming this quaternion into another.
			///
			///Both quaternions must be normalized.
			///
			///# Panics
			///
			///Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
			angle_between(self,LuaDQuat) -> f64 ,
			
			///Returns true if the absolute difference of all elements between `self` and `other`
			///is less than or equal to `max_abs_diff`.
			///
			///This can be used to compare if two quaternions contain similar elements. It works
			///best when comparing with a known value. The `max_abs_diff` that should be used used
			///depends on the values being compared against.
			///
			///For more see
			///[comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
			abs_diff_eq(self,LuaDQuat,f64) -> bool ,
			
			///Performs a linear interpolation between `self` and `other` based on
			///the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s`
			///is `1.0`, the result will be equal to `other`.
			///
			///# Panics
			///
			///Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
			lerp(self,LuaDQuat,f64) -> LuaDQuat ,
			
			///Performs a spherical linear interpolation between `self` and `end`
			///based on the value `s`.
			///
			///When `s` is `0.0`, the result will be equal to `self`.  When `s`
			///is `1.0`, the result will be equal to `end`.
			///
			///# Panics
			///
			///Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
			slerp(self,LuaDQuat,f64) -> LuaDQuat ,
			
			///Multiplies a quaternion and a 3D vector, returning the rotated vector.
			///
			///# Panics
			///
			///Will panic if `self` is not normalized when `glam_assert` is enabled.
			mul_vec3(self,LuaDVec3) -> LuaDVec3 ,
			
			///Multiplies two quaternions. If they each represent a rotation, the result will
			///represent the combined rotation.
			///
			///Note that due to floating point rounding the result may not be perfectly normalized.
			///
			///# Panics
			///
			///Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
			mul_quat(self,LuaDQuat) -> LuaDQuat ,
			
			as_f32(self) -> LuaQuat ,
		
//	
//			///Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
//			from_affine3() -> LuaDQuat 
//			Error: Unsupported argument `crate::DAffine3` in type: `Some("DQuat")`.
		) 
},
{
    
	///Euler rotation sequences.
	///
	///The angles are applied starting from the right.
	///E.g. XYZ will first apply the z-axis rotation.
	///
	///YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
	///
	///The two-axis rotations (e.g. ZYZ) are not fully tested and have to be treated with caution.
    glam::euler::EulerRot : NonReflect(EulerRot):
        UnaryOps(
			
			) 
		+ BinOps(
			
			) 
},
]);
