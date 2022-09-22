local my_routine;

function on_update()

    if my_routine == nil then
        my_routine = coroutine.create(function()
            local starttime = os.time()
            local endtime = starttime + 5
            while os.time() < endtime do
                print(os.date("waiting %H:%M:%S", os.time()))
                coroutine.yield()
            end

            print(os.date("finished! %H:%M:%S", os.time()))
        end)
    else
        if coroutine.status(my_routine) ~= "dead" then
            coroutine.resume(my_routine)
        else
            print("Couroutine has finished, no longer running")
        end
    end

end
