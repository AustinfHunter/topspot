use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct Track {
    #[serde(rename(deserialize = "artist(s)_name"))]
    pub artist_name: String,
    pub track_name: String,
    pub artist_count: u8,
    #[serde(rename(deserialize = "released_year"))]
    pub release_year: u16,
    #[serde(rename(deserialize = "released_month"))]
    pub release_month: u8,
    #[serde(rename(deserialize = "released_day"))]
    pub release_day: u8,
    pub streams: u64,
    pub bpm: u16,
    #[serde(flatten)]
    pub feel: TrackFeel
}

#[derive(Debug,Serialize, Deserialize)]
pub struct TrackFeel { 
    #[serde(rename(deserialize = "danceability_%"))]
    pub danceability: u8,
    #[serde(rename(deserialize = "valence_%"))]
    pub valence: u8,
    #[serde(rename(deserialize = "energy_%"))]
    pub energy: u8,
    #[serde(rename(deserialize = "acousticness_%"))]
    pub acousticness: u8,
    #[serde(rename(deserialize = "instrumentalness_%"))]
    pub instrumentalness: u8,
    #[serde(rename(deserialize = "liveness_%"))]
    pub liveness: u8,
    #[serde(rename(deserialize = "speechiness_%"))]
    pub speechiness: u8
}

struct User<'a> {
    handle: String,
    taste: &'a TrackFeel
}
