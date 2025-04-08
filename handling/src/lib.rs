use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;
// open the file or create file if it doesn't exist
// append the content to file
// should panic if error is expected
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .expect("Failed to open file");

    writeln!(file, "{content}").expect("failed to write");
}

