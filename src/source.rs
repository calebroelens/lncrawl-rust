mod sourcemanager {

    struct NovelSource {
        base_url: String,
        has_manga: bool,
        machine_translation: bool,
        clean_tags: Vec<String>,
        login_email: String,
        login_password: String,
        title_location_tags: String,
        author_location_tags: String,
        chapter_count_taggable: bool,
        chapter_count_location_tags: String,
        cover_image_location_tags: String,
        chapter_content_location_tags: String,
        chapter_title_location_tags: String,
        chapter_id_location_tags: String,
    }
}