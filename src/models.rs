use chrono::datetime::DateTime;
use chrono::naive::time;
use chrono::offset::utc::UTC;
use uuid::Uuid;

pub struct Post {
    title: String,
    body: String,
    author: String,
    datetime: DateTime<UTC>,
    uuid: Uuid,
}

impl Post {
    pub fn new(
        title: &str,
        body: &str,
        author: &str,
        datetime: DateTime<UTC>,
        uuid: Uuid,
    ) -> Post {
        Post {            
            title: title.to_string(),
            body: body.to_string(),
            author: author.to_string(),
            datetime,
            uuid,
        }
    }
}