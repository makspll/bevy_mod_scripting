
local comp

function on_update()
    if comp == nil then
        comp = world:get_component(entity,"MyComponent")

        print(string.format("%s",comp))


        comp.vec2 = comp.vec2 + comp.vec2
        comp.uvec2 = comp.uvec2 + comp.uvec2
        comp.usize = comp.vec2:min_element()
        comp.f32 = comp.f32 + comp.f32 + comp.vec2:min_element()
        comp.vec2 = Vec2.new(2,1)
        comp.vec4 = Vec4.splat(3)
        comp.quat = Quat.from_xyzw(3,2,1,4)
        comp.dquat = comp.dquat * 2
        a = Mat3.from_cols(Vec3.new(1,0,0),Vec3.new(0,1,0),Vec3.new(0,0,-1))


        comp.mat3[1][1] = 42

        print(string.format("%s", comp))
    end

end