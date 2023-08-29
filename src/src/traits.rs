pub trait ResultExtension<T, E> {
    fn or_error(self, msg: String) -> Result<T, anyhow::Error>;
}

impl<T, E> ResultExtension<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn or_error(self, msg: String) -> Result<T, anyhow::Error> {
        self.or_else(|e| Err(anyhow::anyhow!(format!("{}: {}", msg, e))))
    }
}
