use super::Page;

#[derive(Debug)]
pub struct MainPage {
    
}

impl Page for MainPage {
    fn page_title(&self) -> &str {
        "Main Page"
    }
}
