pub struct BaseTemplateData {
    pub is_logged_in: bool,
}

impl BaseTemplateData {
    pub fn new(is_logged_in: bool) -> Self {
        Self { is_logged_in }
    }
}
