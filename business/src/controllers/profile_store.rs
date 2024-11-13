use crate::models::{
    id_profile::IdProfile, pagination::Pagination, profile::Profile, profile_store::ProfileStore,
    update_profile::UpdateProfile,
};
use std::sync::atomic::Ordering;

impl ProfileStore {
    pub fn add_profile(&mut self, profile: Profile) -> IdProfile {
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let new_profile = IdProfile::new(id, profile);
        self.store.insert(id, new_profile.clone());

        new_profile
    }

    pub fn get_profile(&self, id: usize) -> Option<&IdProfile> {
        self.store.get(&id)
    }

    pub fn get_profiles(&self, pagination: Pagination) -> Vec<IdProfile> {
        self.store
            .values()
            .skip(pagination.offset.unwrap_or(usize::MIN))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn update_profile(&mut self, id: &usize, profile: UpdateProfile) -> Option<&IdProfile> {
        if let Some(item) = self.store.get_mut(id) {
            if let Some(name) = profile.name {
                item.profile.name = name;
            }
            if let Some(role) = profile.role {
                item.profile.role = role;
            }
            if let Some(employer) = profile.employer {
                item.profile.employer = employer;
            }
            if let Some(description) = profile.description {
                item.profile.description = description;
            }

            Some(item)
        } else {
            None
        }
    }

    pub fn remove_profile(&mut self, id: usize) -> Option<IdProfile> {
        self.store.remove(&id)
    }
}
