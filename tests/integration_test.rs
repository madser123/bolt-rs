
#[tokio::test]
async fn get_users() {
    dotenv::from_filename(".dev.env").ok();
    let token = dotenv::var("SLACK_BOT_TOKEN").unwrap();

    //match bolt_rs::user::User::from_email(&token, "maj@hiper.dk").await {
    //    Ok(u) => println!("{u:?}"),
    //    Err(e) => panic!("{e}"),
    //};

    //match bolt_rs::user::User::from_email(&token, "mfc@hiper.dk").await {
    //    Ok(u) => println!("{u:?}"),
    //    Err(e) => panic!("{e}"),
    //};

    //match bolt_rs::user::User::from_id(&token, "USLACKBOT").await {
    //    Ok(u) => println!("{u:?}"),
    //    Err(e) => panic!("{e}"),
    //};

    //match bolt_rs::user::UserList::new(&token).await {
    //    Ok(ul) => println!("{ul:?}"),
    //    Err(e) => panic!("{e}"),
    //}
}
