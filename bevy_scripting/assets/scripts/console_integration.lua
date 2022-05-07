local a = 0

function on_update()

    -- print("hello world") 
    -- print(test()) -- test() is defined in console_integration.rs by the api provider
    -- print(a)


    if a % 100 == 0 then
        print(a)
    end

    a = a + 1
end