use std::fmt::{self, Display, Formatter, Result};

use crate::biblerefrange::BibleRefRange;
use crate::parser::parse_pericope_data;
use crate::pericopes::Pericope;

pub struct Plan {
    pub title: String,
    pub range: BibleRefRange,
    pub num_days: i32
}

impl Plan {
    pub fn num_verses(&self) -> i32 {
        return self.range.num_verses();
    }
}

impl Display for Plan {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{} ({})", self.title, self.range)
    }
}

pub fn get_pericopes_for_plan(plan: &Plan) -> Vec<Pericope> {
    let plan_start = plan.range.start;
    let plan_end = plan.range.end;
    let all_pericopes = &parse_pericope_data();

    let mut pericopes: Vec<Pericope> = vec![];

    for pericope in all_pericopes {
        let pericope_start = pericope.range.start;
        let pericope_end = pericope.range.end;

        if pericope_start >= plan_start && pericope_end <= plan_end {
            pericopes.push(pericope.clone());
        }
    }

    return pericopes;
}