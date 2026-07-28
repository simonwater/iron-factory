//! 使用和文件随机访问相同的接口访问Vec<u8>
#[cfg(test)]
mod tests {
    use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
    use std::io::Cursor;

    #[test]
    fn write_read_vec() {
        let mut w = vec![];
        let one = 1u32;
        let two = 2i8;
        let three = 3.0_f64;
        w.write_u32::<LittleEndian>(one).unwrap();
        println!("{:?}", &w);
        w.write_i8(two).unwrap();
        println!("{:?}", &w);
        w.write_f64::<LittleEndian>(three).unwrap();
        println!("{:?}", &w);

        let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
        let one_ = r.read_u32::<LittleEndian>().unwrap();
        let two_ = r.read_i8().unwrap();
        let three_ = r.read_f64::<LittleEndian>().unwrap();

        assert_eq!(one, one_);
        assert_eq!(two, two_);
        assert_eq!(three, three_);
    }
}
