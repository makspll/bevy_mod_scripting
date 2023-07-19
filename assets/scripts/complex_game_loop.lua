

function on_pre_physics(id) 
    print("on_pre_physics, Handling:")
    print(string.format("\t-> id: %d",id))
end

function on_post_physics(id)
    print("on_post_physics, Handling:")
    print(string.format("\t-> id: %d",id))
end 


function on_post_update(id)
    print("on_post_update, Handling:")
    print(string.format("\t-> id: %d",id))
end
