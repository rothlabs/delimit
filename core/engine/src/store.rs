use super::*;

pub(crate) struct Entry<T> {
    entry: Status<T>,
    generation: u32,
}

pub struct Key {
    pub index: usize,
    pub generation: u32,
}

enum Status<T> {
    Free { next_free: usize },
    Occupied { value: T },
}

impl<T> Default for Store<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            free_head: 0,
            len: 0,
        }
    }
}

impl<T> Store<T> {
    pub fn insert(&mut self, value: T) -> Key {
        let key = if let Some(Entry { entry, generation }) = self.data.get_mut(self.free_head) {
            // Update
            if let Status::Free { next_free } = entry {
                let key = Key {
                    index: self.free_head,
                    generation: *generation,
                };
                self.free_head = *next_free;
                *entry = Status::Occupied { value };
                key
            } else {
                // We have found an occupied entry, what?!
                panic!("corrupt free list");
            }
        } else {
            // Insert
            let generation = 0;
            let key = Key {
                index: self.data.len(),
                generation,
            };
            let entry = Status::Occupied { value };
            let gen_entry = Entry { entry, generation };
            self.data.push(gen_entry);
            self.free_head = key.index + 1;
            key
        };
        self.len += 1;
        key
    }

    pub fn get(&self, key: &Key) -> Option<&T> {
        let Entry { entry, generation } = &self.data[key.index];
        if let Status::Occupied { value } = entry {
            if *generation == key.generation {
                return Some(value);
            }
        }
        None
    }

    pub fn remove(&mut self, key: &Key) {
        let Entry { entry, generation } = &mut self.data[key.index];
        if let Status::Occupied { .. } = entry {
            if *generation != key.generation {
                // Trying to remove an older generation
                return;
            }
            *generation += 1;
            *entry = Status::Free {
                next_free: self.free_head,
            };
            self.free_head = key.index;
            self.len -= 1;
        } else {
            // If we get there it mean's that the user is trying to remove an already
            // removed key, just do nothing.
        }
    }
}