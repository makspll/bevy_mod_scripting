local a

function on_update()

    if a == nil then
        a = test:get_script_component(entity_test)
    end

    print(a.scripts)

end