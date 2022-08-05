extern crate substring;

use substring::Substring;

use crate::bibleref::BibleRef;
use crate::biblerefrange::BibleRefRange;
use crate::pericopes::Pericope;
use crate::str_utils::read_lines;

const INPUT_FILE: &str = "input/esv_pericope-starts-ends.txt";

pub fn parse_pericope_data() -> Vec<Pericope> {
    let mut pericopes: Vec<Pericope> = vec!();

    if let Ok(lines) = read_lines(INPUT_FILE) {
        for line in lines {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split("-").collect();
                let parts2: Vec<&str> = parts[1].split(" => ").collect();

                let start = parse_ref(parts[0]);
                let end = parse_ref(parts2[0]);
                let title = String::from(parts2[1]);

                let range = BibleRefRange { start, end };

                let pericope = Pericope { range, title };
                pericopes.push(pericope);
            }
        }
    }
    return pericopes;
}

fn parse_ref(from: &str) -> BibleRef {
    let book_string = from.substring(0, 2);
    let chap_string = from.substring(3, 5);
    let verse_string = from.substring(6, 9);

    let book: i32 = book_string.parse().expect("Invalid book");
    let chapter: i32 = chap_string.parse().expect("Invalid chapter");
    let verse: i32 = verse_string.parse().expect("Invalid verse");

    return BibleRef { book, chapter, verse }
}