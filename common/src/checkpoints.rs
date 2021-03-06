use std::collections::HashMap;

use crypto::hash::Hash256;

pub struct Checkpoint {
    pub height: u64,
    pub hash: Hash256
}

pub struct Checkpoints {
    checkpoints: HashMap<u64, Hash256>
}

impl Checkpoints {
    pub fn new() -> Checkpoints {
        Checkpoints {
            checkpoints: HashMap::new()
        }
    }
    pub fn for_network(_nettype: &str) -> Checkpoints {
        let mut c = Checkpoints::new();
        c
    }
    pub fn add_checkpoint(&mut self, height: u64, hash: &Hash256) -> Result<(), ()> {
        // If we have the checkpoint already with a different hash, return an error
        if self.checkpoints.contains_key(&height) && self.checkpoints[&height] != *hash {
            return Err(());
        }
        Ok(())
    }
    pub fn in_checkpoint_zone(&self, height: &u64) -> bool {
        !self.checkpoints.len() == 0 && height <= self.checkpoints.iter().last().unwrap().0
    }
    pub fn check_block(&self, height: &u64, hash: &Hash256) -> Result<bool, ()> {
        if !self.checkpoints.contains_key(height) {
            return Ok(false);
        } else if self.checkpoints[height] == *hash {
            debug!("CHECKPOINT PASSED FOR HEIGHT {} {}", height, hash);
            return Ok(true);
        }
        warn!("CHECKPOINT FAILED FOR HEIGHT {}. EXPECTED HASH: {}, , FETCHED HASH: {}", height, self.checkpoints[height], hash);
        Err(())
    }
}
