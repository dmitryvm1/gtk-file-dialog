
use std::path::Path;
use std::path::PathBuf;

use raw_window_handle::RawWindowHandle;
use raw_window_handle::HasWindowHandle;

#[derive(Debug, Clone)]
pub(crate) struct Filter {
    #[allow(dead_code)]
    pub name: String,
    pub extensions: Vec<String>,
}

#[derive(Default, Debug, Clone)]
pub struct FileDialog {
    pub(crate) filters: Vec<Filter>,
    pub(crate) starting_directory: Option<PathBuf>,
    pub(crate) file_name: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) parent: Option<RawWindowHandle>,
}


impl FileDialog {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_filter(mut self, name: impl Into<String>, extensions: &[impl ToString]) -> Self {
        self.filters.push(Filter {
            name: name.into(),
            extensions: extensions.iter().map(|e| e.to_string()).collect(),
        });
        self
    }

    pub fn set_directory<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path = path.as_ref();
        if path.to_str().map(|p| p.is_empty()).unwrap_or(false) {
            self.starting_directory = None;
        } else {
            self.starting_directory = Some(path.into());
        }
        self
    }

    pub fn set_file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn set_parent<W: HasWindowHandle>(mut self, parent: &W) -> Self {
        self.parent = parent.window_handle().ok().map(|x| x.as_raw());
        self
    }
}
