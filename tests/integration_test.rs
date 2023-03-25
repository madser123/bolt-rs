use std::net::SocketAddr;

use bolt_rs::{
    App,
    app::{
        Auth,
        Shortcut,
        AppResult,
    },
};


async fn shortcut1(i: Shortcut) -> AppResult<()> {
    println!("{i:?}");
    Ok(())
}

#[tokio::test]
async fn create_app() {
    let auth = Auth::default();

    let app = App::new(auth)
        .address(SocketAddr::from(([127, 0, 0, 0], 3000)))
        .shortcut("shortcut1", shortcut1)
        .start()
        .await;
}
