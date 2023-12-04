pub mod data;

pub use crate::data::*;

fn calculate_score(winning_numbers: Vec<AnswerDtype>, my_numbers: Vec<AnswerDtype>) ->AnswerDtype {
    
    let mut my_score = 0;

    for i in winning_numbers {

        if my_numbers.contains(&i) {
            if my_score == 0 {
                my_score = 1;
            } else {
                my_score *= 2;
            }
        }
    }

    my_score

}



pub fn answer_part1(data: Vec<Parsed>) -> AnswerDtype {
    
    data.iter().map(|x| calculate_score(x.0.clone(), x.1.clone())).sum()

}

// pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
//     todo!()
// }
