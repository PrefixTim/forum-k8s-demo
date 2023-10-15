mod note;
mod routes;
mod services;

pub use self::{note::{Note, NoteBase}, services::NoteSer, routes::router};