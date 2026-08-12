use notifyx_rust::api::router::api_router;
use tokio::{
    net::TcpListener,
};
use notifyx_rust::configuration::config;

#[tokio::main(flavor="multi_thread", worker_threads=3)]
async fn main() {
    // loading all configuration
    let app_state = config::load().await;

    // creating router
    let router = api_router(app_state);

    // create tcp listner
    let listner = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed too bind listner to address");
    println!("http://127.0.0.1:3000/");

    //
    axum::serve(listner,router)
        .await
        .expect("failed to serve application")


}
