let entity = Entity.from_raw.call(9999);
// does not throw
let out = entity.eq.call(entity);