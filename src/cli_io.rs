use ascii_table::*;
use plugin_manager::query::QueryResult;
use std::{ops::Range, vec};

/// Possible choices for main menu
pub enum Action {
    Search,
    List,
    Listen,
    ClearCache,
    ClearDownload,
    Quit,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Action::Search => write!(f, "Search a song"),
            Action::List => write!(f, "List downloaded songs"),
            Action::Listen => write!(f, "Listen to a song"),
            Action::ClearCache => write!(f, "Clear cache directory"),
            Action::ClearDownload => write!(f, "Clear download directory"),
            Action::Quit => write!(f, "Quit"),
        }
    }
}

impl Action {
    /// Returns a vector holdin all the possible actions
    pub fn items() -> Vec<Action> {
        vec![
            Action::Search,
            Action::List,
            Action::Listen,
            Action::ClearCache,
            Action::ClearDownload,
            Action::Quit,
        ]
    }
}

/// Holds the pieces of information needed to print
/// results of a query or a listing
pub struct PrintData {
    header: Vec<String>,
    data: Vec<Vec<String>>,
    rows: Range<usize>,
}

impl PrintData {
    /// Prints the provided data
    pub fn print(&self) {
        let mut table = AsciiTable::default();
        for (index, item) in self.header.iter().enumerate() {
            table.column(index).set_header(item).set_align(Align::Left);
        }

        let terminal_size = match terminal_size::terminal_size() {
            Some((terminal_size::Width(w), _)) => w,
            None => 150,
        } as usize;

        table.set_max_width(terminal_size);
        table.print(&self.data);
    }

    /// Returns the range of id's associated to the displayable records
    pub fn rows(&self) -> &Range<usize> {
        &self.rows
    }
}

/// A trait that can be implemented on types representing a
/// result of a quaery or a listing
pub trait Print {
    /// Constructs a `PrintData` instance starting
    /// from `&self`
    fn prepare(&self) -> PrintData;
}

impl Print for QueryResult {
    fn prepare(&self) -> PrintData {
        let header = vec!["Index", "Track name", "Artist name", "Url"]
            .iter()
            .map(|item| item.to_string())
            .collect();

        let mut data: Vec<Vec<String>> = Vec::new();
        for (index, item) in self.data().iter().enumerate() {
            let track_name: String = item.track_name().into();
            let artist_name: String = item.artist_name().into();
            let url: String = item.track_url().to_string();

            let mut row = Vec::new();
            row.push(index.to_string());
            row.push(track_name);
            row.push(artist_name);
            row.push(url);
            data.push(row);
        }

        let rows = 0..self.data().len();
        PrintData { header, data, rows }
    }
}

impl Print for Vec<std::io::Result<std::fs::DirEntry>> {
    fn prepare(&self) -> PrintData {
        let header = vec!["Index", "Track name", "Artist name", "Path"]
            .iter()
            .map(|item| item.to_string())
            .collect();

        let regex = regex::Regex::new(r#"(.*)--(.*)\..*$"#).unwrap();
        let mut data: Vec<Vec<String>> = vec![];
        for (index, item) in self.iter().enumerate() {
            let item = item.as_ref().unwrap();
            let full_name = item.file_name();
            let full_name = full_name.to_str().unwrap();

            let parts = regex.captures(full_name);
            if let None = parts {
                continue;
            }
            let parts = parts.unwrap();
            let track_name = match parts.get(1) {
                Some(track_name) => track_name.as_str(),
                None => "",
            }
            .trim()
            .to_string();
            let artist_name = match parts.get(2) {
                Some(artist_name) => artist_name.as_str(),
                None => "",
            }
            .trim()
            .to_string();

            let path = item.path().to_str().unwrap().to_string();

            let row = vec![index.to_string(), track_name, artist_name, path];
            data.push(row);
        }

        let rows = 0..data.len();
        PrintData { header, data, rows }
    }
}
