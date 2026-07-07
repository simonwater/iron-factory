#[cfg(test)]
mod tests {

    #[test]
    fn integer_test() {
        let three = 0b11; // 0b前缀表示二进制
        let thirty = 0o36; // 0o前缀表示八进制
        let three_hundred = 0x12c; // 0x前缀表示16进制

        println!("base 10: {} {} {}", three, thirty, three_hundred);
        println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
        println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
        println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

        let a: i32 = -10;
        let b: u16 = 100;
        if (a as u16) < b {
            println!("less!");
        }
    }

    #[test]
    fn float_test() {
        let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
        let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
        println!("abc(f32)");
        println!("  0.1 + 0.2:{:x}", (abc.0 + abc.1).to_bits());
        println!("        0.3:{:x}", abc.2.to_bits());
        println!();

        println!("xyz(f64)");
        println!("  0.1 + 0.2:{:x}", (xyz.0 + xyz.1).to_bits());
        println!("        0.3:{:x}", xyz.2.to_bits());

        assert!(abc.0 + abc.1 == abc.2);
        assert!((xyz.0 + xyz.1 - xyz.2).abs() <= f64::EPSILON);

        let x: f32 = 1.0 / 0.0;
        assert!(x.is_infinite());
    }

    #[test]
    fn loop_test() {}
}
