//! Virtual machine flags.

bitflags::bitflags! {
  #[derive(Default)]
  pub struct VmKey: u16 {
    const KEY_0 = 0b0000000000000001;
    const KEY_1 = 0b0000000000000010;
    const KEY_2 = 0b0000000000000100;
    const KEY_3 = 0b0000000000001000;
    const KEY_4 = 0b0000000000010000;
    const KEY_5 = 0b0000000000100000;
    const KEY_6 = 0b0000000001000000;
    const KEY_7 = 0b0000000010000000;
    const KEY_8 = 0b0000000100000000;
    const KEY_9 = 0b0000001000000000;
    const KEY_A = 0b0000010000000000;
    const KEY_B = 0b0000100000000000;
    const KEY_C = 0b0001000000000000;
    const KEY_D = 0b0010000000000000;
    const KEY_E = 0b0100000000000000;
    const KEY_F = 0b1000000000000000;
  }
}

impl VmKey {
  /// Gets the list of activated keys in the given key mask.
  pub fn to_vec(self) -> Vec<u8> {
    let mut keys = Vec::new();

    if self.contains(VmKey::KEY_0) {
      keys.push(VmKey::KEY_0.into());
    }

    if self.contains(VmKey::KEY_1) {
      keys.push(VmKey::KEY_1.into());
    }

    if self.contains(VmKey::KEY_2) {
      keys.push(VmKey::KEY_2.into());
    }

    if self.contains(VmKey::KEY_3) {
      keys.push(VmKey::KEY_3.into());
    }

    if self.contains(VmKey::KEY_4) {
      keys.push(VmKey::KEY_4.into());
    }

    if self.contains(VmKey::KEY_5) {
      keys.push(VmKey::KEY_5.into());
    }

    if self.contains(VmKey::KEY_6) {
      keys.push(VmKey::KEY_6.into());
    }

    if self.contains(VmKey::KEY_7) {
      keys.push(VmKey::KEY_7.into());
    }

    if self.contains(VmKey::KEY_8) {
      keys.push(VmKey::KEY_8.into());
    }

    if self.contains(VmKey::KEY_9) {
      keys.push(VmKey::KEY_9.into());
    }

    if self.contains(VmKey::KEY_A) {
      keys.push(VmKey::KEY_A.into());
    }

    if self.contains(VmKey::KEY_B) {
      keys.push(VmKey::KEY_B.into());
    }

    if self.contains(VmKey::KEY_C) {
      keys.push(VmKey::KEY_C.into());
    }

    if self.contains(VmKey::KEY_D) {
      keys.push(VmKey::KEY_D.into());
    }

    if self.contains(VmKey::KEY_E) {
      keys.push(VmKey::KEY_E.into());
    }

    if self.contains(VmKey::KEY_F) {
      keys.push(VmKey::KEY_F.into());
    }

    keys
  }
}

impl From<u8> for VmKey {
  fn from(key: u8) -> Self {
    match key {
      0 => VmKey::KEY_0,
      1 => VmKey::KEY_1,
      2 => VmKey::KEY_2,
      3 => VmKey::KEY_3,
      4 => VmKey::KEY_4,
      5 => VmKey::KEY_5,
      6 => VmKey::KEY_6,
      7 => VmKey::KEY_7,
      8 => VmKey::KEY_8,
      9 => VmKey::KEY_9,
      10 => VmKey::KEY_A,
      11 => VmKey::KEY_B,
      12 => VmKey::KEY_C,
      13 => VmKey::KEY_D,
      14 => VmKey::KEY_E,
      15 => VmKey::KEY_F,
      _ => VmKey::empty(),
    }
  }
}

impl From<VmKey> for u8 {
  fn from(key: VmKey) -> Self {
    match key {
      VmKey::KEY_0 => 0,
      VmKey::KEY_1 => 1,
      VmKey::KEY_2 => 2,
      VmKey::KEY_3 => 3,
      VmKey::KEY_4 => 4,
      VmKey::KEY_5 => 5,
      VmKey::KEY_6 => 6,
      VmKey::KEY_7 => 7,
      VmKey::KEY_8 => 8,
      VmKey::KEY_9 => 9,
      VmKey::KEY_A => 10,
      VmKey::KEY_B => 11,
      VmKey::KEY_C => 12,
      VmKey::KEY_D => 13,
      VmKey::KEY_E => 14,
      VmKey::KEY_F => 15,
      _ => 0xff,
    }
  }
}
