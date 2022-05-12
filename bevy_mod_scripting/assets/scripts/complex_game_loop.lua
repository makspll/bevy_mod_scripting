

function on_pre_physics_one(id) 
    print("on_pre_physics_one, Handling:")
    print(string.format("\t-> id: %d",id))
end

function on_pre_physics_two(id)
    print("on_pre_physics_two, Handling:")
    print(string.format("\t-> id: %d",id))
end

function on_post_physics_one(id)
    print("on_post_physics_one, Handling:")
    print(string.format("\t-> id: %d",id))
end 

function on_post_physics_two(id)
    print("on_post_physics_two, Handling:")
    print(string.format("\t-> id: %d",id))
end 

function on_post_update_one(id)
    print("on_post_update_one, Handling:")
    print(string.format("\t-> id: %d",id))
end

function on_post_update_two(id)
    print("on_post_update_two, Handling:")
    print(string.format("\t-> id: %d",id))
end