---
applyTo: **/.rhai
---

# Convert lua to rhai script
Convert the current test to a rhai test. below are examples and instructions on the conversions necessary:

## Dynamic function calls
Functions which are not native to rhai MUST be explicitly called using the below syntax:

```rhai
type.function.call(arguments...)
```

native rhai functions MUST NOT be converted using the syntax above.

Below is a list of some of the native functions their argument names and some constants available in rhai:

```csv
function,arguments
is_odd,
is_even,
min,"a, b"
max,"a, b"
to_float,
to_decimal,
abs,value
sign,value
is_zero,
sin,angle
cos,angle
tan,angle
sinh,angle
cosh,angle
tanh,angle
hypot,"x, y"
asin,value
acos,value
atan,value
atan,"x, y"
asinh,value
acosh,value
atanh,value
sqrt,value
exp,value
ln,value
log,value
floor,
ceiling,
round,
round,decimal_points
int,
fraction,
round_up,decimal_points
round_down,decimal_points
round_half_up,decimal_points
round_half_down,decimal_points
to_int,
to_decimal,
to_float,
to_degrees,
to_radians,
is_nan,
is_finite,
is_infinite,
parse_int,"string, [radix]"
parse_float,string
parse_decimal,string
to_binary,value
to_octal,value
to_hex,value
PI,
E,
```

## Operators
Operators are different in lua and rhai, below is a list of operators supported:

```
Operators	Assignment operators	Supported types
(see standard types)
+,	+=	

    INT
    FLOAT (if not no_float)
    Decimal (requires decimal)
    char
    string

-, *, /, %, **,	-=, *=, /=, %=, **=	

    INT
    FLOAT (if not no_float)
    Decimal (requires decimal)

<<, >>	<<=, >>=	

    INT

&, |, ^	&=, |=, ^=	

    INT (bit-wise)
    bool (non-short-circuiting)

&&, ||		

    bool (short-circuits)

==, !=		

    INT
    FLOAT (if not no_float)
    Decimal (requires decimal)
    bool
    char
    string
    BLOB
    numeric range
    ()

>, >=, <, <=		

    INT
    FLOAT (if not no_float)
    Decimal (requires decimal)
    char
    string
    ()
```

## Function syntax
Functions in rhai look like this:

```rhai
fn function_name(arg1, arg2) {
    return value;
}
```

## Semicolons
Every statement must end in a semicolon


Below is a new section on Rhai strings that you can add to the prompt:

## Rhai Strings

Rhai supports different string types such as raw strings (enclosed by matching `#` and double-quotes), multi-line literal strings (enclosed by backticks to preserve exact formatting), and strings with interpolation (using `${…}` inside multi-line literals). These variants allow you to easily include complex content like newlines, quotes, and even embedded expressions while keeping the original formatting. Here are three examples:

````rhai
// Raw string example:
let raw_str = #"Hello, raw string! \n No escape sequences here."#;
````

````rhai
// Multi-line literal string example:
let multi_line = `
This is a multi-line literal string,
which preserves whitespaces, newlines, and "quotes" exactly.
`;
````

````rhai
// String interpolation example:
let value = 42;
let interpolated = `The answer is ${value}, which is computed dynamically.`;
````

## Null Checks
null checks can be performed by checking `type_of(value) == "()"`

## Examples
Below is an example lua test and its equivalent rhai script:

### Lua
```lua
local entity_a = world.spawn()
local entity_b = world.spawn()
local entity_c = world.spawn()
local entity_d = world._get_entity_with_test_component("CompWithFromWorldAndComponentData")

local component_with = world.get_type_by_name("CompWithFromWorldAndComponentData")
local component_without = world.get_type_by_name("CompWithDefaultAndComponentData")

world.add_default_component(entity_a, component_with)
world.add_default_component(entity_b, component_with)
world.add_default_component(entity_c, component_with)

world.add_default_component(entity_b, component_without)

local found_entities = {}
for i,result in pairs(world.query():component(component_with):without(component_without):build()) do
    table.insert(found_entities, result:entity())
end

assert(#found_entities == 3, "Expected 3 entities, got " .. #found_entities)

expected_entities = {
    entity_c,
    entity_d,
    entity_a,
}

for i, entity in ipairs(found_entities) do
    assert(entity:index() == expected_entities[i]:index(), "Expected entity " .. expected_entities[i]:index() .. " but got " .. entity:index())
end
```

### Rhai
```rhai
let entity_a = world.spawn_.call();
let entity_b = world.spawn_.call();
let entity_c = world.spawn_.call();
let entity_d = world._get_entity_with_test_component.call("CompWithFromWorldAndComponentData");

let component_with = world.get_type_by_name.call("CompWithFromWorldAndComponentData");
let component_without = world.get_type_by_name.call("CompWithDefaultAndComponentData");

world.add_default_component.call(entity_a, component_with);
world.add_default_component.call(entity_b, component_with);
world.add_default_component.call(entity_c, component_with);

world.add_default_component.call(entity_b, component_without);

let found_entities = [];
for (result, i) in world.query.call().component.call(component_with).without.call(component_without).build.call() {
    found_entities.push(result.entity.call());
}

assert(found_entities.len == 3, "Expected 3 entities, got " + found_entities.len);

let expected_entities = [
    entity_d,
    entity_a,
    entity_c,
];

for (entity, i) in found_entities {
    assert(entity.index.call() == expected_entities[i].index.call(), "Expected entity " + expected_entities[i].index.call() + " but got " + entity.index.call());
}
```