use std::fmt::{self, Formatter, Result};
use crate::bibleref::BibleRef;
use crate::bibleref_utils::{book_name, BOOK_NAMES};
use crate::bibleref::distance_between_refs;

pub struct BibleRefRange {
    pub start: BibleRef,
    pub end: BibleRef
}

impl Copy for BibleRefRange {}

impl Clone for BibleRefRange {
    fn clone(&self) -> Self {
        return BibleRefRange {
            start: self.start.clone(),
            end: self.end.clone()
        }
    }
}


impl fmt::Display for BibleRefRange {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        if self.start.book == self.end.book {
            if self.start.chapter == self.end.chapter {
                if self.start.verse == self.end.verse {
                    write!(formatter, "{}", self.start)
                }
                else {
                    write!(
                        formatter,
                        "{} {}:{}-{}",
                        book_name(self.start.book), self.start.chapter, self.start.verse, self.end.verse
                    )
                }
            }
            else {
                write!(
                    formatter,
                    "{} {}:{} - {}:{}",
                    book_name(self.start.book), self.start.chapter, self.start.verse,
                    self.end.chapter, self.end.verse
                )
            }
        }
        else {
            write!(
                formatter,
                "{} {}:{} - {} {}:{}",
                book_name(self.start.book), self.start.chapter, self.start.verse,
                book_name(self.end.book), self.end.chapter, self.end.verse
            )
        }
    }
}

impl BibleRefRange {
    pub fn num_verses(&self) -> i32 {
        return distance_between_refs(&self.start, &self.end) + 1;
    }
}