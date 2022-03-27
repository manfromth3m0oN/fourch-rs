use serde::{Deserialize, Serialize};

/// Represents a thread
#[derive(Serialize, Deserialize, Debug)]
pub struct Thread {
    pub posts: Vec<Post>,
}

impl Thread {
    /// Get a thread by ID
    /// ```
    /// let thread = Thread::from_id("pol", "22310966")
    /// ````
    pub async fn from_id(
        client: reqwest::Client,
        board: &str,
        thread_id: &str,
    ) -> Result<Thread, reqwest::Error> {
        client
            .get(format!("a.4cdn.org/{}/thread/{}", board, thread_id))
            .send()
            .await?
            .json::<Thread>()
            .await
    }
}

/// Represents all the possible capcodes
#[derive(Serialize, Deserialize, Debug)]
pub enum Capcode {
    Mod,
    Admin,
    AdminHightlight,
    Manager,
    Developer,
    Founder,
}

/// Represents a single post in a thread
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    /// The numeric post ID
    #[serde(rename = "no")]
    pub post_id: u16,
    /// The post being replied to in the case of a non OP
    #[serde(rename = "resto")]
    pub res_to: Option<u16>,
    /// If the thread is pinned as a sticky, OP only
    pub sticky: Option<u8>,
    /// If the thread is closed to replies
    pub closed: Option<u8>,
    /// MM/DD/YY(Day)HH:MM (:SS on some boards), EST/EDT timezone
    pub now: String,
    /// UNIX timestamp of post creation
    pub time: u16,
    /// Name of the user that posted
    pub name: String,
    /// The tripcode of the post
    #[serde(rename = "trip")]
    pub tripcode: Option<String>,
    /// The poster's ID
    pub id: Option<String>,
    /// The capcode (capability code?) of the poster e.g. Not set, `mod`, `admin`, `admin_highlight`, `manager`, `developer`, `founder`
    pub capcode: Option<Capcode>,
    /// The country of poster in ISO 3166 format
    pub country: Option<String>,
    /// The full string name of the poster's country
    pub country_name: Option<String>,
    /// The poster's board flag code
    pub board_flag: Option<String>,
    /// The poster's board flag name
    pub flag_name: Option<String>,
    /// The thread subject, only present on the OP's post
    #[serde(rename = "sub")]
    pub subject: Option<String>,
    /// The HTML escaped string of the poster's comment
    #[serde(rename = "com")]
    pub comment: Option<String>,
    /// The UNIX time plus the microtime the image attached to the post was uploaded
    #[serde(rename = "tim")]
    pub upload_time: Option<u16>,
    /// The filename of the image as it appeared on the uploader's device
    pub filename: Option<String>,
    /// The extension of the uploaded file
    #[serde(rename = "ext")]
    pub upload_ext: Option<String>,
    /// The size of the uploaded file in bytes
    #[serde(rename = "fsize")]
    pub upload_size: Option<u16>,
    /// The MD5 hash of the file
    #[serde(rename = "md5")]
    pub file_hash: Option<String>,
    /// Width of the uploaded image
    #[serde(rename = "w")]
    pub width: Option<u16>,
    /// The height of the uploaded image
    #[serde(rename = "h")]
    pub height: Option<u16>,
    /// The width of the thumbnail
    #[serde(rename = "tn_w")]
    pub thumbnail_height: Option<u16>,
    /// The height of thethumbnail
    #[serde(rename = "th_h")]
    pub thumbnail_width: Option<u16>,
    /// If the file has been deleted
    #[serde(rename = "filedeleted")]
    pub file_deleted: Option<u8>,
    /// If the image is spoilered
    pub spoiler: Option<u8>,
    /// Custom spoiler ID
    pub custom_spoiler: Option<u8>,
    /// The total number of replies to a thread, OP only
    pub replies: Option<u16>,
    /// The total number of images in a thread, OP only
    pub images: Option<u16>,
    /// The bump limit of a thread, OP only
    #[serde(rename = "bumplimit")]
    pub bump_limit: Option<u16>,
    /// The image limit of a thread, OP only
    #[serde(rename = "imagelimit")]
    pub image_limit: Option<u16>,
    /// The cagegory of flash (`.swf`) upload, OP only, /f/ only
    pub tag: Option<String>,
    /// The SEO URL slug for a thread, OP only
    pub semantic_url: Option<String>,
    /// The year a user's 4chan pass was bought
    #[serde(rename = "since4pass")]
    pub since_4pass: Option<u16>,
    /// The number of unique IPs that have posted in a thread, OP only
    pub unique_ips: Option<u16>,
    /// If a mobile optimized image exists for a post
    #[serde(rename = "m_img")]
    pub mobile_img: Option<u8>,
    /// Weather a thrad is archived or not, OP only
    pub archived: Option<u8>,
    /// The date the thread was archived on, UNIX timestamp, OP only
    pub archived_on: Option<u32>,
}
