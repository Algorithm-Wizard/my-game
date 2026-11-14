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

pub enum EcsState {
    Alive,
    Dead,
    Stale,
    Invalid,
}

pub enum EcsResult {
    Success,
    Dead,
    Stale,
    Invalid,
    Missing,
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
            gn = self.gens[id as usize].wrapping_neg() & 0x7fffffff;//roll overflow to 0
            self.gens[id as usize] = gn;
        }
        Entity {
            id,
            gn
        }
    }
    pub fn is_alive(&self, entity: Entity) -> bool {
        match self.state(entity) {
            EcsState::Alive => true,
            _ => false,
        }
    }
    pub fn state(&self, entity: Entity) -> EcsState {
        let index = entity.id as usize;
        if index >= self.gens.len() || entity.gn < 0 {
            return EcsState::Invalid;
        } else if self.gens[index] < 0 {
            return EcsState::Dead;
        } else if entity.gn != self.gens[index] {
            return EcsState::Stale;
        }
        return  EcsState::Alive;
    }
    pub fn kill(&mut self, entity: Entity) -> EcsResult {
        match self.state(entity) {
            EcsState::Alive => {
                self.gens[entity.id as usize] = !self.gens[entity.id as usize];// increment and negate
                self.free.push(entity.id);
                EcsResult::Success
            }
            EcsState::Dead => EcsResult::Dead,
            EcsState::Invalid => EcsResult::Invalid,
            EcsState::Stale => EcsResult::Stale,
        }
    }
}

pub struct Comp<T> {
    dense: Vec<T>, // dense packed components
    sparse: Vec<EntityId>, // EntityId -> dense index 
    owner: Vec<usize> // dense index -> EntityId
}

impl<T> Comp<T> {
    fn new() -> Self {
        Self {
            dense: Vec::new(),
            sparse: Vec::new(),
            owner: Vec::new()
        }
    }
    fn exists(&self, entities: &Entities ,entity: Entity) -> bool {
        return entities.is_alive(entity)
            && (entity.id as usize) < self.sparse.len()
            && self.sparse[entity.id as usize] != EMPTY;
    }
    fn insert(&mut self, entities: &Entities, entity: Entity, component: T) -> EcsResult {
        match entities.state(entity) {
            EcsState::Alive => {
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
                    self.owner.push(entity.id as usize);
                } else {
                    let denseId = self.sparse[entity.id as usize];
                    self.dense[denseId as usize] = component;
                }
                EcsResult::Success
            }
            EcsState::Dead => EcsResult::Dead,
            EcsState::Invalid => EcsResult::Invalid,
            EcsState::Stale => EcsResult::Stale,
        }
    }
    fn remove(&mut self, entities: &Entities, entity: Entity) -> EcsResult {
        match entities.state(entity) {
            EcsState::Alive =>
            {
                if entity.id >= self.sparse.len() as EntityId
                || self.sparse[entity.id as usize] == EMPTY {
                    return  EcsResult::Missing;
                }
                let did = self.sparse[entity.id as usize] as usize;
                self.dense.swap_remove(did);
                self.owner.swap_remove(did);
                self.sparse[entity.id as usize] = EMPTY;
                if did < self.dense.len() {
                    self.sparse[self.owner[did]] = did as EntityId;
                }
                return EcsResult::Success;
            }
            EcsState::Dead => return EcsResult::Dead,
            EcsState::Stale => return EcsResult::Stale,
            EcsState::Invalid => return EcsResult::Invalid,
        }
    }
}