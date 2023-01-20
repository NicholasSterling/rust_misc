
// LIMITS[log] is the highest value for which log10_floor(x) will give you log.
const LIMITS: [u16; 5] = [
         9,
        99,
       999,
     9_999,
    u16::MAX
];

const LOG10S_FOR_LOG2S: [u8; 16] = [
//        log2     x
//        ---- ------
    0, //   0      1
    0, //   1      2
    0, //   2      4
    0, //   3      8  *
    1, //   4     16
    1, //   5     32
    1, //   6     64  *
    2, //   7    128
    2, //   8    256
    2, //   9    512  *
    3, //  10   1024
    3, //  11   2048
    3, //  12   4096
    3, //  13   8192  *
    4, //  14  16384
    4, //  15  32768
];

// Returns the floor of log base 10 of its argument.
// In reality you would make this generic, supporting types up to u128.
// The same tables could be used for all of the u* types.
// This routine uses the floor(log2(x)) function in order to get good performance;
// on modern architectures there is typically a fairly quick instruction for that.
pub fn ilog10_floor(x: u16) -> u8 {
    let log2x = x.log2() as usize;
    let log10x_guess = unsafe {
        // SAFETY: ilog2_floor of a u16 can only be 0..15,
        // for which there are elements in the array.
        *(&LOG10S_FOR_LOG2S).get_unchecked(log2x)
    };
    let limit = unsafe {
        // SAFETY: Indices come from LOG10S_FOR_LOG2S,
        // and we made sure we have an entry for each.
        *(&LIMITS).get_unchecked(log10x_guess as usize)
    };
    if x > limit {
        log10x_guess + 1
    } else {
        log10x_guess
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test0() {
        assert_eq!(log10_floor(0), 0);
    }

    #[test]
    fn test1() {
        assert_eq!(log10_floor(       1), 0);
        assert_eq!(log10_floor(       9), 0);
        assert_eq!(log10_floor(      10), 1);
        assert_eq!(log10_floor(      99), 1);
        assert_eq!(log10_floor(     100), 2);
        assert_eq!(log10_floor(     999), 2);
        assert_eq!(log10_floor(   1_000), 3);
        assert_eq!(log10_floor(   9_999), 3);
        assert_eq!(log10_floor(  10_000), 4);
        assert_eq!(log10_floor(u16::MAX), 4);
    }

}
