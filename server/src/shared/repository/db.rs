use std::sync::LazyLock;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn db_connect(
    host: String,
    user: String,
    pass: String,
    ns: String,
    db: String,
) -> Result<(), String> {
    println!("Connecting to database...");
    match DB.connect::<Ws>(host).await {
        Ok(_) => (),
        Err(e) => return Err(format!("DB HOST ERROR: {}", e.to_string())),
    };

    if let Err(e) = DB
        .signin(Root {
            username: user.as_str(),
            password: pass.as_str(),
        })
        .await
    {
        return Err(format!("DB AUTH ERROR: {}", e.to_string()));
    }

    if let Err(e) = DB.use_ns(ns.as_str()).use_db(db.as_str()).await {
        return Err(format!("DB NAMESPACE ERROR: {}", e.to_string()));
    }

    println!("Database connected!");

    Ok(())
}
