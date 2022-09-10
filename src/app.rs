use std::{
    cmp,
    env,
    fs::{
        self,
        DirEntry,
    },
    io,
    path::PathBuf,
};

use crate::list::*;

pub enum AppState {
    TagEditor
}

pub struct App {
    pub state: AppState,
    pub quit: bool,
    pub pwd: GenericList<PathBuf>,
}

impl App {
    pub fn default() -> Result<Self, io::Error> {
        let is_dir = |res: Result<DirEntry, io::Error>| {
            let file_type = res.as_ref().unwrap().file_type().unwrap();
            if file_type.is_dir() {
                Some(res.unwrap().path())
            } else {
                None
            }
        };

        let is_file = |res: Result<DirEntry, io::Error>| {
            let file_type = res.as_ref().unwrap().file_type().unwrap();
            if file_type.is_file() {
                Some(res.unwrap().path())
            } else {
                None
            }
        };

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
            state: AppState::TagEditor,
            quit: false,
            pwd: GenericList::<PathBuf>::from(pwd),
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
        let is_dir = |res: Result<DirEntry, io::Error>| {
            let file_type = res.as_ref().unwrap().file_type().unwrap();
            if file_type.is_dir() {
                Some(res.unwrap().path())
            } else {
                None
            }
        };

        let is_file = |res: Result<DirEntry, io::Error>| {
            let file_type = res.as_ref().unwrap().file_type().unwrap();
            if file_type.is_file() {
                Some(res.unwrap().path())
            } else {
                None
            }
        };

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
            AppState::TagEditor => {}
        }
    }

    pub fn list_down(&mut self) {
        match self.state {
            AppState::TagEditor => {
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
            AppState::TagEditor => {
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
            AppState::TagEditor => {
                if !self.pwd.is_empty() {
                    self.pwd.select(Some(0));
                }
            }
        }
    }

    pub fn jump_to_list_bottom(&mut self) {
        match self.state {
            AppState::TagEditor => {
                if !self.pwd.is_empty() {
                    let index = self.pwd.len() - 1;
                    self.pwd.select(Some(index));
                }
            }
        }
    }

    pub fn select(&mut self) -> Result<(), io::Error> {
        match self.state {
            AppState::TagEditor => {
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
}
