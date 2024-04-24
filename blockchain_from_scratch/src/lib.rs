
use std::{
    collections::hash_map::DefaultHasher,
    hash::{
        Hash,Hasher
    }
};
mod c1_state_machine;

fn hash<T: Hash>(t: &T) -> u64 {
	let mut s = DefaultHasher::new();
	t.hash(&mut s);
	s.finish()
}