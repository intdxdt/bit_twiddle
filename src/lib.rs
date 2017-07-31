/**
 * Bit twiddling hacks for Rust.
 *
 * Author: Titus Tienaah
 * @after Mikola Lysenko
 * Port of js-version here : https://github.com/mikolalysenko/bit-twiddle
 * Ported from Stanford bit twiddling hack library:
 *    http://graphics.stanford.edu/~seander/bithacks.html
 */


//Number of bits in an integer
pub const INT_BITS: i32 = 32;
pub const INT_MAX: i32 = 0x7fffffff;
pub const INT_MIN: i32 = -1 << (INT_BITS - 1);
pub const REVERSE_TABLE: [i32; 256] = [0, 128, 64, 192, 32, 160, 96, 224, 16, 144, 80, 208, 48, 176, 112, 240, 8, 136, 72, 200, 40, 168, 104, 232, 24, 152, 88, 216, 56, 184, 120, 248, 4, 132, 68, 196, 36, 164, 100, 228, 20, 148, 84, 212, 52, 180, 116, 244, 12, 140, 76, 204, 44, 172, 108, 236, 28, 156, 92, 220, 60, 188, 124, 252, 2, 130, 66, 194, 34, 162, 98, 226, 18, 146, 82, 210, 50, 178, 114, 242, 10, 138, 74, 202, 42, 170, 106, 234, 26, 154, 90, 218, 58, 186, 122, 250, 6, 134, 70, 198, 38, 166, 102, 230, 22, 150, 86, 214, 54, 182, 118, 246, 14, 142, 78, 206, 46, 174, 110, 238, 30, 158, 94, 222, 62, 190, 126, 254, 1, 129, 65, 193, 33, 161, 97, 225, 17, 145, 81, 209, 49, 177, 113, 241, 9, 137, 73, 201, 41, 169, 105, 233, 25, 153, 89, 217, 57, 185, 121, 249, 5, 133, 69, 197, 37, 165, 101, 229, 21, 149, 85, 213, 53, 181, 117, 245, 13, 141, 77, 205, 45, 173, 109, 237, 29, 157, 93, 221, 61, 189, 125, 253, 3, 131, 67, 195, 35, 163, 99, 227, 19, 147, 83, 211, 51, 179, 115, 243, 11, 139, 75, 203, 43, 171, 107, 235, 27, 155, 91, 219, 59, 187, 123, 251, 7, 135, 71, 199, 39, 167, 103, 231, 23, 151, 87, 215, 55, 183, 119, 247, 15, 143, 79, 207, 47, 175, 111, 239, 31, 159, 95, 223, 63, 191, 127, 255];

#[inline]
pub fn not(v: i32) -> i32 { -(v + 1) }

///Returns -1, 0, +1 depending on sign of x
pub fn sign(v: i32) -> i32 {
    ((v > 0) as i32) - ((v < 0) as i32)
}

///Computes absolute value of integer
pub fn abs(v: i32) -> i32 {
    let mask = v >> (INT_BITS - 1);
    (v ^ mask) - mask
}

///Computes minimum of integers x and y
pub fn min(x: i32, y: i32) -> i32 {
    y ^ ((x ^ y) & -((x < y) as i32))
}

///Computes maximum of integers x and y
pub fn max(x: i32, y: i32) -> i32 {
    return x ^ ((x ^ y) & -((x < y) as i32));
}

///Checks if a number is a power of two
pub fn is_pow2(v: i32) -> bool {
    (v != 0) && ((v & (v - 1)) == 0)
}

///Computes log base 2 of v
pub fn log2(v: u32) -> u32 {
    let mut v:u32 = v;
    let mut r:u32;
    let mut shift:u32;
    r =     ((v > 0xFFFF) as u32)<< 4; v >>= r;
    shift = ((v > 0xFF) as u32)  << 3; v >>= shift;r |= shift;
    shift = ((v > 0xF)  as u32)  << 2; v >>= shift;r |= shift;
    shift = ((v > 0x3)  as u32)  << 1; v >>= shift;r |= shift;
    r | (v >> 1)
}

///Computes log base 10 of v
pub fn log10(v: i32) -> i32 {
    if v >= 1000000000 { 9 } else if v >= 100000000 { 8 } else if v >= 10000000 { 7 } else if v >= 1000000 { 6 } else if v >= 100000 { 5 } else if v >= 10000 { 4 } else if v >= 1000 { 3 } else if v >= 100 { 2 } else if v >= 10 { 1 } else { 0 }
}

///Counts number of bits
pub fn pop_count(v: i32) -> i32 {
    let mut v = v - ((v >> 1) & 0x55555555);
    v = (v & 0x33333333) + ((v >> 2) & 0x33333333);
    ((v + (v >> 4) & 0xF0F0F0F) * 0x1010101) >> 24
}

///Counts number of trailing zeros
pub fn count_trailing_zeros(v: i32) -> i32 {
    let mut v = v;
    let mut c = 32;
    v &= -v;
    if v != 0 { c -= 1 };
    if v & 0x0000FFFF != 0 { c -= 16 };
    if v & 0x00FF00FF != 0 { c -= 8 };
    if v & 0x0F0F0F0F != 0 { c -= 4 };
    if v & 0x33333333 != 0 { c -= 2 };
    if v & 0x55555555 != 0 { c -= 1 };
    c
}


///Rounds to next power of 2
pub fn next_pow2(v: i32) -> i32 {
    let mut v = v;
    v += (v == 0) as i32;
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
pub fn reverse(v: i32) -> i32 {
    (REVERSE_TABLE[(v & 0xff) as usize] << 24) |
        (REVERSE_TABLE[((v >> 8) & 0xff) as usize] << 16) |
        (REVERSE_TABLE[((v >> 16) & 0xff) as usize] << 8) |
        REVERSE_TABLE[((v >> 24) & 0xff) as usize]
}

///Interleave bits of 2 coordinates with 16 bits.  Useful for fast quadtree codes
pub fn interleave2(x: i32, y: i32) -> i32 {
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


///Interleave bits of 3 coordinates, each with 10 bits.  Useful for fast octree codes
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
pub fn next_combination(v: i32) -> i32 {
    let t = v | (v - 1);
    (t + 1) | (((not(t) & -not(t)) - 1) >> (count_trailing_zeros(v) + 1))
}

#[cfg(test)]
mod bit_twiddle_test {
    use super::not;
    #[test]
    fn test_bit_twiddle() {
        assert_eq!(not(170), -171);
    }
}
