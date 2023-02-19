use reqwest::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let iters = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);
    let url = "https://hello.deno.dev";
    let client = Client::builder()
        .use_rustls_tls()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let mut times = 10;
    let mut sum = 0;
    while times > 0 {
        let time = iter(&client, iters, url).await;
        sum += time;
        times -= 1;
    }
    println!("{}ms", sum / 10);
}

async fn iter(client: &Client, iters: usize, base: &str) -> usize {
    let futures = tokio::task::LocalSet::new();
    for i in 0..iters {
        let url = format!("{}?cachebust={}", base, i);

        let future = client.get(&url).send();

        futures.spawn_local(future);
    }

    let start = std::time::Instant::now();
    futures.await;
    start.elapsed().as_millis() as usize
}
