#[derive(Debug)]
pub struct Key {
    pub index: u32,
    pub generation: u32,
}

impl Clone for Key {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Key {}
 
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.generation == other.generation
    }
}

impl Eq for Key {}
 
impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.generation.hash(state);
    }
}

enum Slot<T> {
    Occupied { value: T, generation: u32 },
    Free { next_free: Option<u32>, generation: u32}
}

pub struct SlotMap<T> {
    slots: Vec<Slot<T>>,
    free_head: Option<u32>,
    len: u32
}

impl<T> SlotMap<T> {
    pub fn default() -> Self {
        Self {
            slots: Vec::new(),
            free_head: None,
            len: 0 as u32,
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, value: T) -> Key {
        self.len += 1;

        match self.free_head {
            Some(index) => {
                let slot = &mut self.slots[index as usize];

                match slot {
                    Slot::Free { next_free, generation } => {
                        let generation = *generation + 1;
                        self.free_head = *next_free;

                        *slot = Slot::Occupied { value, generation };

                        Key {
                            index,
                            generation
                        }
                    },
                    Slot::Occupied { .. } => {
                        panic!("free_head is actually occupied - this isn't supposed to happen.");
                    }
                }
            },
            None => {
                let index = self.slots.len() as u32; // using slots.len() because len has already
                                                     // been incremented
                let generation = 0; // New slot
                
                self.slots.push(Slot::Occupied { value, generation });

                Key {
                    index,
                    generation
                }
            }
        }
    }

    pub fn remove(&mut self, key: Key) -> Option<T> {
        let slot = &mut self.slots[key.index as usize];

        match slot {
            Slot::Occupied { value, generation } if *generation == key.generation => {
                let new_generation = *generation + 1;
                let old_free_head = self.free_head;

                let old = std::mem::replace(
                    slot,
                    Slot::Free { next_free: old_free_head, generation: new_generation },
                );

                self.free_head = Some(key.index);
                self.len -= 1;

                match old {
                    Slot::Occupied { value, .. } => Some(value),
                    Slot::Free { .. } => unreachable!(), // This scenario shouldn't happen if the
                                                         // code is correct
                }
            },
           _ => None, 
        }
    }

    pub fn get(&self, key: Key) -> Option<&T> {
        let slot = &self.slots[key.index as usize];

        match slot {
            Slot::Occupied { generation, value } if *generation == key.generation => {
                Some(value)
            },
            _ => {
                None
            }
        }
    }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        let slot = &mut self.slots[key.index as usize];

        match slot {
            Slot::Occupied { generation, value } if *generation == key.generation => {
                Some(value)
            },
            _ => {
                None
            }
        }
    }
}
