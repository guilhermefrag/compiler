use phf::phf_map;

pub const TYPES: [i32; 4] = [13, 18, 24, 3];

pub static MATCH_TYPES: phf::Map<i32, i32> = phf_map! {
  13i32 => 5,
  18i32 => 6,
  24i32 => 8,
  3i32 => 10,
};