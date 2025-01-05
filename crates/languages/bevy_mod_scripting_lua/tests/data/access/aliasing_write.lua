local entity = Entity.from_raw(9999);
_claim_write_access(entity);

if pcall(function()
    entity:eq(entity)
end)
then
    error("Aliasing access did not panick")
else 
    -- all good
end
