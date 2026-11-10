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
        let mut id: EntityId;
        let mut gn: Generation;
        if self.free.is_empty() {
            id = self.gens.len() as EntityId;
            gn = 0;
            self.gens.push(0);
        } else {
            id = self.free.pop().unwrap();
            gn = -self.gens[id as usize];
        }
        Entity {
            id,
            gn
        }
    }
    pub fn is_alive(&mut self, entity: Entity) -> bool {
        return entity.id < self.gens.len() as EntityId// paranoid "always true"
        && entity.gn == self.gens[entity.id as usize]// not stale = not dead
        && entity.gn >= 0//bullet proof
    }
    pub fn kill(&mut self, entity: Entity) {
        if self.is_alive(entity) {
            self.gens[entity.id as usize] = -1 - self.gens[entity.id as usize];
            self.free.push(entity.id);
        }
    }
}

pub struct Comp<T> {
    dense: Vec<T>, // dense packed components
    sparse: Vec<EntityId>, // EntityId -> dense index 
    map: Vec<u32> // dense index -> EntityId
}

impl Comp<T> {
    fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: Vec::new(),
            map: Vec::new()
        }
    }
    fn exists(&self, entity: Entity) -> bool {
        return entity.id < self.sparse.len() as EntityId
            && self.sparse[entity.id as usize] != EMPTY;
    }
    fn insert(&mut self, entity: Entity, component: T) {
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
            self.map.push(entity.id);
        } else {
            let denseId = self.sparse[entity.id as usize];
            self.dense[denseId as usize] = component;
        }
    }
}