//! 通常数据写入磁盘再读取出来时需要验证是否一致。以防数据写入时发生异常而中断，或者被非法篡改。
//! 常规套路是在原始数据上执行校验函数，将计算出的校验和与原始数据一起写入磁盘。
//! 从磁盘读取数据时，重新计算校验和，然后比较两个校验和。
//!
//! 常见校验函数：奇偶校验、crc32(循环冗余校验，返回结果为32位)、密码学哈希函数。
//! 这三种方式返回结果和实现难度依次增大，执行速度依次变慢，结果稳定性依次提升

/// 奇偶校验实现
pub fn parity_bit(bytes: &[u8]) -> u8 {
    // 获取一个字节切片作为参数bytes，并返回一个单字节作为输出。此函数可以很容易地返回一个布尔值，
    // 但是在这里返回u8，可以让这个返回结果在之后能够移位到某个期望的位置上。
    let mut n_ones: u32 = 0;

    for byte in bytes {
        // Rust的所有整数类型，都配有count_ones() 方法和count_zeros() 方法。
        let ones = byte.count_ones();
        n_ones += ones;
        println!("{} (0b{:08b}) has {} one bits", byte, byte, ones);
    }

    // 有多种方法可以用来优化这个函数。一种很简单的方法就是，可以硬编码一个类型为const [u8; 256]的数组，
    // 数组中的0和1与预期的结果相对应，然后用每个字节对此数组进行索引。
    (n_ones % 2 == 0) as u8
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test() {
        let abc = b"abc";
        println!("input: {:?}", abc);
        println!("output: {:08x}", parity_bit(abc));
        println!();
        let abcd = b"abcd";
        println!("input: {:?}", abcd);
        println!("result: {:08x}", parity_bit(abcd))
    }
}
