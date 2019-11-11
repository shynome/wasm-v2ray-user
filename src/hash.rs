
use wasm_bindgen::prelude::*;

type Unit8Array = Box<[u8]>;

#[wasm_bindgen]
pub fn generate_new_hashes(id: Unit8Array) -> Unit8Array {
  id
}

pub fn generate_hash(id: Unit8Array, now_sec: i64) -> Unit8Array {
  id
}
