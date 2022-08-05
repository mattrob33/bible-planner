use std::fmt::{self, Formatter, Result};
use crate::bibleref_utils::{book_name, num_verses_in_chapter, num_chapters_in_book, num_verses_in_book};
use crate::bibleref_utils::BOOK_NAMES;
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};

pub struct BibleRef {
    pub book: i32,
    pub chapter: i32,
    pub verse: i32
}

impl BibleRef {
    pub fn plus(&self, num_verses: i32) -> Option<BibleRef> {
        let mut num_verses_remaining = num_verses;

        let mut book = self.book;
        let mut chapter = self.chapter;
        let mut verse = self.verse;

        while num_verses_remaining > 0 {
            let num_verses_in_chapter = num_verses_in_chapter(book, chapter);

            if verse + num_verses_remaining <= num_verses_in_chapter {
                verse += num_verses_remaining;
                num_verses_remaining = 0;
            }
            else {
                num_verses_remaining -= num_verses_in_chapter;

                if chapter < num_chapters_in_book(book) {
                    chapter += 1;
                    verse = 1;
                }
                else {
                    book += 1;
                    chapter = 1;
                    verse = 1;

                    if book > 66 { return None }
                }
            }
        }

        return Some(BibleRef { book, chapter, verse })
    }
}

impl Copy for BibleRef {}

impl Clone for BibleRef {
    fn clone(&self) -> Self {
        return BibleRef {
            book: self.book,
            chapter: self.chapter,
            verse: self.verse
        }
    }
}

impl Eq for BibleRef {}

impl PartialEq for BibleRef {
    fn eq(&self, other: &Self) -> bool {
        return self.book == other.book &&
            self.chapter == other.chapter &&
            self.verse == other.verse;
    }

    fn ne(&self, other: &Self) -> bool {
        return self.book != other.book ||
            self.chapter != other.chapter ||
            self.verse != other.verse;
    }
}

impl Ord for BibleRef {
    fn cmp(&self, other: &Self) -> Ordering {
        return if self.book == other.book {
            if self.chapter == other.chapter {
                if self.verse == other.verse {
                    Equal
                }
                else {
                    if self.verse > other.verse { Greater } else { Less }
                }
            }
            else {
                if self.chapter > other.chapter { Greater } else { Less }
            }
        }
        else {
            if self.book > other.book { Greater } else { Less }
        }
    }
}

impl PartialOrd for BibleRef {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl fmt::Display for BibleRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}:{}", BOOK_NAMES[(self.book - 1) as usize], self.chapter, self.verse)
    }
}

pub fn distance_between_refs(first: &BibleRef, second: &BibleRef) -> i32 {
    let mut distance = 0;

    if first.book == second.book {
        if first.chapter == second.chapter {
            distance = second.verse - first.verse
        }
        else {
            distance += num_verses_in_chapter(first.book, first.chapter) - first.verse;
            for chap in (first.chapter + 1)..second.chapter {
                distance += num_verses_in_chapter(first.book, chap);
            }
            distance += second.verse - 1;
        }
    }
    else {
        // Num verses to end of first ref's chapter
        distance += num_verses_in_chapter(first.book, first.chapter) - first.verse;

        // Verses to end of book
        for chap in (first.chapter + 1)..(num_chapters_in_book(first.book) + 1) {
            distance += num_verses_in_chapter(first.book, chap);
        }

        // Num verses in intervening books
        for book in (first.book + 1)..second.book {
            distance += num_verses_in_book(book);
        }

        // Num verses from beginning to current verse in second ref's chapter
        for chap in 1..second.chapter {
            distance += num_verses_in_chapter(second.book, chap);
        }

        // Num verses from beginning to current verse in second ref's chapter
        distance += second.verse - 1;
    }

    return distance;
}