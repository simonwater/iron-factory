use core::slice;

pub fn split_at_mut(arr: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = arr.len();
    let ptr = arr.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut arr = [1, 2, 3];
        let (l, r) = split_at_mut(&mut arr, 0);
        println!("{:?}", l);
        println!("{:?}", r);

        let (l, r) = split_at_mut(&mut arr, 3);
        println!("{:?}", l);
        println!("{:?}", r);

        let (l, r) = split_at_mut(&mut arr, 1);
        println!("{:?}", l);
        println!("{:?}", r);

        let (l, r) = split_at_mut(&mut arr, 2);
        println!("{:?}", l);
        println!("{:?}", r);
    }

    #[test]
    fn test2() {
        let ptr = 0 as *const u8;
        let ptr_bytes = unsafe { *ptr }; // 解引用空指针报错
        println!("{}", ptr_bytes);

        let ptr = 1 as *const u8;
        let ptr_bytes = unsafe { *ptr }; // 解引用无访问权限的地址，报错
        println!("{}", ptr_bytes);

        let x = 123u8;
        let ptr = (&x) as *const u8;
        let ptr_bytes = unsafe { *ptr }; // 成功解引用
        println!("{:p}: {}", ptr, ptr_bytes);
    }
}
