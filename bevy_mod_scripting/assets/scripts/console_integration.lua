local a = 0

function on_update()

    if a % 100 == 0 then
        -- print_to_console()() is defined in console_integration.rs
        -- by the api provider
        print_to_console(string.format("%d, entity_bits:%d", a,entity))
    end

    a = a + 1
end