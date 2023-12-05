pub mod data;

pub use crate::data::*;

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

struct MapData2 {
    destination_range_start: AnswerDtype,
}

pub fn answer_part1(data: Parsed) -> AnswerDtype {
    let (seeds, maps) = data;

    seeds
        .into_iter()
        .map(|seed| {
            let mut item = seed;

            for map in maps.clone() {
                item = get_mapped_number_faster(&map, item);
            }

            item
        })
        .min()
        .expect("error getting minimum value")
}

#[allow(unused_results)]
pub fn answer_part2(data: Parsed) -> AnswerDtype {
    let (seed_info, maps) = data;

    let seed_pairs = seed_info.chunks(2).map(|chunk| match chunk {
        &[a, b] => (a, b),
        _ => panic!("error chunks"),
    });

    let seeds = seed_pairs.flat_map(|(start, len)| (start..(start + len)).collect::<Vec<_>>()).collect::<Vec<_>>();

    let total_seeds = seeds.len();

    seeds
        .into_iter()
        .enumerate()
        .map(|(handled, seed)| {
            // println!("seed: {}", seed);
            let mut item = seed;

            for map in maps.clone() {
                item = get_mapped_number_faster(&map, item);
            }

            if handled % 10000 == 0 {
                println!("handled: {}%", handled as f64 / total_seeds as f64 * 100f64);
            }

            item
        })
        .min()
        .expect("error getting minimum value")
}
