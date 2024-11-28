use reqwest::Client;

pub static URL: &str = "https://jsonplaceholder.typicode.com/posts/2";

async fn get_response() {
    let c: Client = Client::new();
    let res = c.get(URL).send().await;
    let body = match res {
        Ok(r) => match r.text().await {
            Ok(txt) => txt,
            Err(e) => panic!("{:#?}", e),
        },
        Err(e) => panic!("{:#?}", e),
    };
    println!("Body: {:#?}", body);
}

#[tokio::main]
async fn main() {
    let _ = get_response().await;
}
