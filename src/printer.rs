use crate::parser::parse_pericope_data;
use crate::pericopes::Pericope;
use crate::bibleref_utils::num_verses_in_chapter;

pub fn print_json(pericopes: &Vec<Pericope>) {
    println!("[");

    for pericope in pericopes {
        let num_verses = pericope_size(&pericope);
        println!("  {{");
        println!("    \"title\": \"{}\"", pericope.title);
        println!("    \"start\": \"{:02}{:03}{:03}\"", pericope.range.start.book, pericope.range.start.chapter, pericope.range.start.verse);
        println!("    \"end\": \"{:02}{:03}{:03}\"", pericope.range.end.book, pericope.range.end.chapter, pericope.range.end.verse);
        println!("    \"num_verses\": \"{}\"", num_verses);
        println!("  }},");
    }
    println!("]");
}

pub fn print_report(pericopes: &Vec<Pericope>) {
    let mut largest_pericope = 0;
    let mut total_verses = 0;
    let mut num_pericopes = 0;

    let mut pericope_sizes: Vec<i32> = vec![];

    for pericope in pericopes {
        let num_verses = pericope_size(&pericope);
        total_verses += num_verses;
        pericope_sizes.push(num_verses);

        println!("{} - {} verses", pericope, num_verses);

        if num_verses > largest_pericope {
            largest_pericope = num_verses;
        }
        num_pericopes += 1;
    }

    pericope_sizes.sort();

    println!("————————————————————————————————————————————————————————————————————————");
    println!("There are {} total pericopes.", num_pericopes);
    println!("Largest: {} verses", largest_pericope);
    println!("Average: {} verses", total_verses / num_pericopes);
    println!("Median: {} verses", pericope_sizes[pericope_sizes.len()/2]);
    println!("————————————————————————————————————————————————————————————————————————");
}

fn pericope_size(pericope: &Pericope) -> i32 {

    let start = &pericope.range.start;
    let end = &pericope.range.end;

    return if start.chapter == end.chapter {
        end.verse - start.verse + 1
    }
    else {
        let verses_in_start_chap = num_verses_in_chapter(start.book, start.chapter);
        (verses_in_start_chap - start.verse + 1) + end.verse
    }
}