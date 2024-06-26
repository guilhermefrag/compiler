use phf::phf_map;

pub const TYPES: [i32; 4] = [13, 18, 24, 3];

pub static MATCH_TYPES: phf::Map<i32, i32> = phf_map! {
  13i32 => 5,
  18i32 => 6,
  24i32 => 8,
  3i32 => 10,
};

pub static MATCH_TYPES_TO_STRING: phf::Map<i32, &str> = phf_map!{
  18i32 => "float",
  13i32 => "integer",
  24i32 => "char",
  3i32 => "string"
};

pub static MATCH_TYPES_ASSIGNMENT: phf::Map<i32, &str> = phf_map!{
  8i32 => "char",
  10i32 => "string",
  6i32 => "float",
  5i32 => "integer",
  18i32 => "float",
  13i32 => "integer",
  24i32 => "char",
  3i32 => "string"
};