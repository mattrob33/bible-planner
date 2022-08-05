use std::fmt::{self, Formatter, Result};

use crate::biblerefrange::BibleRefRange;

pub struct Pericope {
    pub range: BibleRefRange,
    pub title: String
}

impl fmt::Display for Pericope {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{} ({})", self.title, self.range)
    }
}

impl Clone for Pericope {
    fn clone(&self) -> Self {
        return Pericope {
            range: self.range.clone(),
            title: self.title.clone()
        }
    }
}