/// Rook test:
/// 
/// * rook_standart_moves:                          successful
/// * rook_corner_moves:                            successful
/// * rook_unable_to_move_when_fully_blocked:       successful
/// * rook_blocked_from_three_sides_right_open:     successful
/// * rook_blocked_from_three_sides_left_open:      successful
/// * rook_blocked_from_three_sides_top_open:       successful
/// * rook_blocked_from_three_sides_bottom_open:    successful
/// * rook_blocked_from_two_sides_top_bottom_open:  successful
/// * rook_blocked_from_two_sides_right_left_open:  successful
/// * rook_blocked_from_two_sides_top_right_open:   successful
/// * rook_blocked_from_two_sides_top_left_open:    successful
/// * rook_blocked_from_two_sides_bottom_right_open:successful
/// * rook_blocked_from_two_sides_bottom_left_open: successful
/// * rook_on_right_blocked:                        successful
/// * rook_on_left_blocked:                         successful
/// * rook_on_top_blocked:                          successful
/// * rook_on_bottom_blocked:                       successful
/// * rook_can_not_take_through_enemy_piece:        successful
/// 
mod rook;

mod pawn;
mod knight;
mod bishop;
mod king;