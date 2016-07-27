/* Period parameters -- These are all magic.  Don't change. */
const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u32 = 0x9908b0dfu32;    /* constant vector a */
const UPPER_MASK: u32 = 0x80000000u32;  /* most significant w-r bits */
const LOWER_MASK: u32 = 0x7fffffffu32;  /* least significant r bits */

pub struct Random
{
    mt: [u32;N],
    mag01: [u32;2], // {0x0U, MATRIX_A} == mag01[x] = x * MATRIX_A  for x=0,1
    index: usize
}

impl Random {
    pub fn new(seed: u32) -> Random
    {
        let mut mt: [u32;N] = [0u32;N];
        mt[0] = seed;
        for mti in 1..N {
            mt[mti] =
            1812433253u32.wrapping_mul(mt[mti-1] ^ (mt[mti-1] >> 30)).wrapping_add(mti as u32);
            /* See Knuth TAOCP Vol2. 3rd Ed. P.106 for multiplier. */
            /* In the previous versions, MSBs of the seed affect   */
            /* only MSBs of the array mt[].                        */
            /* 2002/01/09 modified by Makoto Matsumoto             */
        }
        Random{mt: mt, mag01: [0x0u32, MATRIX_A], index: N}
    }
    pub fn rand_u32(&mut self) -> u32
    {
        let mag01 = self.mag01;  // Lazy
        let mut mt = self.mt;        // mode
                                 // true
        if self.index >= N { /* generate N words at one time */
            for kk in 0..(N-M) {
                let y = (mt[kk]&UPPER_MASK)|(mt[kk+1]&LOWER_MASK);
                mt[kk] = mt[kk+M] ^ (y >> 1) ^ mag01[(y & 0x1u32) as usize];
            }
            for kk in (N-M)..(N-1) {
                let y = (mt[kk]&UPPER_MASK)|(mt[kk+1]&LOWER_MASK);
                mt[kk] = mt[((kk as i32)+((M as i32)-(N as i32))) as usize] ^ (y >> 1) ^ mag01[(y & 0x1u32) as usize];
            }
            let y = (mt[N-1]&UPPER_MASK)|(mt[0]&LOWER_MASK);
            mt[N-1] = mt[M-1] ^ (y >> 1) ^ mag01[(y & 0x1u32) as usize];

            self.index = 0;
        }

        let mut y = mt[self.index]; self.index += 1;
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680u32;
        y ^= (y << 15) & 0xefc60000u32;
        y ^= y >> 18;
        y
    }
}
