pub struct Bits {
    pub masks: Vec<i32>,
}

impl Bits {
    pub fn new() -> Bits {
        let mut masks: Vec<i32> = vec![0; 33];
        let mut incr: i32 = 2;
        for i in 1..33 {
            masks[i] = incr.wrapping_sub(1);
            incr = incr.wrapping_add(incr);
        }
        return Bits { masks };
    }

    /// Counts the number of set bits (1s) in the binary representation of an `i32`.
    ///
    /// This function uses a bitwise operation to efficiently count the number of bits
    /// that are set to `1` in the provided integer.
    ///
    /// # Arguments
    ///
    /// * `num` - The `i32` whose set bits are to be counted.
    ///
    /// # Example
    ///
    /// ```
    /// use math::bits::Bits;
    ///
    /// let count = Bits::bitcount(15); // 15 in binary is 1111, so count is 4
    /// assert_eq!(count, 4);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the number of set bits (1s) in `num`.
    #[inline(always)]
    pub fn bitcount(num: i32) -> i32 {
        let one: i32 = num - ((num >> 1) & 0x55555555);
        let two: i32 = (one & 0x33333333) + ((one >> 2) & 0x33333333);
        return (((two + (two >> 4)) & 0xf0f0f0f) * 0x1010101) >> 24;
    }

    /// Sets a range of bits to `1` within an `i32`.
    ///
    /// This function sets all the bits in the range `[start, end]` to `1` within the
    /// provided integer `num`. The range is inclusive on both ends.
    ///
    /// # Arguments
    ///
    /// * `num` - The `i32` whose bits are to be modified.
    /// * `start` - The starting position of the bit range (0-based).
    /// * `end` - The ending position of the bit range (0-based).
    ///
    /// # Example
    ///
    /// ```
    /// use math::bits::Bits;
    ///
    /// let bits = Bits::new();
    /// let result = bits.setbit_range(0, 1, 3); // Sets bits 1 to 3 in 0, resulting in 14 (1110 in binary)
    /// assert_eq!(result, 14);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the modified integer with the bits in the specified range set to `1`.
    #[inline(always)]
    pub fn setbit_range(&self, num: i32, start: i32, end: i32) -> i32 {
        return num | (self.masks[(end - start + 1) as usize] << start);
    }

    /// Sets a range of bits to the value of the provided `i32` within a number.
    ///
    /// This function sets the bits in the range `[start, end]` to match the bits of
    /// `value`. If `value` exceeds the maximum value that can fit in the range, it is
    /// capped to the maximum allowable value.
    ///
    /// # Arguments
    ///
    /// * `num` - The `i32` whose bits are to be modified.
    /// * `value` - The value to assign to the bit range.
    /// * `start` - The starting position of the bit range (0-based).
    /// * `end` - The ending position of the bit range (0-based).
    ///
    /// # Example
    ///
    /// ```
    /// use math::bits::Bits;
    ///
    /// let bits = Bits::new();
    /// let result = bits.setbit_range_toint(0, 3, 1, 3); // Sets bits 1 to 3 to match 3, resulting in 6
    /// assert_eq!(result, 6);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the modified integer with the bit range set to the value of `value`.
    #[inline(always)]
    pub fn setbit_range_toint(&self, num: i32, value: i32, start: i32, end: i32) -> i32 {
        let cleared: i32 = self.clearbit_range(num, start, end);
        let max: i32 = self.masks[(end - start + 1) as usize];
        let assign: i32 = if value > max { max } else { value };
        return cleared | (assign << start);
    }

    /// Clears (sets to 0) a range of bits within an `i32`.
    ///
    /// This function sets all the bits in the range `[start, end]` to `0` within the
    /// provided integer `num`. The range is inclusive on both ends.
    ///
    /// # Arguments
    ///
    /// * `num` - The integer whose bits are to be cleared.
    /// * `start` - The starting position of the bit range (0-based).
    /// * `end` - The ending position of the bit range (0-based).
    ///
    /// # Example
    ///
    /// ```
    /// use math::bits::Bits;
    ///
    /// let bits = Bits::new();
    /// let result = bits.clearbit_range(15, 1, 3); // Clears bits 1 to 3 in 15, resulting in 1
    /// assert_eq!(result, 1);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the modified integer with the bits in the specified range cleared (set to 0).
    #[inline(always)]
    pub fn clearbit_range(&self, num: i32, start: i32, end: i32) -> i32 {
        return num & !(self.masks[(end - start + 1) as usize] << start);
    }
}
