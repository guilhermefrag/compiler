fn get_can_be_empty(index: i32) -> bool {
  let can_be_empty: [i32; 15] = [50, 51, 53, 55, 59, 60, 62, 63, 64, 66, 70, 73, 74, 78, 80];
  can_be_empty.contains(&index)
}

pub const GET_CAN_BE_EMPTY: fn(i32) -> bool = get_can_be_empty;