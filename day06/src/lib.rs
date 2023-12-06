pub mod data;

pub use crate::data::*;


fn find_ways_to_win(time: AnswerDtype, distance: AnswerDtype) -> AnswerDtype {
    
    let mut ways = 0;
    
    for seconds in 0..=time {
        
        let time_traveled = time - seconds;
        let speed = seconds * time_traveled;

        if speed > distance {
            ways += 1;
        }

    }


    ways
}


pub fn answer_part1(data: Vec<Race>) -> AnswerDtype {
    
    data.into_iter().map(
        |race|find_ways_to_win(race.time, race.distance)
    ).map(|x| {println!("{}", &x); x})
    .fold(1, |acc, x| acc * x)

}

// pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
//     todo!()
// }
