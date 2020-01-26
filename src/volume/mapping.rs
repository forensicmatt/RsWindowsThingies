use lru::LruCache;
use serde::Serialize;
use std::collections::HashMap;
use winstructs::ntfs::mft_reference::MftReference;


#[derive(Serialize, Debug)]
pub struct EntryMapping {
    pub name: String,
    pub parent: MftReference,
}


pub struct FolderMapping {
    pub mapping: HashMap<MftReference, EntryMapping>,
    pub cache: LruCache<MftReference, String>
}

impl FolderMapping {
    pub fn new() -> Self {
        let mapping: HashMap<MftReference, EntryMapping> = HashMap::new();
        let cache: LruCache<MftReference, String> = LruCache::new(100);

        FolderMapping {
            mapping,
            cache
        }
    }

    pub fn contains_reference(&self, entry_reference: &MftReference) -> bool {
        self.mapping.contains_key(
            entry_reference
        )
    }

    pub fn remove_mapping(&mut self, entry_reference: MftReference) {
        self.mapping.remove(
            &entry_reference
        );
    }

    pub fn add_mapping(
        &mut self, 
        entry_reference: MftReference, 
        name: String, 
        parent: MftReference
    ) {
        let entry_map = EntryMapping {
            name: name,
            parent: parent
        };

        // If there is a cached entry for this reference, we need to remove it
        // so that it can be recreated with the new mapping.
        self.cache.pop(
            &entry_reference
        );

        self.mapping.insert(
            entry_reference,
            entry_map
        );
    }

    fn enumerate_path_queue(
        &self, 
        lookup_ref: &MftReference, 
        path_queue: &mut Vec<String>
    ) {
        if lookup_ref.entry != 5 {
            match self.mapping.get(&lookup_ref) {
                Some(folder_map) => {
                    self.enumerate_path_queue(
                        &folder_map.parent,
                        path_queue
                    );

                    path_queue.push(folder_map.name.clone());
                },
                None => {
                    path_queue.push("[<unknown>]".to_string());
                }
            }
        } else {
            path_queue.push("[root]".to_string());
        }
    }

    pub fn enumerate_path(
        &mut self, 
        entry: u64, 
        sequence: u16
    ) -> Option<String> {
        let lookup_ref = MftReference {
            entry, sequence
        };
        
        match self.cache.get_mut(&lookup_ref) {
            Some(full_path) => {
                return Some(full_path.clone());
            },
            None => {
                let mut path_queue: Vec<String> = Vec::new();

                self.enumerate_path_queue(
                    &lookup_ref, 
                    &mut path_queue
                );

                let full_path = path_queue.join("/");

                self.cache.put(
                    lookup_ref, 
                    full_path.clone()
                );

                return Some(full_path);
            }
        }
    }
}