use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::vec;

pub struct Namer<T> {
    seed: usize,
    name_id_map: HashMap<String, Vec<T>>,
    id_name_map: HashMap<T, Vec<String>>,
}

impl<T> Namer<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(seed: usize) -> Self {
        Self {
            seed,
            name_id_map: Default::default(),
            id_name_map: Default::default(),
        }
    }

    pub fn register_name(&mut self, id: T, name: String) -> Result<(), &'static str> {
        if self.id_name_map.contains_key(&id) {
            return Err("id already used");
        }

        if let Some(ids) = self.name_id_map.get(&name) {
            if ids.len() <= 1 {
                for id in ids {
                    let suffix = self.create_suffix(id);

                    self.id_name_map
                        .insert(id.clone(), vec![name.clone(), suffix]);
                }
            }

            let suffix = self.create_suffix(&id);

            let mut ids = ids.clone();
            ids.push(id.clone());

            self.name_id_map.insert(name.clone(), ids);
            self.id_name_map.insert(id, vec![name.clone(), suffix]);
        } else {
            let ids = vec![id.clone()];

            let name_parts = vec![name.clone()];

            self.name_id_map.insert(name.clone(), ids);
            self.id_name_map.insert(id.clone(), name_parts);
        }

        todo!()
    }

    pub fn get_name(&self, id: &T) -> Option<&Vec<String>> {
        self.id_name_map.get(id)
    }

    fn create_suffix(&self, id: &T) -> String {
        let mut hasher = DefaultHasher::new();

        hasher.write_usize(self.seed);
        id.hash(&mut hasher);

        let hash = hasher.finish();

        format!("{}", radix_fmt::radix_36(hash))
    }
}
