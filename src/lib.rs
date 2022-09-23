use std::io::BufRead;

pub fn find_matches<R: BufRead>(content: R, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        let l = line.expect("Unable to read line");
        if l.contains(pattern) {
            writeln!(writer, "{}", l);
        }
    }
}
