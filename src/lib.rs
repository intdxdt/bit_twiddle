/**
 * Bit twiddling hacks for Rust.
 * Based on Stanford bit twiddling hack library:
 *    http://graphics.stanford.edu/~seander/bithacks.html
 */

pub const INT_BITS: i32 = 32;//number of bits in an integer
pub const INT_MAX: i32 = 0x7fffffff;
pub const INT_MIN: i32 = -1 << (INT_BITS - 1);
pub const REVERSE_TABLE: [u32; 256] = [0, 128, 64, 192, 32, 160, 96, 224, 16, 144, 80, 208, 48,
    176, 112, 240, 8, 136, 72, 200, 40, 168, 104, 232, 24, 152, 88, 216, 56, 184, 120, 248, 4,
    132, 68, 196, 36, 164, 100, 228, 20, 148, 84, 212, 52, 180, 116, 244, 12, 140, 76, 204, 44,
    172, 108, 236, 28, 156, 92, 220, 60, 188, 124, 252, 2, 130, 66, 194, 34, 162, 98, 226, 18,
    146, 82, 210, 50, 178, 114, 242, 10, 138, 74, 202, 42, 170, 106, 234, 26, 154, 90, 218, 58,
    186, 122, 250, 6, 134, 70, 198, 38, 166, 102, 230, 22, 150, 86, 214, 54, 182, 118, 246, 14,
    142, 78, 206, 46, 174, 110, 238, 30, 158, 94, 222, 62, 190, 126, 254, 1, 129, 65, 193, 33,
    161, 97, 225, 17, 145, 81, 209, 49, 177, 113, 241, 9, 137, 73, 201, 41, 169, 105, 233, 25,
    153, 89, 217, 57, 185, 121, 249, 5, 133, 69, 197, 37, 165, 101, 229, 21, 149, 85, 213, 53,
    181, 117, 245, 13, 141, 77, 205, 45, 173, 109, 237, 29, 157, 93, 221, 61, 189, 125, 253, 3,
    131, 67, 195, 35, 163, 99, 227, 19, 147, 83, 211, 51, 179, 115, 243, 11, 139, 75, 203, 43,
    171, 107, 235, 27, 155, 91, 219, 59, 187, 123, 251, 7, 135, 71, 199, 39, 167, 103, 231, 23,
    151, 87, 215, 55, 183, 119, 247, 15, 143, 79, 207, 47, 175, 111, 239, 31, 159, 95, 223, 63,
    191, 127, 255];

///not ~
#[inline]
pub fn not(v: i32) -> i32 { -(v + 1) }

///bool to i32
#[inline]
pub fn bi32(b: bool) -> i32 { b as i32 }

///bool to u32
#[inline]
pub fn bu32(b: bool) -> u32 { b as u32 }

///u32 to usize
#[inline]
pub fn usz(v: u32) -> usize { v as usize }

///Returns -1, 0, +1 depending on sign of x
#[inline]
pub fn sign(v: i32) -> i32 {
    bi32(v > 0) - bi32(v < 0)
}

///Computes absolute value of integer
#[inline]
pub fn abs(v: i32) -> i32 {
    let mask = v >> (INT_BITS - 1);
    (v ^ mask) - mask
}

///Computes minimum of integers x and y
#[inline]
pub fn min(x: i32, y: i32) -> i32 {
    y ^ ((x ^ y) & -(bi32(x < y)))
}

///Computes maximum of integers x and y
#[inline]
pub fn max(x: i32, y: i32) -> i32 {
    x ^ ((x ^ y) & -(bi32(x < y)))
}

///Checks if a number is a power of two
pub fn is_pow2(v: i32) -> bool {
    (v != 0) && ((v & (v - 1)) == 0)
}

///Computes log base 2 of v
pub fn log2(v: u32) -> u32 {
    let mut v: u32 = v;
    let mut r: u32;
    let mut shift: u32;
    r = bu32(v > 0xFFFF) << 4;
    v >>= r;
    shift = bu32(v > 0xFF) << 3;
    v >>= shift;
    r |= shift;
    shift = bu32(v > 0xF) << 2;
    v >>= shift;
    r |= shift;
    shift = bu32(v > 0x3) << 1;
    v >>= shift;
    r |= shift;
    r | (v >> 1)
}

///Computes log base 10 of v
pub fn log10(v: i32) -> i32 {
    if v >= 1000000000 { 9 } else if v >= 100000000 { 8 } else if v >= 10000000 { 7 } else if v >= 1000000 { 6 } else if v >= 100000 { 5 } else if v >= 10000 { 4 } else if v >= 1000 { 3 } else if v >= 100 { 2 } else if v >= 10 { 1 } else { 0 }
}

///Counts number of bits
pub fn pop_count(v: u32) -> u32 {
    //    other solution
    //    let mut v = v - ((v >> 1) & 0x55555555);
    //    v = (v & 0x33333333) + ((v >> 2) & 0x33333333);
    //    ((v + (v >> 4) & 0xF0F0F0F) * 0x1010101) >> 24 //i32 & u32 cause overflow on this line
    let mut c;
    let (s0, s1, s2, s3, s4) = (1, 2, 4, 8, 16); // magic binary numbers
    let (b0, b1, b2, b3, b4) = (0x55555555, 0x33333333, 0x0F0F0F0F, 0x00FF00FF, 0x0000FFFF);

    c = v - ((v >> s0) & b0);
    c = ((c >> s1) & b1) + (c & b1);
    c = ((c >> s2) + c) & b2;
    c = ((c >> s3) + c) & b3;
    ((c >> s4) + c) & b4
}

//finding the log base 2 in parallel
//first isolate the lowest 1 bit, and then
// proceed with c starting at the maximum and decreasing
//pub fn count_trailing_zeros(v: u32) -> u32 {
//    let mut v = v;
//    let mut c = 32u32;
//    let sv = v as i32;
//    v = (sv & -sv) as u32; //NOTE: may overflow u32 -> -i32
//    if v != 0 { c -= 1 };
//    if v & 0x0000FFFF != 0 { c -= 16 };
//    if v & 0x00FF00FF != 0 { c -= 8 };
//    if v & 0x0F0F0F0F != 0 { c -= 4 };
//    if v & 0x33333333 != 0 { c -= 2 };
//    if v & 0x55555555 != 0 { c -= 1 };
//    c
//}
///Computes the number of trailing zeros by accumulating c
/// in a manner akin to binary search
pub fn count_trailing_zeros(v: u32) -> u32 {
    let mut v = v;
    let mut c = 32u32;
    // NOTE: if 0 == v, then c = 31.
    //short circuit v == 0 as 32
    if v == 0 { return c; }
    if v & 0x1 != 0 {
        c = 0;// special case for odd v (assumed to happen half of the time)
    } else {
        c = 1;
        if (v & 0xffff) == 0 {
            v >>= 16;
            c += 16;
        }
        if (v & 0xff) == 0 {
            v >>= 8;
            c += 8;
        }
        if (v & 0xf) == 0 {
            v >>= 4;
            c += 4;
        }
        if (v & 0x3) == 0 {
            v >>= 2;
            c += 2;
        }
        c -= v & 0x1;
    }
    c
}


///Rounds to next power of 2
pub fn next_pow2(v: i32) -> i32 {
    let mut v = v;
    v += bi32(v == 0);
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}

///Rounds down to previous power of 2
pub fn prev_pow2(v: i32) -> i32 {
    let mut v = v;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v - (v >> 1)
}

///Computes parity of word
pub fn parity(v: i32) -> i32 {
    let mut v = v;
    v ^= v >> 16;
    v ^= v >> 8;
    v ^= v >> 4;
    v &= 0xf;
    (0x6996 >> v) & 1
}

//fn _gen_reverse_table(table:&mut [i32;256]) {
//  for i in  0..256 {
//    let mut  v = i;
//    let mut r = i;
//    let mut s = 7;
//    v >>= 1;
//    while v != 0 {
//      r <<= 1;
//      r |= v & 1;
//      s-=1;
//      v >>= 1;
//    }
//    table[i] = (r << s) & 0xff;
//  }
//}


///Reverse bits in a 32 bit word
pub fn reverse(v: u32) -> u32 {
    (REVERSE_TABLE[usz((v & 0xff))] << 24) |
        (REVERSE_TABLE[usz((v >> 8) & 0xff)] << 16) |
        (REVERSE_TABLE[usz((v >> 16) & 0xff)] << 8) |
        REVERSE_TABLE[usz((v >> 24) & 0xff)]
}

///Interleave bits of 2 coordinates with 16   Useful for fast quadtree codes
pub fn interleave2(x: u32, y: u32) -> u32 {
    let mut x = x;
    let mut y = y;
    x &= 0xFFFF;
    x = (x | (x << 8)) & 0x00FF00FF;
    x = (x | (x << 4)) & 0x0F0F0F0F;
    x = (x | (x << 2)) & 0x33333333;
    x = (x | (x << 1)) & 0x55555555;

    y &= 0xFFFF;
    y = (y | (y << 8)) & 0x00FF00FF;
    y = (y | (y << 4)) & 0x0F0F0F0F;
    y = (y | (y << 2)) & 0x33333333;
    y = (y | (y << 1)) & 0x55555555;

    x | (y << 1)
}

///Extracts the nth interleaved component
pub fn deinterleave2(v: u32, n: u32) -> u32 {
    let mut v = v;
    v = (v >> n) & 0x55555555;
    v = (v | (v >> 1)) & 0x33333333;
    v = (v | (v >> 2)) & 0x0F0F0F0F;
    v = (v | (v >> 4)) & 0x00FF00FF;
    v = (v | (v >> 16)) & 0x000FFFF;
    (v << 16) >> 16
}


///Interleave bits of 3 coordinates, each with 10   Useful for fast octree codes
pub fn interleave3(x: u32, y: u32, z: u32) -> u32 {
    let mut x = x;
    let mut y = y;
    let mut z = z;
    x &= 0x3FF;
    x = (x | (x << 16)) & 4278190335;
    x = (x | (x << 8)) & 251719695;
    x = (x | (x << 4)) & 3272356035;
    x = (x | (x << 2)) & 1227133513;

    y &= 0x3FF;
    y = (y | (y << 16)) & 4278190335;
    y = (y | (y << 8)) & 251719695;
    y = (y | (y << 4)) & 3272356035;
    y = (y | (y << 2)) & 1227133513;
    x |= y << 1;

    z &= 0x3FF;
    z = (z | (z << 16)) & 4278190335;
    z = (z | (z << 8)) & 251719695;
    z = (z | (z << 4)) & 3272356035;
    z = (z | (z << 2)) & 1227133513;

    x | (z << 2)
}

///Extracts nth interleaved component of a 3-tuple
pub fn deinterleave3(v: u32, n: u32) -> u32 {
    let mut v = v;
    v = (v >> n) & 1227133513;
    v = (v | (v >> 2)) & 3272356035;
    v = (v | (v >> 4)) & 251719695;
    v = (v | (v >> 8)) & 4278190335;
    v = (v | (v >> 16)) & 0x3FF;
    (v << 22) >> 22
}

///Computes next combination in colexicographic order (this is mistakenly called
/// nextPermutation on the bit twiddling hacks page)
pub fn next_combination(v: u32) -> u32 {
    let t = v | (v - 1);
    let c = (not(t as i32) & -not(t as i32)) as u32 - 1;
    (t + 1) | (c >> (count_trailing_zeros(v) + 1))
}

#[cfg(test)]
mod bit_twiddle_test {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(not(170), -171);
        assert_eq!(not(0), -1);
        assert_eq!(not(-3), 2);
    }

    #[test]
    fn test_sign() {
        assert_eq!(sign(-100), -1);
        assert_eq!(sign(100), 1);
        assert_eq!(sign(0), 0);
        assert_eq!(sign(INT_MAX), 1);
        assert_eq!(sign(INT_MIN), -1);
    }

    #[test]
    fn test_abs() {
        assert_eq!(abs(0), 0);
        assert_eq!(abs(1), 1);
        assert_eq!(abs(-1), 1);
        assert_eq!(abs(INT_MAX), INT_MAX);
        assert_eq!(abs(-INT_MAX), INT_MAX);
        //abs(-INT_MIN) -- overflow
    }

    #[test]
    fn test_min() {
        assert_eq!(min(0, 0), 0);
        assert_eq!(min(-1, 1), -1);
        assert_eq!(min(INT_MAX, INT_MAX), INT_MAX);
        assert_eq!(min(INT_MIN, INT_MIN), INT_MIN);
        assert_eq!(min(INT_MAX, INT_MIN), INT_MIN);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(0, 0), 0);
        assert_eq!(max(-1, 1), 1);
        assert_eq!(max(INT_MAX, INT_MAX), INT_MAX);
        assert_eq!(max(INT_MIN, INT_MIN), INT_MIN);
        assert_eq!(max(INT_MAX, INT_MIN), INT_MAX);
    }

    #[test]
    fn test_is_pow2() {
        assert!(!is_pow2(0));
        for i in 0..31 {
            assert!(is_pow2((1 << i)));
        }
        assert!(!is_pow2(100));
        assert!(!is_pow2(0x7fffffff));
        assert!(!is_pow2(-1000000));
    }
}

#[test]
fn test_log2() {
    for i in 0..31 {
        if i > 0 {
            assert_eq!(log2((1 << i) - 1), i - 1);
            assert_eq!(log2((1 << i) + 1), i);
        }
        assert_eq!(log2((1 << i)), i);
    }
}

#[test]
fn test_pop_count() {
    assert_eq!(pop_count(0), 0);
    assert_eq!(pop_count(1), 1);
    //assert_eq!(pop_count(-1), 32);
    for i in 0..31 {
        assert_eq!(pop_count(1 << i), 1);
        assert_eq!(pop_count((1 << i) - 1), i);
    }
    assert_eq!(pop_count(0xf0f00f0f), 16); //overflow for i32
}

#[test]
fn test_count_trailing_zeros() {
    assert_eq!(count_trailing_zeros(0), 32);
    assert_eq!(count_trailing_zeros(1), 0);
    //    assert_eq!(count_trailing_zeros(-1), 0);
    for i in 0..31 {
        assert_eq!(count_trailing_zeros(1 << i), i);
        if i > 0 {
            assert_eq!(count_trailing_zeros((1 << i) - 1), 0)
        }
    }
    assert_eq!(count_trailing_zeros(0xf81700), 8);
}

#[test]
fn test_next_pow2() {
    for i in 0..31 {
        if i != 1 {
            assert_eq!(next_pow2((1 << i) - 1), 1 << i);
        }
        assert_eq!(next_pow2((1 << i)), 1 << i);
        if i < 30 {
            assert_eq!(next_pow2((1 << i) + 1), 1 << (i + 1));
        }
    }
}

#[test]
fn test_prev_pow2() {
    println!("{i:>2}    {input:>w$}    {prev:>w$}", i = "i", input = "((1 << i) + 1)", prev = "prev_pow2", w = 10);
    println!("{}", "-".repeat(34));
    for i in 0..31 {
        if i > 0 {
            assert_eq!(prev_pow2((1 << i) - 1), 1 << (i - 1));
        }
        assert_eq!(prev_pow2((1 << i)), 1 << i);

        if 0 < i && i < 30 {
            println!("{i:>2} .. {input:>w$} .. {prev:>w$}", i = i, input = ((1 << i) + 1), prev = prev_pow2((1 << i) + 1), w = 10);
            assert_eq!(prev_pow2((1 << i) + 1), 1 << i);
        }
    }
}

#[test]
fn test_parity() {
    assert_eq!(parity(1), 1);
    assert_eq!(parity(0), 0);
    assert_eq!(parity(0xf), 0);
    assert_eq!(parity(0x10f), 1);
}

#[test]
fn test_reverse() {
    assert_eq!(reverse(0), 0);
    //    assert_eq!(reverse(-1), -1);
}

#[test]
fn test_next_combination() {
    assert_eq!(next_combination(1), 2);
    assert_eq!(next_combination(0x300), 0x401);
}

#[test]
fn test_interleave2() {
    for x in 0..100 {
        for y in 0..100 {
            let h = interleave2(x, y);
            assert_eq!(deinterleave2(h, 0), x);
            assert_eq!(deinterleave2(h, 1), y);
        }
    }
}

#[test]
fn test_interleave3() {
    for x in 0..(25 + 1) {
        for y in 0..(25 + 1) {
            for z in 0..(25 + 1) {
                let h = interleave3(x, y, z);
                assert_eq!(deinterleave3(h, 0), x);
                assert_eq!(deinterleave3(h, 1), y);
                assert_eq!(deinterleave3(h, 2), z);
            }
        }
    }
}