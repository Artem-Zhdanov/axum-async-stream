use async_stream::stream;
use axum::response::IntoResponse;
use axum::routing::*;
use axum::Router;
use axum_streams::*;
use futures::prelude::*;
use serde_json::json;
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum MyError {}

/*
This code works.
The problem is that I need to describe returing value of `create_stream_of_int` (line 23 )
using a Trait object (dyn Trait), something like:

`async fn get_pg_data_stream() -> Result<Box<dyn Stream<Item = String>>, MyError>`

but it doesn't work out :-(
*/
async fn create_stream_of_int() -> Result<impl Stream<Item = u32>, MyError> {
    let strm = stream! {
        for i in 1..5 {
            let my_int =  async {
                // Mimic async operation
                sleep(Duration::from_millis(1000)).await;
                i
            }.await;
            yield  my_int
        }
    };
    Ok(strm)
}

async fn request_jobs_handler() -> impl IntoResponse {
    let stream_of_ints = create_stream_of_int().await.unwrap();

    // This part is important. I need to transform stream
    let stream_of_objs = stream_of_ints.map(|i| json!({"result": i * 2}));

    StreamBodyAs::json_array(stream_of_objs)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(request_jobs_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
