use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Namer<T> {
    seed: usize,
    name_id_map: HashMap<String, Vec<T>>,
    id_name_map: HashMap<T, Vec<String>>,
}

impl<T> Namer<T> {
    pub fn new() -> Self {
        Self {
            seed: Default::default(),
            name_id_map: Default::default(),
            id_name_map: Default::default(),
        }
    }

    pub fn register_name(id: T, name: String) {
        todo!()
    }

    pub fn get_name(id: T) -> String {
        todo!()
    }

    fn create_suffix(id: T) -> String {
        todo!()
    }
}
