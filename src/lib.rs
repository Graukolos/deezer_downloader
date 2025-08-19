//! deezer downloader provides basic functionality needed to download a song from [deezer](http://deezer.com)

pub mod downloader;
pub mod error;
pub mod playlist;
pub mod song;

pub use downloader::Downloader;
pub use playlist::Playlist;
pub use song::{Song, SongMetadata};

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::song::Song;

    use super::downloader::Downloader;

    #[tokio::test]
    async fn download_song_by_id() -> Result<(), Box<dyn Error>> {
        let downloader = Downloader::new().await?;
        let song = Song::download(92719900, &downloader).await?;
        song.write_to_file(format!("output/{}.mp3", song.metadata.id))?;
        Ok(())
    }
}
