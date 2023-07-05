use crate::pre::{comp, BoltResult, Deserialize, Error, Request, Serialize};
use comp::Text;
use reqwest::multipart::{Form, Part};
use std::{fs, marker::PhantomData};

/// A [File] originating from Slack
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct File {
    id: String,
    name: String,
    url_private: String,
    url_private_download: String,
    permalink: String,
    permalink_public: String,
}

impl File {
    /// Returns the `url_private` property
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url_private
    }

    /// Returns the `url_private_download` property
    #[must_use]
    pub fn url_download(&self) -> &str {
        &self.url_private_download
    }

    /// Returns the `permalink` property
    #[must_use]
    pub fn permalink(&self) -> &str {
        &self.permalink
    }

    /// Returns the `permalink_public` property
    #[must_use]
    pub fn permalink_public(&self) -> &str {
        &self.permalink_public
    }

    /// Returns the slack-file-id
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the name of the file
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Uses alreaady known URL's to construct the public url to send as a message in slack.
    /// This URL will **only** work if the file is made public in slack. This can be done by using the
    /// `publish` method.
    #[must_use]
    pub fn get_public_url(&self) -> String {
        let filename = self.name().to_lowercase();
        let url = url::Url::parse(self.permalink_public()).expect("Public permalink unparseable!?");
        let segments: Vec<&str> = url
            .path_segments()
            .expect("No path segments on public permalink?!")
            .next()
            .expect("Only one segment on public permalink?!")
            .split('-')
            .collect();

        let (team_id, file_id, pub_secret) = (
            // Suppress clippy warning for using "get(0)" to get the first element
            #[allow(clippy::get_first)]
            segments
                .get(0)
                .expect("Failed to extract team-id from public permalink"),
            segments
                .get(1)
                .expect("Failed to extract file-id from public permalink"),
            segments
                .get(2)
                .expect("Failed to extract pub-secret from public permalink"),
        );

        format!("https://files.slack.com/files-pri/{team_id}-{file_id}/{filename}?pub_secret={pub_secret}")
    }

    /// Publishes the file for "public" consumption in slack.
    ///
    /// **Disclaimer**: This is needed for the `get_public_url` method to work,
    /// if the file is not already available to the public.
    ///
    /// # Errors
    ///
    /// An error will occur if the request fails to be sent.
    ///
    pub async fn publish(self, token: &str) -> BoltResult<Self> {
        let form = Form::new()
            .text("token", token.to_owned())
            .text("file", self.id);

        Request::post("files.sharedPublicURL", token)
            .multipart(form)
            .send()
            .await?
            .unpack()
    }
}

/// A file-upload to slack.
#[derive(Default, Debug)]
pub struct Upload<C = Text> {
    c: PhantomData<C>,

    channels: Option<String>,
    content: Option<String>,
    file: Option<Vec<u8>>,
    filename: Option<String>,
    filetype: Option<String>,
    initial_comment: Option<String>,
    thread_ts: Option<String>,
    title: Option<String>,
}

/// An upload-payload constructor for uploading files to slack.
impl Upload {
    /// Load a file from a path to be uploaded to slack
    ///
    /// # Errors
    ///
    /// Errors will occur if:
    /// * A file is not found on the path specified
    /// * The name of the file can't be determined
    ///
    pub fn from_path<P: std::convert::AsRef<std::path::Path>>(path: P) -> BoltResult<Upload<File>> {
        let file = match fs::read(&path) {
            Ok(file) => file,
            Err(error) => return Err(Error::File(error.to_string())),
        };
        let filename = match path.as_ref().file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => return Err(Error::File("Couldn't determine file name.".to_string())),
        };
        Ok(Self::from_bytes(file).filename(&filename))
    }

    /// Load a file from bytes to be uploaded to slack
    #[must_use]
    pub fn from_bytes(bytes: Vec<u8>) -> Upload<File> {
        Upload::<File> {
            c: PhantomData::<File>,
            file: Some(bytes),
            ..Default::default()
        }
    }

    /// Create a file from plaintext to be uploaded to slack
    #[must_use]
    pub fn from_text(text: &str) -> Self {
        Self {
            c: PhantomData::<Text>,
            content: Some(text.to_string()),
            ..Default::default()
        }
    }
}

impl<C: Send + Sync> Upload<C> {
    /// Sets the channels that the file should be sent to after uploading as a message.
    #[must_use]
    pub fn channels(mut self, channels: &[&str]) -> Self {
        self.channels = Some(channels.join(","));
        self
    }

    /// Sets the filename (Automatically set with the `from_path` or `from_bytes` methods)
    #[must_use]
    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }

    /// Sets the filetype
    #[must_use]
    pub fn filetype(mut self, filetype: &str) -> Self {
        self.filetype = Some(filetype.to_string());
        self
    }

    /// Upload the file with a comment
    #[must_use]
    pub fn initial_comment(mut self, comment: &str) -> Self {
        self.initial_comment = Some(comment.to_string());
        self
    }

    /// Sets the thread to send the uploaded file to as a message.
    #[must_use]
    pub fn thread_ts(mut self, thread_ts: &str) -> Self {
        self.thread_ts = Some(thread_ts.to_string());
        self
    }

    /// A title for the file - This differs from the file-name.
    #[must_use]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

        /// Uploads the content to slack, returning the file-object.
    ///
    /// # Errors
    ///
    /// An error will occur if the request can't be sent
    ///
    pub async fn upload(self, token: &str) -> BoltResult<File> {
        let mut form = Form::new().text("token", token.to_owned());

        if let Some(channels) = self.channels {
            form = form.text("channels", channels);
        }
        if let Some(content) = self.content {
            form = form.text("content", content);
        }
        if let Some(file) = self.file {
            form = form.part("content", Part::bytes(file));
        }
        if let Some(filename) = self.filename {
            form = form.text("filename", filename);
        }
        if let Some(filetype) = self.filetype {
            form = form.text("filetype", filetype);
        }
        if let Some(comment) = self.initial_comment {
            form = form.text("initial_comment", comment);
        }
        if let Some(thread_ts) = self.thread_ts {
            form = form.text("thread_ts", thread_ts);
        }
        if let Some(title) = self.title {
            form = form.text("title", title);
        }

        Request::post("files.upload", token)
            .multipart(form)
            .send::<File>()
            .await?
            .unpack()
    }
}
