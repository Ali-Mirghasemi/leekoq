//!
//! 

#![no_std]

pub struct LeeKoq;

impl LeeKoq {
    // 1-bit lookup table for the NLF
    pub const LUT: u32 = 0x3A5C742E;

    /// Encrypts a 32-bit block of plaintext using the KeeLoq algorithm.
    /// 
    /// # Arguments
    /// 
    /// * `block` - 32-bit plaintext block
    /// * `key` - 64-bit key
    /// * `return` - 32-bit cipher-text block
    /// 
    /// # Example
    /// 
    /// ```rust,run
    /// use leekoq::LeeKoq;
    /// 
    /// let plain: u32 = 0x12345678;
    /// let key: u64 = 0xCAFED00D;
    /// assert_eq!(LeeKoq::encrypt(plain, key), 0xD0FB287C);
    /// ```
    /// 
    pub fn encrypt(mut block: u32, mut key: u64) -> u32 {
        for _ in 0..528 {
            // Calculate LUT key
            let lutkey = (block >> 1) & 1 | (block >> 8) & 2 | (block >> 18) & 4 | (block >> 23) & 8 | (block >> 27) & 16;

            // Calculate next bit to feed
            let msb = (block >> 16 & 1) ^ (block & 1) ^ (Self::LUT >> lutkey & 1) ^ ((key & 1) as u32);

            // Feed it
            block = msb << 31 | block >> 1;

            // Rotate key right
            key = (key & 1) << 63 | key >> 1;
        }

        block
    }

    /// Decrypts a 32-bit block of ciphertext using the KeeLoq algorithm.
    /// 
    /// # Arguments 
    /// 
    /// * `block` - 32-bit plaintext block
    /// * `key` - 64-bit key
    /// * `return` - 32-bit cipher-text block
    /// 
    /// # Example
    /// 
    /// ```rust,run
    /// use leekoq::LeeKoq;
    /// 
    /// let cipher: u32 = 0xD0FB287C;
    /// let key: u64 = 0xCAFED00D;
    /// assert_eq!(LeeKoq::decrypt(cipher, key), 0x12345678);
    /// ```
    ///
    pub fn decrypt(mut block: u32, mut key: u64) -> u32 {
        for _ in 0..528 {
            // Calculate LUT key
            let lutkey = (block >> 0) & 1 | (block >> 7) & 2 | (block >> 17) & 4 | (block >> 22) & 8 | (block >> 26) & 16;

            // Calculate next bit to feed
            let lsb = (block >> 31) ^ (block >> 15 & 1) ^ (Self::LUT >> lutkey & 1) ^ ((key >> 15 & 1) as u32);

            // Feed it
            block = (block & 0x7FFFFFFF) << 1 | lsb;

            // Rotate key left
            key = (key & 0x7FFFFFFFFFFFFFFF) << 1 | key >> 63;
        }

        block
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt() {
        assert_eq!(LeeKoq::encrypt(0xf741e2db, 0x5cec6701b79fd949), 0xe44f4cdf);
        assert_eq!(LeeKoq::encrypt(0x0ca69b92, 0x5cec6701b79fd949), 0xa6ac0ea2);
        assert_eq!(LeeKoq::encrypt(0x12345678, 0xCAFED00D), 0xD0FB287C);
    }

    #[test]
    fn decrypt() {
        assert_eq!(LeeKoq::decrypt(0xe44f4cdf, 0x5cec6701b79fd949), 0xf741e2db);
        assert_eq!(LeeKoq::decrypt(0xa6ac0ea2, 0x5cec6701b79fd949), 0x0ca69b92);
        assert_eq!(LeeKoq::decrypt(0xD0FB287C, 0xCAFED00D), 0x12345678);
    }
}
