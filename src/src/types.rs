impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} from {} for/to {} and idx is {}",
            self.action_type, self.node, self.for_node, self.idx
        )
    }
}

impl std::str::FromStr for QuerySource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "liked" => Ok(QuerySource::LikedSongs),
            "playlists" => Ok(QuerySource::Playlists),
            "albums" => Ok(QuerySource::Albums),
            _ => Err(anyhow::anyhow!(format!("Invalid source: {}", s))),
        }
    }
}

#[derive(Debug)]
pub struct Action {
    pub action_type: ActionType,
    pub node: String,
    pub for_node: String,
    pub idx: usize,
    pub playlist_url: Option<String>,
}

#[derive(Debug)]
pub enum ActionType {
    CreatePlaylist,
    QuerySongs(Option<String>),
    QuerySongsByArtist(QuerySongsByArtist),

    /// SaveChanges is responsible for also saving the state locally.
    SaveChanges(Option<String>),
    CopySongs,

    /// RemoveSongs should only remove songs not added by the user. Only be the bot.
    /// There is also a chance that song from a child playlist was added by a user.
    /// In that case we should not remove it. (No idea how to do that yet)
    RemoveSongs,
}

#[derive(Debug)]
pub enum QuerySource {
    LikedSongs,
    Playlists,
    Albums,
}

#[derive(Debug)]
pub struct QuerySongsByArtist {
    /// The artist id.
    /// Can be found by searching for the artist on spotify and then looking at the url.
    /// https://open.spotify.com/artist/7gW0r5CkdEUMm42w9XpyZO
    pub artist_id: String,

    /// If true includes songs in which the artist is featured.
    /// If false only includes songs where the artist is the main artist.
    /// If None includes both.
    pub include_features: Option<bool>,

    /// If none includes all sources.
    pub source: Option<QuerySource>,

    /// If true, only includes songs that are liked by the user. (Part of the liked songs playlist)
    /// If false, only includes songs that are not liked by the user.
    /// If None, includes both.
    pub must_be_liked: Option<bool>,
}
