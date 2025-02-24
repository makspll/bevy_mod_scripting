local entity = Entity.from_raw(9999);
-- does not throw
entity:eq(entity);