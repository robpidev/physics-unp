use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub type DB = Surreal<Client>;

pub async fn db_connect(host: String) -> Result<DB, String> {
    println!("Connecting to database...");
    let db = match Surreal::new::<Ws>(host).await {
        Ok(db) => db,
        Err(e) => return Err(format!("DB HOST ERROR: {}", e.to_string())),
    };

    if let Err(e) = db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        return Err(format!("DB AUTH ERROR: {}", e.to_string()));
    }

    if let Err(e) = db.use_ns("test").use_db("test").await {
        return Err(format!("DB NAMESPACE ERROR: {}", e.to_string()));
    }

    println!("Database connected!");

    Ok(db)
}
