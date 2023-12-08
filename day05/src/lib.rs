pub mod data;

pub use crate::data::*;

use rayon::prelude::*;

fn get_mapped_number_faster(maps: &Vec<MapData>, item: AnswerDtype) -> AnswerDtype {
    for map_data in maps {
        let start = map_data.source_range.start;
        let end = map_data.source_range.end;

        if start <= item && item < end {
            let pos = item - start;

            return map_data.destination_range_start + pos as AnswerDtype;
        }
    }

    item
}

pub fn answer_part1(data: Parsed) -> AnswerDtype {
    let (seeds, maps) = data;

    seeds
        .into_par_iter()
        .map(|seed| {
            let mut item = seed;

            for map in &maps {
                item = get_mapped_number_faster(&map, item);
            }

            item
        })
        .min()
        .expect("error getting minimum value")
}

pub fn answer_part2(data: Parsed) -> AnswerDtype {
    let (seed_info, maps) = data;

    let seed_pairs: Vec<_> = seed_info
        .chunks(2)
        .map(|chunk| match chunk {
            &[a, b] => (a, b),
            _ => panic!("error chunks"),
        })
        .collect();

    seed_pairs
        .into_par_iter()
        .flat_map(|(start, len)| (start..(start + len)).collect::<Vec<_>>())
        .map(|seed| {
            let mut item = seed;

            for map in &maps {
                item = get_mapped_number_faster(&map, item);
            }

            item
        })
        .min()
        .expect("error getting minimum value")
}
