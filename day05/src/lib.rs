pub mod data;

use std::collections::HashMap;

pub use crate::data::*;

fn get_mapped_number(maps: &mut Vec<MapData>, item: AnswerDtype) -> AnswerDtype {
    for map_data in maps {
        if let Some(mapped_number) = map_data
            .source_range
            .position(|x| x == item)
            .map(|idx| map_data.destination_range_start + idx as AnswerDtype)
        {
            return mapped_number;
        }
    }

    item
}

pub fn answer_part1(data: Parsed) -> AnswerDtype {
    let (seeds, maps) = data;

    seeds
        .into_iter()
        .map(|seed| {
            let mut item = seed;

            for mut map in maps.clone() {
                item = get_mapped_number(&mut map, item);
            }

            item
        })
        .min().expect("error getting minimum value")

}

#[allow(unused_results)]
pub fn answer_part2(data: Parsed) -> AnswerDtype {

    let (seed_info, maps) = data;

    let seed_pairs = seed_info.chunks(2).map(|chunk| {
        match chunk {
            &[a, b] => (a, b),
            _ => panic!("error chunks"),
        }
    });

    let seeds = seed_pairs.flat_map(|(start, len)| (start..(start + len)).collect::<Vec<_>>());

    let map_list: Vec<HashMap<AnswerDtype, AnswerDtype>> = maps
        .into_iter()
        .map(|maps| {
            let mut hash_map = HashMap::new();

            for MapData { source_range, destination_range_start } in maps {
                for (idx, item) in source_range.enumerate() {
                    hash_map.insert(item, destination_range_start + idx as AnswerDtype);
                }
            }

            hash_map

        })
        .collect();


    // seeds
    //     .map(|seed| {

    //         item
    //     })
    //     .min().expect("error getting minimum value")

    0

}
