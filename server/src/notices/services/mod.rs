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

    pub async fn delete(id: String) -> Result<(), (u16, String)> {
        NoticeRepository::delete(id).await
    }

    pub async fn get_all() -> Result<Vec<Notice>, (u16, String)> {
        NoticeRepository::get_all().await
    }
}
