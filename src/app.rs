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
    FileNavigation,
}

pub struct App {
    pub state: AppState,
    pub quit: bool,
    pub pwd: GenericList<PathBuf>,
    pub selected_files: Vec<PathBuf>,
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
            selected_files: Vec::new(),
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

    pub fn select(&mut self) -> Result<(), io::Error> {
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
    ) -> Result<(), io::Error> {
        if let Some(index) = self.selected_files
            .iter()
            .position(|p| p == &path)
        {
            self.selected_files.remove(index);
        } else {
            self.selected_files.push(path.clone());
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

        Ok(())
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
