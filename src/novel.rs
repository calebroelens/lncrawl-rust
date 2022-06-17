pub mod novel_fetch {
    #[derive(rocket::serde::Deserialize)]
    pub struct NovelInfo {
        source: String,
        identifier: String
    }
}