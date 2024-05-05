use rspotify::model::FullTrack;

pub trait ResultExtension<T, E> {
    fn or_error(self, msg: String) -> Result<T, anyhow::Error>;
    fn or_error_str(self, msg: &str) -> Result<T, anyhow::Error>;
    fn or_status_str(self, msg: &str) -> Result<T, tonic::Status>;
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

    fn or_status_str(self, msg: &str) -> Result<T, tonic::Status> {
        self.or_else(|e| Err(tonic::Status::internal(format!("{}: {}", msg, e))))
    }
}

pub trait OptionExtension<T> {
    fn or_error(self, msg: String) -> Result<T, anyhow::Error>;
    fn or_error_str(self, msg: &str) -> Result<T, anyhow::Error>;
    fn or_status_str(self, msg: &str) -> Result<T, tonic::Status>;
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

    fn or_status_str(self, msg: &str) -> Result<T, tonic::Status> {
        if let Some(value) = self {
            return Ok(value);
        }

        return Err(tonic::Status::internal(msg));
    }
}

impl Into<crate::types::Track> for FullTrack {
    fn into(self) -> crate::types::Track {
        crate::types::Track {
            id: self.id,
            name: self.name,
            is_local: self.is_local,
            artists: self.artists,
            album_artists_ids: self
                .album
                .artists
                .into_iter()
                .map(|artist| artist.id.unwrap())
                .collect::<Vec<_>>(),
        }
    }
}

impl PartialEq for crate::types::TrackTuple {
    fn eq(&self, other: &Self) -> bool {
        self.artist_id == other.artist_id && self.name == other.name
    }
}
