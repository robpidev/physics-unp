use serde::Deserialize;

use serde::de::DeserializeOwned;
use surrealdb::sql::Thing;
use surrealdb::{Error, Response};

use crate::shared::repository::DB;

use super::Notice;

#[derive(Deserialize)]
struct NoticeDB {
    id: Thing,
    note: String,
    datetime: String,
}

pub struct NoticeRepository;

impl NoticeRepository {
    fn process_response<S>(resp: Result<Response, Error>) -> Result<Vec<S>, (u16, String)>
    where
        S: DeserializeOwned,
    {
        let mut resp = match resp {
            Ok(resp) => resp,
            Err(e) => return Err((500, format!("DB conection error: {}", e.to_string()))),
        };

        let data: Vec<S> = match resp.take(0) {
            Ok(data) => data,
            Err(e) => return Err((500, format!("DB error parse: {}", e.to_string()))),
        };

        Ok(data)
    }

    pub async fn create(note: String) -> Result<Notice, (u16, String)> {
        let query = "
            CREATE notice CONTENT {
	              note: $note
            }";

        let notices: Vec<NoticeDB> =
            Self::process_response(DB.query(query).bind(("note", note)).await)?;

        let notice = notices.into_iter().next();

        match notice {
            Some(n) => Ok(Notice {
                id: n.id.id.to_string(),
                note: n.note,
                datetime: n.datetime,
            }),
            None => Err((500, "DB response error".to_string())),
        }
    }
}
