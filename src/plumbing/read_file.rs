use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: String) -> String {
    let mut file = File::open(path).unwrap(); 
    let mut file_content = String::new();

    file.read_to_string(&mut file_content).unwrap();

    let file_content = file_content.replace("\r", "");

    file_content
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_file_content() {
        let file_content = read_file("./src/files_for_testing/a.txt".to_string());
        let content = String::from("Hello, world!\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit.\n\n");

        assert_eq!(file_content, content);
    }
}
