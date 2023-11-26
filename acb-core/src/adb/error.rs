#[derive(Debug)]
pub enum RustADBError {
    Utf8StringError,
    CommandError,
    NotFoundError
}