pub struct Randomizer {
    state: u64,
}

impl Randomizer {
    /// Create a new randomizer from a seed.
    pub fn new(seed: u64) -> Randomizer {
        Randomizer {
            state: seed.wrapping_add(0xDEAD_BEEF_DEAD_BEEF),
        }
    }

    /// Read a byte from the randomizer.
    #[allow(clippy::cast_possible_truncation)]
    pub fn read_u8(&mut self) -> u8 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        (self
            .state
            .wrapping_mul(1_152_921_504_735_157_271)
            .rotate_right(2)
            ^ 0xFAB0_0105_C0DE) as u8
    }

    /// Write a byte into the randomizer.
    ///
    /// This is used for collecting entropy to the randomizer.
    pub fn write_u8(&mut self, b: u8) {
        self.state ^= u64::try_from(b).unwrap();
        self.read_u8();
    }
}
