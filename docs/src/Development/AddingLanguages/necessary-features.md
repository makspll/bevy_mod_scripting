<div class="warning">
    This section needs work and is not fully accurate as is.
</div>

# Necessary Features

In order for a language to be called "implemented" in BMS, it needs to support the following features:

- Every script function which is registered on a type's namespace must:
    - Be callable on a `ReflectReference` representing object of that type in the script
    ```lua
    local my_reference = ...
    my_reference:my_Registered_function()
    ```
    - If it's static it must be callable from a global proxy object for that type, i.e.
    ```lua
    MyType.my_static_function()
    ```
- `ReflectReferences` must support a set of basic features:
    - Access to fields via reflection i.e.:
    ```lua
    local my_reference = ...
    my_reference.my_field = 5
    print(my_reference.my_field)
    ```
    - Basic operators and standard operations are overloaded with the appropriate standard dynamic function registered:
        - Addition: dispatches to the `add` binary function on the type
        - Multiplication: dispatches to the `mul` binary function on the type
        - Division: dispatches to the `div` binary function on the type
        - Subtraction: dispatches to the `sub` binary function on the type
        - Modulo: dispatches to the `rem` binary function on the type
        - Negation: dispatches to the `neg` unary function on the type
        - Exponentiation: dispatches to the `pow` binary function on the type
        - Equality: dispatches to the `eq` binary function on the type
        - Less than: dispatches to the `lt` binary function on the type
        - Length: calls the `len` method on `ReflectReference` or on the table if the value is one.
        - Iteration: dispatches to the `iter` method on `ReflectReference` which returns an iterator function, this can be repeatedly called until it returns `ScriptValue::Unit` to signal the end of the iteration.
        - Print: calls the `display` method on `ReflectReference` or on the table if the value is one.
        - Debug print: calls the `debug` method on `ReflectReference` or on the table if the value is one.
- Script handlers, loaders etc. must be implemented such that the `ThreadWorldContainer` is set for every interaction with script contexts, or anywhere else it might be needed.
    
