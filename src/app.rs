use id3::{
    Content,
    Tag,
};
use std::{
    cmp,
    collections::HashMap,
    env,
    error,
    fs::{
        self,
        DirEntry,
    },
    io,
    path::PathBuf,
};

use crate::list::*;

pub enum AppState {
    FileNavigation,
}

pub struct App {
    pub state: AppState,
    pub quit: bool,
    pub pwd: GenericList<PathBuf>,
    pub selected_files: HashMap<PathBuf, Option<Tag>>,
    pub tag_sum: HashMap<String, String>,
}

impl App {
    pub fn default() -> Result<Self, io::Error> {
        let mut pwd = vec![PathBuf::from("..")];
        let mut dirs = fs::read_dir(".")?
            .filter_map(|res| is_dir(res))
            .collect::<Vec<PathBuf>>();
        dirs.sort();
        pwd.append(&mut dirs);
        let mut files = fs::read_dir(".")?
            .filter_map(|res| is_file(res))
            .collect::<Vec<PathBuf>>();
        files.sort();
        pwd.append(&mut files);

        let app = Self {
            state: AppState::FileNavigation,
            quit: false,
            pwd: GenericList::<PathBuf>::from(pwd),
            selected_files: HashMap::new(),
            tag_sum: HashMap::new(),
        };

        Ok(app)
    }

    pub fn exit_dir(&mut self) -> Result<(), io::Error> {
        let mut pwd = env::current_dir()?;
        if pwd.pop() {
            env::set_current_dir(&pwd)?;
            self.refresh_pwd()?;
        }

        Ok(())
    }

    fn refresh_pwd(&mut self) -> Result<(), io::Error> {
        let mut pwd = vec![PathBuf::from("..")];
        let mut dirs = fs::read_dir(".")?
            .filter_map(|res| is_dir(res))
            .collect::<Vec<PathBuf>>();
        dirs.sort();
        pwd.append(&mut dirs);
        let mut files = fs::read_dir(".")?
            .filter_map(|res| is_file(res))
            .collect::<Vec<PathBuf>>();
        files.sort();
        pwd.append(&mut files);

        self.pwd = GenericList::<PathBuf>::from(pwd);

        Ok(())
    }

    pub fn on_tick(&mut self) {
        match self.state {
            AppState::FileNavigation => {}
        }
    }

    pub fn list_down(&mut self) {
        match self.state {
            AppState::FileNavigation => {
                if !self.pwd.is_empty() {
                    let index = self.pwd.get_index().unwrap();
                    let index = cmp::min(index + 1, self.pwd.len() - 1);
                    self.pwd.select(Some(index));
                }
            }
        }
    }

    pub fn list_up(&mut self) {
        match self.state {
            AppState::FileNavigation => {
                if !self.pwd.is_empty() {
                    let index = self.pwd.get_index().unwrap();
                    let index = index.checked_sub(1).unwrap_or(0);
                    self.pwd.select(Some(index));
                }
            }
        }
    }

    pub fn jump_to_list_top(&mut self) {
        match self.state {
            AppState::FileNavigation => {
                if !self.pwd.is_empty() {
                    self.pwd.select(Some(0));
                }
            }
        }
    }

    pub fn jump_to_list_bottom(&mut self) {
        match self.state {
            AppState::FileNavigation => {
                if !self.pwd.is_empty() {
                    let index = self.pwd.len() - 1;
                    self.pwd.select(Some(index));
                }
            }
        }
    }

    pub fn enter_dir(&mut self) -> Result<(), io::Error> {
        match self.state {
            AppState::FileNavigation => {
                if let Some(entry) = self.pwd.get_selected() {
                    if entry.is_dir() {
                        env::set_current_dir(&entry)?;
                        self.refresh_pwd()?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn select(&mut self) -> Result<(), Box<dyn error::Error>> {
        match self.state {
            AppState::FileNavigation => {
                if let Some(entry) = self.pwd.get_selected() {
                    if self.pwd.get_index().unwrap() != 0 {
                        self.toggle_select_path(entry)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn toggle_select_path(
        &mut self,
        path: PathBuf
    ) -> Result<(), Box<dyn error::Error>> {
        if let Some(_) = self.selected_files.get(&path) {
            self.selected_files.remove(&path);
        } else {
            if let Ok(tags) = Tag::read_from_path(path.clone()) {
                self.selected_files.insert(path.clone(), Some(tags));
            } else {
                self.selected_files.insert(path.clone(), None);
            }
        }

        if path.is_dir() {
            let mut entries = Vec::new();
            let mut dirs = fs::read_dir(path.clone())?
                .filter_map(|res| is_dir(res))
                .collect::<Vec<PathBuf>>();
            dirs.sort();
            entries.append(&mut dirs);
            let mut files = fs::read_dir(path)?
                .filter_map(|res| is_file(res))
                .collect::<Vec<PathBuf>>();
            files.sort();
            entries.append(&mut files);

            for entry in entries.iter() {
                self.toggle_select_path(entry.clone())?;
            }
        }

        self.update_tag_sum();

        Ok(())
    }

    pub fn update_tag_sum(&mut self) {
        let tags = self.selected_files
            .iter()
            .filter_map(|(k, v)| {
                if k.is_file() {
                    Some(v.clone().unwrap_or(Tag::new()))
                } else {
                    None
                }
            })
            .collect::<Vec<Tag>>();

        self.tag_sum.clear();
        for tag in tags {
            for frame in tag.frames() {
                let tag_value = match frame.content() {
                    // TODO: Handle 'TXXX' frames (Custom frame data)
                    // TODO: Handle 'USLT' frame (Unsynced Lyrics)
                    // TODO: TCON (Genre) strips out slashes, which is not ideal
                    Content::Text(text) => text.to_string(),
                    Content::Comment(comment) => comment.text.to_string(),
                    Content::Picture(picture) => {
                        // TODO: Handle empty picture description
                        format!("{} <{}>", picture.description, picture.mime_type)
                    },
                    _ => "<other>".to_string()
                };

                self.add_to_tag_sum(frame.id().to_string(), tag_value);
            }
        }
    }

    fn add_to_tag_sum(&mut self, key: String, value: String) {
        if self.tag_sum.contains_key(&key) {
            // Ignore the case where the values are the same
            if self.tag_sum.get(&key).unwrap() != &value {
                self.tag_sum.insert(key, "<multiple>".to_string());
            }
        } else {
            self.tag_sum.insert(key, value);
        }
    }

    pub fn num_selected_files(&self) -> usize {
        let count = self.selected_files
            .iter()
            .filter_map(|(path, _)| {
                if path.is_file() {
                    Some(1)
                } else {
                    None
                }
            })
            .sum::<usize>();

        count
    }

    pub fn clear_selected_files(&mut self) {
        self.selected_files.clear();
        self.update_tag_sum();
    }
}

fn is_dir(res: Result<DirEntry, io::Error>) -> Option<PathBuf> {
    let entry = res.as_ref().unwrap();
    let file_type = entry.file_type().unwrap();
    if file_type.is_dir() {
        Some(entry.path().canonicalize().unwrap())
    } else {
        None
    }
}

fn is_file(res: Result<DirEntry, io::Error>) -> Option<PathBuf> {
    let entry = res.as_ref().unwrap();
    let file_type = entry.file_type().unwrap();
    if file_type.is_file() {
        // Only accept mp3 files
        // TODO: In the future, we want to accept any filetype that can use id3 tags
        if let Some(extension) = entry.path().extension() {
            if extension == "mp3" {
                Some(entry.path().canonicalize().unwrap())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
