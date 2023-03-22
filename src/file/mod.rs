use crate::pre::*;
use comp::Text;
use reqwest::multipart::{Form, Part};
use std::{fs, marker::PhantomData};

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
    pub fn url(&self) -> &str {
        &self.url_private
    }

    pub fn url_download(&self) -> &str {
        &self.url_private_download
    }

    pub fn permalink(&self) -> &str {
        &self.permalink
    }

    pub fn permalink_public(&self) -> &str {
        &self.permalink_public
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_public_url(&self) -> String {
        let filename = self.name().to_lowercase();
        let url = url::Url::parse(self.permalink_public()).unwrap();
        let segments: Vec<&str> = url
            .path_segments()
            .unwrap()
            .next()
            .unwrap()
            .split('-')
            .collect();

        let (team_id, file_id, pub_secret) = (
            // Suppress clippy warning for using "get(0)" to get the first element
            #[allow(clippy::get_first)]
            segments.get(0).unwrap(),
            segments.get(1).unwrap(),
            segments.get(2).unwrap(),
        );

        format!("https://files.slack.com/files-pri/{team_id}-{file_id}/{filename}?pub_secret={pub_secret}")
    }

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

impl Upload {
    pub fn from_path<P: std::convert::AsRef<std::path::Path>>(
        path: P,
    ) -> BoltResult<Upload<File>> {
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

    pub fn from_bytes(bytes: Vec<u8>) -> Upload<File> {
        Upload::<File> {
            c: PhantomData::<File>,
            file: Some(bytes),
            ..Default::default()
        }
    }

    pub fn from_text(text: &str) -> Upload<Text> {
        Upload::<Text> {
            c: PhantomData::<Text>,
            content: Some(text.to_string()),
            ..Default::default()
        }
    }
}

impl<C> Upload<C> {
    pub fn channels(mut self, channels: Vec<&str>) -> Self {
        self.channels = Some(channels.join(","));
        self
    }

    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_string());
        self
    }

    pub fn filetype(mut self, filetype: &str) -> Self {
        self.filetype = Some(filetype.to_string());
        self
    }

    pub fn initial_comment(mut self, comment: &str) -> Self {
        self.initial_comment = Some(comment.to_string());
        self
    }

    pub fn thread_ts(mut self, thread_ts: &str) -> Self {
        self.thread_ts = Some(thread_ts.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

impl Upload<Text> {
    pub fn file(self, file: Vec<u8>) -> Upload<File> {
        Upload::<File> {
            c: PhantomData::<File>,
            file: Some(file),
            ..Default::default()
        }
    }
}

impl Upload<File> {
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
