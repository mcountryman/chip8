//! Virtual machine sprite definitions.

const SPRITE_0: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
const SPRITE_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
const SPRITE_3: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_4: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_5: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_6: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_7: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_8: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_9: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_A: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_B: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_C: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_D: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_E: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];
const SPRITE_F: [u8; 5] = [0xf0, 0x90, 0x90, 0x90, 0xf0];

/// Copies sprites to supplied slice.
pub fn copy_to(to: &mut [u8]) {
  debug_assert!(to.len() >= 5 * 16);

  to[0..5].copy_from_slice(&SPRITE_0);
  to[5..10].copy_from_slice(&SPRITE_1);
  to[10..15].copy_from_slice(&SPRITE_2);
  to[15..20].copy_from_slice(&SPRITE_3);
  to[20..25].copy_from_slice(&SPRITE_4);
  to[25..30].copy_from_slice(&SPRITE_5);
  to[30..35].copy_from_slice(&SPRITE_6);
  to[35..40].copy_from_slice(&SPRITE_7);
  to[40..45].copy_from_slice(&SPRITE_8);
  to[45..50].copy_from_slice(&SPRITE_9);
  to[50..55].copy_from_slice(&SPRITE_A);
  to[55..60].copy_from_slice(&SPRITE_B);
  to[60..65].copy_from_slice(&SPRITE_C);
  to[65..70].copy_from_slice(&SPRITE_D);
  to[70..75].copy_from_slice(&SPRITE_E);
  to[75..80].copy_from_slice(&SPRITE_F);
}
