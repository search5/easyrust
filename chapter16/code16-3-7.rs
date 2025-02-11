pub fn create<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new().write(true).create(true).truncate(true).open(path.as_ref())
}
