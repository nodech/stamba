use std::fmt::Debug;

pub mod home;
pub use home::MainPage;

pub trait Page: Debug {
    fn page_title(&self) -> &str;
}
