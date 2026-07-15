mod tests {
    use trpl::StreamExt;

    #[test]
    fn test() {
        trpl::block_on(async {
            let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let iter = values.iter().map(|v| v * v);
            let mut stream = trpl::stream_from_iter(iter);
            while let Some(val) = stream.next().await {
                println!("{val}");
            }
        });
    }
}
