pub type EntityId = u32;
pub type Generation = i32;

const EMPTY:EntityId = EntityId::MAX;

#[derive(Copy, Clone)]
pub struct Entity {
    id: EntityId,
    gn: Generation
}

pub struct Entities {
    free: Vec<EntityId>,
    gens: Vec<Generation>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            free: Vec::new(),
            gens: Vec::new()
        }
    }
    pub fn spawn(&mut self) -> Entity {
        let id: EntityId;
        let gn: Generation;
        if self.free.is_empty() {
            id = self.gens.len() as EntityId;
            gn = 0;
            self.gens.push(0);
        } else {
            id = self.free.pop().unwrap();
            gn = (-self.gens[id as usize]) & 0x7fffffff;//roll overflow to 0
        }
        Entity {
            id,
            gn
        }
    }
    pub fn is_alive(&self, entity: Entity) -> bool {
        return entity.id < self.gens.len() as EntityId// paranoid "always true"
        && entity.gn == self.gens[entity.id as usize]// not stale = not dead
        && entity.gn >= 0//bullet proof
    }
    pub fn kill(&mut self, entity: Entity) {
        if self.is_alive(entity) {
            self.gens[entity.id as usize] = !self.gens[entity.id as usize];// increment and negate
            self.free.push(entity.id);
        }
    }
}

pub struct Comp<T> {
    dense: Vec<T>, // dense packed components
    sparse: Vec<EntityId>, // EntityId -> dense index 
    map: Vec<usize> // dense index -> EntityId
}

impl<T> Comp<T> {
    fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: Vec::new(),
            map: Vec::new()
        }
    }
    fn exists(&self, entities: &Entities ,entity: Entity) -> bool {
        return entities.is_alive(entity)
            && (entity.id as usize) < self.sparse.len()
            && self.sparse[entity.id as usize] != EMPTY;
    }
    fn insert(&mut self, entities: &Entities, entity: Entity, component: T) {
        if !entities.is_alive(entity) {
            return;
        }
        if entity.id >= self.sparse.len() as EntityId {
            let mut new_len = self.sparse.len().max(2);
            while entity.id as usize >= new_len {
                new_len += new_len >> 1;
            }
            self.sparse.resize(new_len, EMPTY);
        }
        if self.sparse[entity.id as usize] == EMPTY {
            let denseId = self.dense.len() as EntityId;
            self.sparse[entity.id as usize] = denseId;
            self.dense.push(component);
            self.map.push(entity.id as usize);
        } else {
            let denseId = self.sparse[entity.id as usize];
            self.dense[denseId as usize] = component;
        }
    }
    fn remove(&mut self, entities: &Entities, entity: Entity) {
        if !entities.is_alive(entity) {
            return;
        }
        if entity.id < self.sparse.len() as EntityId
            && self.sparse[entity.id as usize] != EMPTY {
            let did = self.sparse[entity.id as usize] as usize;
            self.dense.swap_remove(did);
            self.map.swap_remove(did);
            self.sparse[entity.id as usize] = EMPTY;
            self.sparse[self.map[did]] = did as EntityId;
        }
    }
}