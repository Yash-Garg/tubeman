use atom_syndication::Feed;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct YtFeed {
    pub author: String,
    pub updated_at: DateTime<Utc>,
    pub uri: String,
    pub videos: Vec<YtVideo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct YtVideo {
    pub id: String,
    pub title: String,
    pub thumbnail: String,
    pub url: String,
    pub published: Option<DateTime<Utc>>,
}

impl From<Feed> for YtFeed {
    fn from(feed: Feed) -> Self {
        let author = feed.title().to_string();
        let updated = feed.updated().with_timezone(&Utc);
        let uri = feed.authors()[0].uri().unwrap_or_default().to_string();

        let videos = feed
            .entries()
            .iter()
            .map(|entry| {
                let id = entry.id().to_string();
                let title = entry.title().to_string();
                let url = entry.links()[0].href().to_string();

                let thumb = if let Some(media) = entry.extensions().get("media") {
                    let group = media.get("group").unwrap();
                    let thumb_group = group[0].children.get("thumbnail").unwrap();

                    Some(thumb_group[0].attrs.get("url").unwrap().to_string())
                } else {
                    None
                };

                let published = entry.published().map(|d| d.with_timezone(&Utc));

                YtVideo {
                    id,
                    title,
                    thumbnail: thumb.unwrap_or_default(),
                    url,
                    published,
                }
            })
            .collect();

        YtFeed {
            author,
            uri,
            updated_at: updated,
            videos,
        }
    }
}
