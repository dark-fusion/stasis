/// Generic `Error` type returned by most functions.
///
/// Temporary convenience type until a specialized enum type is implemented to
/// represent all the different types of errors that may occur.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
/// Generic `Result<T>` type provided for convenience and flexibility.
pub type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
