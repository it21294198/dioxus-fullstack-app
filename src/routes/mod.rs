mod about; // Import the submodule about.rs
mod home;

pub use about::About; // Re-export items from about.rs
pub use home::Home;
