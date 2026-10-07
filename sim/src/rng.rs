//! Seeded random streams. Every draw comes from a named stream derived from the board seed, so
//! runs are deterministic and arms share draws until their choices diverge.

#[derive(Clone)]
pub struct Rng {
    s: [u64; 4],
}

fn splitmix(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// Mixes a seed with stream labels into a new seed.
pub fn derive(seed: u64, parts: &[u64]) -> u64 {
    let mut x = seed ^ 0x51ED_5EED;
    let mut h = splitmix(&mut x);
    for &p in parts {
        x ^= p.wrapping_mul(0x2545F4914F6CDD1D) ^ h;
        h = splitmix(&mut x);
    }
    h
}

pub fn hash_str(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut x = seed;
        Rng {
            s: [
                splitmix(&mut x),
                splitmix(&mut x),
                splitmix(&mut x),
                splitmix(&mut x),
            ],
        }
    }
    pub fn stream(seed: u64, parts: &[u64]) -> Self {
        Rng::new(derive(seed, parts))
    }
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        let result = (self.s[1].wrapping_mul(5)).rotate_left(7).wrapping_mul(9);
        let t = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);
        result
    }
    #[inline]
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    #[inline]
    pub fn below(&mut self, n: usize) -> usize {
        (((self.next_u64() >> 32) * n as u64) >> 32) as usize
    }
    pub fn chance(&mut self, p: f64) -> bool {
        self.f64() < p
    }
    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = self.below(i + 1);
            v.swap(i, j);
        }
    }
    /// A uniformly random set bit of a nonzero mask.
    pub fn pick_bit(&mut self, m: u64) -> u8 {
        let n = m.count_ones() as usize;
        let mut k = self.below(n);
        let mut x = m;
        loop {
            let c = x.trailing_zeros() as u8;
            if k == 0 {
                return c;
            }
            k -= 1;
            x &= x - 1;
        }
    }
    pub fn weighted(&mut self, w: &[f64]) -> usize {
        let total: f64 = w.iter().sum();
        let mut r = self.f64() * total;
        for (i, &x) in w.iter().enumerate() {
            r -= x;
            if r < 0.0 {
                return i;
            }
        }
        w.len() - 1
    }
}
