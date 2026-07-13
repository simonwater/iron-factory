use trpl::Html;

pub async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let resp_text = response.text().await;
    Html::parse(&resp_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

#[cfg(test)]
mod tests {
    use super::*;
    use trpl::Either;

    #[test]
    fn test1() {
        trpl::block_on(async {
            match page_title("http://www.baidu.com").await {
                Some(title) => println!("The title for url was {title}"),
                None => println!("url had no title"),
            }
        });
        println!("over!");
    }

    /// future是惰性的，调用await前什么都不会做
    #[test]
    fn test2() {
        trpl::block_on(async {
            let title_fut_1 = page_title("http://www.yeah.net");
            let title_fut_2 = page_title("http://www.sina.com.cn");

            let maybe_title = match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

            match maybe_title {
                Some(title) => println!("Its page title was: '{title}'"),
                None => println!("It had no title."),
            }
        })
    }
}
