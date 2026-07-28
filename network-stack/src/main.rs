use reqwest;

fn main() {
    let mut response = reqwest::get("http://www.baidu.com").unwrap();
    let content = response.text().unwrap();
    print!("{}", content);
}
