-- 100 operations per call: the cost of one is (this - `empty loop`) / 100
function bench()
    for i = 1, 100 do
    end
end
