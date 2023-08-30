pub trait ResultExtension<T, E> {
    fn or_error(self, msg: String) -> Result<T, anyhow::Error>;
    fn or_error_str(self, msg: &str) -> Result<T, anyhow::Error>;
}

impl<T, E> ResultExtension<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn or_error(self, msg: String) -> Result<T, anyhow::Error> {
        self.or_else(|e| Err(anyhow::anyhow!(format!("{}: {}", msg, e))))
    }

    fn or_error_str(self, msg: &str) -> Result<T, anyhow::Error> {
        self.or_else(|e| Err(anyhow::anyhow!(format!("{}: {}", msg, e))))
    }
}

pub trait OptionExtension<T> {
    fn or_error(self, msg: String) -> Result<T, anyhow::Error>;
    fn or_error_str(self, msg: &str) -> Result<T, anyhow::Error>;
}

impl<T> OptionExtension<T> for Option<T> {
    fn or_error(self, msg: String) -> Result<T, anyhow::Error> {
        if let Some(value) = self {
            return Ok(value);
        }

        return Err(anyhow::anyhow!(msg));
    }

    fn or_error_str(self, msg: &str) -> Result<T, anyhow::Error> {
        if let Some(value) = self {
            return Ok(value);
        }

        return Err(anyhow::anyhow!(msg.to_string()));
    }
}
