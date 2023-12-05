pub mod data;

pub use crate::data::*;

fn get_mapped_number(maps: &mut Vec<MapData>, item: AnswerDtype) -> AnswerDtype {
    for map_data in maps {
        if let Some(mapped_number) = map_data
            .source_range
            .position(|x| x == item)
            .map(|idx| map_data.destination_range_start + idx as u64)
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

// pub fn answer_part2(data: Parsed) -> AnswerDtype {
//     todo!()
// }
