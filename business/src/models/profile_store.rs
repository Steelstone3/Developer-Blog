use std::{collections::HashMap, sync::atomic::AtomicUsize};

use super::id_profile::IdProfile;

#[derive(Default)]
pub struct ProfileStore {
    pub id: AtomicUsize,
    pub store: HashMap<usize, IdProfile>,
}

impl ProfileStore {
    pub fn new_from_hashmap(store: HashMap<usize, IdProfile>) -> Self {
        let id = AtomicUsize::new(store.keys().max().map(|v| v + 1).unwrap_or(0));

        Self { id, store }
    }
}
