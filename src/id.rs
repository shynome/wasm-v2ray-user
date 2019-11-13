use md5;
use wasm_bindgen::prelude::*;

type Uint8Array = Box<[u8]>;

// c48619fe-8f02-49e0-b9e9-edf763e17e21
const BN: [u8; 16] = [
  196, 134, 25, 254, 143, 2, 73, 224, 185, 233, 237, 247, 99, 225, 126, 33,
];
#[wasm_bindgen]
pub fn new_uuid(id: Uint8Array) -> Uint8Array {
  let mut hash = md5::Context::new();
  hash.consume(BN);
  hash.consume(id);
  let digest = hash.compute();
  let b = [
    digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7],
    digest[8], digest[9], digest[10], digest[11], digest[12], digest[13], digest[14], digest[15],
  ];
  Box::new(b)
}

// "16167dc8-16b6-4e6d-b8bb-65dd68113a81"
const B1: [u8; 16] = [
  22, 22, 125, 200, 22, 182, 78, 109, 184, 187, 101, 221, 104, 17, 58, 129,
];
// 533eff8a-4113-4b10-b5ce-0f5d76b98cd2
const B2: [u8; 16] = [
  83u8, 62, 255, 138, 65, 19, 75, 16, 181, 206, 15, 93, 118, 185, 140, 210,
];
const BEMPTY: &'static [u8] = &[];

fn eq_two_hash(a: &[u8], b: &[u8]) -> bool {
  true
    && a[0] == b[0]
    && a[1] == b[1]
    && a[2] == b[2]
    && a[3] == b[3]
    && a[4] == b[4]
    && a[5] == b[5]
    && a[6] == b[6]
    && a[7] == b[7]
    && a[8] == b[8]
    && a[9] == b[9]
    && a[10] == b[10]
    && a[11] == b[11]
    && a[12] == b[12]
    && a[13] == b[13]
    && a[14] == b[14]
    && a[15] == b[15]
}

#[wasm_bindgen]
pub fn next_uuid(_id: Uint8Array) -> Uint8Array {
  let id = &*_id;
  let mut buf = [BEMPTY, id, &B1].concat();
  let mut new_id: md5::Digest;
  loop {
    new_id = md5::compute(&buf);
    if eq_two_hash(&new_id[..], id) == false {
      break;
    }
    buf = [BEMPTY, &buf, &B2].concat();
  }
  let b = [
    new_id[0], new_id[1], new_id[2], new_id[3], new_id[4], new_id[5], new_id[6], new_id[7],
    new_id[8], new_id[9], new_id[10], new_id[11], new_id[12], new_id[13], new_id[14], new_id[15],
  ];
  Box::new(b)
}
