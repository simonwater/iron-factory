#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let inputs: Vec<i64> = vec![0, 1, 2, 3, 4, 512];
        let _result = inputs
            .into_iter()
            .map(u8::try_from)
            .collect::<Result<Vec<_>, _>>();
    }
}
