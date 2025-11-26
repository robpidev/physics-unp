mod entities;
mod repository;
use serde::Serialize;

use entities::Notice;
use repository::NoticeRepository;

pub struct NoticeService;

impl NoticeService {
    pub async fn create(notice: String) -> Result<impl Serialize, (u16, String)> {
        NoticeRepository::create(notice).await
    }
}
