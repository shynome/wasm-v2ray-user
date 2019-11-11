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

#[wasm_bindgen]
pub fn next_uuid(id: Uint8Array) -> Uint8Array {
  id
  // let mut hash = md5::Context::new();
  // let _id = &*id;
  // hash.consume(id);
  // hash.consume(B1);
  // let mut new_id: md5::Digest;
  // loop {
  //   hash.consume(B2);
  //   new_id = hash.compute();
  //   if (false) {
  //     break;
  //   }
  //   hash.consume(B2);
  // }
  // let b = [
  //   new_id[0], new_id[1], new_id[2], new_id[3], new_id[4], new_id[5], new_id[6], new_id[7],
  //   new_id[8], new_id[9], new_id[10], new_id[11], new_id[12], new_id[13], new_id[14], new_id[15],
  // ];
  // Box::new(b)
}
