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
    fn t() {
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
}
