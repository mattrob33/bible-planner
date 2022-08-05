#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

extern crate substring;

use std::env;
use substring::Substring;
use bibleref::{distance_between_refs, BibleRef};
use parser::parse_pericope_data;
use pericopes::Pericope;
use printer::print_report;
use plans::{Plan, get_pericopes_for_plan};
use biblerefrange::BibleRefRange;
use bibleref_utils::{num_chapters_in_book};

pub mod str_utils;
pub mod parser;
pub mod pericopes;
pub mod bibleref;
pub mod biblerefrange;
pub mod bibleref_utils;
mod printer;
mod plans;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (title, scope, num_days) = parse_cli_args(&args);

    let range = map_scope_to_range(&scope);

    let plan = Plan { title, range, num_days };

    let schedule = build_optimal_reading_schedule(&plan);
    let schedule = greedy_build_reading_schedule(&plan);

    for day in 0..schedule.len() {
        println!("Day {}: {}", day + 1, schedule[day]);
    }
}

fn parse_cli_args(args: &Vec<String>) -> (String, String, i32) {

    let stack_string = "test";
    let stack_string = "test".to_string().as_str();


    let mut title = "Custom Bible Plan".to_string();
    let mut scope = "BIBLE".to_string();
    let mut num_days = 365;

    for arg in args {
        if arg.starts_with("title") {
            title = arg.substring(6, arg.len()).to_string();
        }
        else if arg.starts_with("scope") {
            scope = arg.substring(6, arg.len()).to_string();
        }
        else if arg.starts_with("days") {
            let days = arg.substring(5, arg.len()).to_string();
            num_days = days.parse::<i32>().unwrap();
        }
    }

    return (title, scope, num_days);
}

fn map_scope_to_range(scope: &str) -> BibleRefRange {
    return match scope.to_uppercase().as_str() {

        "NT" => BibleRefRange {
            start: BibleRef { book: 40, chapter: 1, verse: 1 },
            end: BibleRef { book: 66, chapter: 22, verse: 21 }
        },

        "OT" => BibleRefRange {
            start: BibleRef { book: 1, chapter: 1, verse: 1 },
            end: BibleRef { book: 39, chapter: 4, verse: 6 }
        },

        "BIBLE"|"ALL" => BibleRefRange {
            start: BibleRef { book: 40, chapter: 1, verse: 1 },
            end: BibleRef { book: 66, chapter: 22, verse: 21 }
        },

        _ => BibleRefRange {
            start: BibleRef { book: 1, chapter: 1, verse: 1 },
            end: BibleRef { book: 66, chapter: 22, verse: 21 }
        }
    }
}

fn build_test_plan() {
    let plan = Plan {
        title: String::from("NT in a Year"),
        range: BibleRefRange {
            start: BibleRef { book: 40, chapter: 1, verse: 1 },
            end: BibleRef { book: 66, chapter: 22, verse: 21 }
        },
        num_days: 365
    };

    let schedule = build_optimal_reading_schedule(&plan);

    for day in 0..schedule.len() {
        println!("Day {}: {}", day + 1, schedule[day]);
    }
}

fn build_optimal_reading_schedule(plan: &Plan) -> Vec<BibleRefRange> {
    let pericopes = get_pericopes_for_plan(&plan);

    let num_pericopes = pericopes.len() as i32;
    let num_verses = plan.num_verses();

    let avg = (f64::from(num_verses) / f64::from(plan.num_days));
    let min = (avg * 0.5).ceil() as i32;
    let max = (avg * 1.5).ceil() as i32;

    let mut optimal_schedule = build_reading_schedule(plan, &pericopes, min);
    let mut num_days_in_optimal_schedule = optimal_schedule.len() as i32;
    let mut optimal_schedule_days_diff = (plan.num_days - num_days_in_optimal_schedule).abs();

    for target in (min + 1)..(max + 1) {
        let schedule = build_reading_schedule(plan, &pericopes, target);
        let num_days_in_schedule = schedule.len() as i32;
        let schedule_days_diff = (plan.num_days - num_days_in_schedule).abs();

        if schedule_days_diff < optimal_schedule_days_diff {
            optimal_schedule = schedule;
            num_days_in_optimal_schedule = num_days_in_schedule;
            optimal_schedule_days_diff = schedule_days_diff;
        }
    }

    return optimal_schedule;
}

fn greedy_build_reading_schedule(plan: &Plan) -> Vec<BibleRefRange> {
    let pericopes = get_pericopes_for_plan(&plan);

    let num_pericopes = pericopes.len() as i32;
    let num_verses = plan.num_verses();

    let avg = (f64::from(num_verses) / f64::from(plan.num_days));
    let min = (avg * 0.5).ceil() as i32;
    let max = (avg * 1.5).ceil() as i32;

    let daily_target = avg as i32 - 1;

    return build_reading_schedule(plan, &pericopes, daily_target);
}

fn build_reading_schedule(plan: &Plan, pericopes: &Vec<Pericope>, daily_target: i32) -> Vec<BibleRefRange> {
    let mut next_pericope_index = 0;
    let mut next_pericope = pericopes[next_pericope_index].clone();

    let mut cur_verse = plan.range.start.clone();

    let mut schedule: Vec<BibleRefRange> = vec![];

    for day in 0..plan.num_days {
        let maybe_target_ref = cur_verse.plus(daily_target);

        if maybe_target_ref.is_none() {
            let start_ref = cur_verse;
            let end_ref = plan.range.end;

            let reading = BibleRefRange {
                start: start_ref,
                end: end_ref
            };

            schedule.push(reading);

            break
        }
        let target_ref = maybe_target_ref.unwrap();

        let next_pericope_end = next_pericope.range.end;

        if next_pericope_end == target_ref {
            schedule.push(next_pericope.range.clone());
            let maybe_cur_verse = target_ref.plus(1);
            if maybe_cur_verse.is_none() {
                break
            }
            cur_verse = maybe_cur_verse.unwrap();
        }
        else {
            let mut test_index = next_pericope_index;
            let mut candidate_pericope = pericopes[test_index].clone();

            while candidate_pericope.range.end < target_ref {
                test_index += 1;
                candidate_pericope = pericopes[test_index].clone();
            }

            let earlier_candidate_pericope = if test_index > 0 {
                pericopes[test_index - 1].clone()
            }
            else {
                candidate_pericope.clone()
            };

            let earlier_diff = distance_between_refs(&target_ref, &earlier_candidate_pericope.range.end).abs();
            let candidate_diff = distance_between_refs(&target_ref, &candidate_pericope.range.end).abs();

            let mut selection_index = if earlier_diff < candidate_diff {
                test_index - 1
            }
            else {
                test_index
            };

            let selection: Pericope = if earlier_candidate_pericope.range.start.book == candidate_pericope.range.start.book {
                pericopes[selection_index].clone()
            }
            else {
                selection_index -= 1;
                earlier_candidate_pericope.clone()
            };

            let start_ref = pericopes[next_pericope_index].range.start;
            let end_ref = selection.range.end;

            let reading = BibleRefRange {
                start: start_ref,
                end: end_ref
            };

            schedule.push(reading);

            let maybe_cur_verse = reading.end.plus(1);
            if maybe_cur_verse.is_none() {
                break
            }
            cur_verse = maybe_cur_verse.unwrap();
            next_pericope_index = selection_index + 1;
        }
    }

    return schedule;
}

fn print_plan_summary(plan: &Plan) {
    let pericopes = get_pericopes_for_plan(&plan);
    let num_pericopes = pericopes.len() as i32;

    let num_verses = plan.num_verses();
    let avg: f32 = (f64::from(num_verses) / f64::from(plan.num_days)) as f32;
    let min = (avg * 0.5).ceil() as i32;
    let max = (avg * 1.5).ceil() as i32;

    for pericope in pericopes {
        println!("{}", pericope);
    }

    println!();
    println!("The plan has {} pericopes and {} verses.", num_pericopes, num_verses);
    println!("The plan will average {:0.1} verses per day, with a max/min of {:0} – {:0}.", avg, min, max);
}