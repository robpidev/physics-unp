use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub type DB = Surreal<Client>;

pub async fn db_connect() -> Result<DB, String> {
    print!("Connecting to database...");
    let db = match Surreal::new::<Ws>("127.0.0.1:8000").await {
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

    Ok(db)
}
