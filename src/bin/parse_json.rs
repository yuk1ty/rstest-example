#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use rstest::rstest;

    #[rstest]
    fn should_read_file(#[files("files/*.json")] path: PathBuf) {
        assert!(File::open(path).is_ok());
    }
}

fn main() {}
