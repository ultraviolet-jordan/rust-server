#[derive(Clone)]
pub struct Isaac {
    rsl: Vec<i32>,
    mem: Vec<i32>,
    count: usize,
    a: i32,
    b: i32,
    c: i32,
}

impl Isaac {
    /// Creates a new `Isaac` instance and initializes it with a provided seed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use io::Isaac;
    ///
    /// let isaac = Isaac::new(vec![0, 0, 0, 0]);
    /// ```
    pub fn new(seed: Vec<i32>) -> Isaac {
        let mut isaac: Isaac = Isaac {
            rsl: vec![0; 256],
            mem: vec![0; 256],
            count: 0,
            a: 0,
            b: 0,
            c: 0,
        };
        isaac.rsl[..seed.len()].copy_from_slice(&seed);
        isaac.init();
        return isaac;
    }

    /// Returns the next random number in the sequence.
    ///
    /// The `next` method generates a new random number by checking the `count`
    /// field. If the `count` is zero, it calls the `isaac` function to update
    /// the state of the generator and resets the `count` to 255. If the `count`
    /// is non-zero, it simply decreases the count and returns the next number
    /// from the `rsl` vector.
    ///
    /// This method is designed to efficiently produce random numbers by cycling
    /// through the pre-generated numbers stored in the `rsl` vector, updating
    /// the sequence only when necessary.
    pub fn next(&mut self) -> i32 {
        if self.count == 0 {
            self.isaac();
            self.count = 255;
        } else {
            self.count -= 1;
        }
        return self.rsl[self.count];
    }

    /// Initializes the state of the `Isaac` random number generator.
    ///
    /// The `init` function performs multiple mixing passes using the initial
    /// seed values, ensuring that the state variables (`a`, `b`, `c`, `d`, `e`,
    /// `f`, `g`, `h`) are properly initialized. This setup process prepares
    /// the generator to produce pseudo-random numbers in subsequent operations.
    ///
    /// # Side Effects
    ///
    /// This function modifies the internal state of the generator, including
    /// the `mem` array, and performs multiple rounds of mixing to ensure
    /// randomness. The function also invokes the `isaac` method at the end to
    /// finalize the initialization process and prepares the `count` to 256.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    fn init(&mut self) {
        let mut a: i32 = 0x9e3779b9u32 as i32;
        let mut b: i32 = 0x9e3779b9u32 as i32;
        let mut c: i32 = 0x9e3779b9u32 as i32;
        let mut d: i32 = 0x9e3779b9u32 as i32;
        let mut e: i32 = 0x9e3779b9u32 as i32;
        let mut f: i32 = 0x9e3779b9u32 as i32;
        let mut g: i32 = 0x9e3779b9u32 as i32;
        let mut h: i32 = 0x9e3779b9u32 as i32;

        // mix
        for _ in 0..4 {
            // a
            a ^= b << 11;
            d = d.wrapping_add(a);
            b = b.wrapping_add(c);
            // b
            b ^= (c as u32 >> 2) as i32;
            e = e.wrapping_add(b);
            c = c.wrapping_add(d);
            // c
            c ^= d << 8;
            f = f.wrapping_add(c);
            d = d.wrapping_add(e);
            // d
            d ^= (e as u32 >> 16) as i32;
            g = g.wrapping_add(d);
            e = e.wrapping_add(f);
            // e
            e ^= f << 10;
            h = h.wrapping_add(e);
            f = f.wrapping_add(g);
            // f
            f ^= (g as u32 >> 4) as i32;
            a = a.wrapping_add(f);
            g = g.wrapping_add(h);
            // g
            g ^= h << 8;
            b = b.wrapping_add(g);
            h = h.wrapping_add(a);
            // h
            h ^= (a as u32 >> 9) as i32;
            c = c.wrapping_add(h);
            a = a.wrapping_add(b);
        }

        // first pass
        for index in (0..256).step_by(8) {
            // rsl
            a = a.wrapping_add(self.rsl[index]);
            b = b.wrapping_add(self.rsl[index + 1]);
            c = c.wrapping_add(self.rsl[index + 2]);
            d = d.wrapping_add(self.rsl[index + 3]);
            e = e.wrapping_add(self.rsl[index + 4]);
            f = f.wrapping_add(self.rsl[index + 5]);
            g = g.wrapping_add(self.rsl[index + 6]);
            h = h.wrapping_add(self.rsl[index + 7]);

            // a
            a ^= b << 11;
            d = d.wrapping_add(a);
            b = b.wrapping_add(c);
            // b
            b ^= (c as u32 >> 2) as i32;
            e = e.wrapping_add(b);
            c = c.wrapping_add(d);
            // c
            c ^= d << 8;
            f = f.wrapping_add(c);
            d = d.wrapping_add(e);
            // d
            d ^= (e as u32 >> 16) as i32;
            g = g.wrapping_add(d);
            e = e.wrapping_add(f);
            // e
            e ^= f << 10;
            h = h.wrapping_add(e);
            f = f.wrapping_add(g);
            // f
            f ^= (g as u32 >> 4) as i32;
            a = a.wrapping_add(f);
            g = g.wrapping_add(h);
            // g
            g ^= h << 8;
            b = b.wrapping_add(g);
            h = h.wrapping_add(a);
            // h
            h ^= (a as u32 >> 9) as i32;
            c = c.wrapping_add(h);
            a = a.wrapping_add(b);

            // mem
            self.mem[index] = a;
            self.mem[index + 1] = b;
            self.mem[index + 2] = c;
            self.mem[index + 3] = d;
            self.mem[index + 4] = e;
            self.mem[index + 5] = f;
            self.mem[index + 6] = g;
            self.mem[index + 7] = h;
        }

        // second pass
        for index in (0..256).step_by(8) {
            a = a.wrapping_add(self.mem[index]);
            b = b.wrapping_add(self.mem[index + 1]);
            c = c.wrapping_add(self.mem[index + 2]);
            d = d.wrapping_add(self.mem[index + 3]);
            e = e.wrapping_add(self.mem[index + 4]);
            f = f.wrapping_add(self.mem[index + 5]);
            g = g.wrapping_add(self.mem[index + 6]);
            h = h.wrapping_add(self.mem[index + 7]);

            // a
            a ^= b << 11;
            d = d.wrapping_add(a);
            b = b.wrapping_add(c);
            // b
            b ^= (c as u32 >> 2) as i32;
            e = e.wrapping_add(b);
            c = c.wrapping_add(d);
            // c
            c ^= d << 8;
            f = f.wrapping_add(c);
            d = d.wrapping_add(e);
            // d
            d ^= (e as u32 >> 16) as i32;
            g = g.wrapping_add(d);
            e = e.wrapping_add(f);
            // e
            e ^= f << 10;
            h = h.wrapping_add(e);
            f = f.wrapping_add(g);
            // f
            f ^= (g as u32 >> 4) as i32;
            a = a.wrapping_add(f);
            g = g.wrapping_add(h);
            // g
            g ^= h << 8;
            b = b.wrapping_add(g);
            h = h.wrapping_add(a);
            // h
            h ^= (a as u32 >> 9) as i32;
            c = c.wrapping_add(h);
            a = a.wrapping_add(b);

            self.mem[index] = a;
            self.mem[index + 1] = b;
            self.mem[index + 2] = c;
            self.mem[index + 3] = d;
            self.mem[index + 4] = e;
            self.mem[index + 5] = f;
            self.mem[index + 6] = g;
            self.mem[index + 7] = h;
        }

        self.isaac();
        self.count = 256;
    }

    /// Generates the next batch of pseudo-random numbers by mixing the values
    /// in the internal `mem` array and producing new values in the `rsl` array.
    ///
    /// This function performs a series of bitwise operations and additions to
    /// mix the values in `mem`, and then updates the `rsl` array with the
    /// resulting pseudo-random numbers.
    ///
    /// # Side Effects
    ///
    /// The `isaac` function modifies both the `mem` and `rsl` arrays, as well as
    /// the internal state variables `a`, `b`, `c`. It also increments `c` and
    /// adds it to `b` as part of the state update, ensuring a continuous stream
    /// of random numbers with each call.
    ///
    /// # Panics
    ///
    /// This function does not panic.
    fn isaac(&mut self) {
        self.c = self.c.wrapping_add(1);
        self.b = self.b.wrapping_add(self.c);

        for index in 0..256 {
            let x: i32 = self.mem[index];

            match index & 3 {
                0 => self.a ^= self.a << 13,
                1 => self.a ^= (self.a as u32 >> 6) as i32,
                2 => self.a ^= self.a << 2,
                3 => self.a ^= (self.a as u32 >> 16) as i32,
                _ => {}
            }

            self.a = self.a.wrapping_add(self.mem[(index + 128) & 0xff]);

            let y: i32 = self.mem[((x >> 2) & 0xff) as usize]
                .wrapping_add(self.a)
                .wrapping_add(self.b);
            self.mem[index] = y;

            self.b = self.mem[((y >> 10) & 0xff) as usize].wrapping_add(x);
            self.rsl[index] = self.b;
        }
    }
}
