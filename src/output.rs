/*
use hyper::Client;
use hyper::http::Uri;
use hyper::body::HttpBody;

#[tokio::test]
pub async fn get() {
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse::<Uri>().unwrap();
    let mut resp = client.get(uri).await.unwrap();
    dbg!(&resp);

    while let Some(chunk) = resp.body_mut().data().await {
        let chunk = chunk.unwrap().to_vec();
        let strs = String::from_utf8(chunk).unwrap();
    
        println!("{}", &strs);
    }
}
*/