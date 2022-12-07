#[derive(PartialEq, Clone)]
enum Shape{
    Rock,
    Paper,
    Scissor,
}

fn input_to_shape_part1(input : char) -> Option<Shape> {
    match input {
        'A' | 'X' => Some(Shape::Rock),
        'B' | 'Y' => Some(Shape::Paper),
        'C' | 'Z' => Some(Shape::Scissor),
        _ => None
    }
}

fn shape_to_points(shape : &Shape) -> u64 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    }
}

fn rock_paper_scissor_points(contestant : &Shape, opponent : &Shape) -> u64
{
    let mut p = 0;

    if contestant.eq(opponent) {
        p = 3;
    } else {
        match contestant {
            Shape::Rock => {
                if opponent.eq(&Shape::Scissor) {
                    p = 6;
                }},
            Shape::Paper => {
                if opponent.eq(&Shape::Rock) {
                    p = 6;
                }},
            Shape::Scissor => {
                if opponent.eq(&Shape::Paper) {
                    p = 6;
                }},
        }
    }

   p + shape_to_points(contestant)
}

fn process_part1(input : &str) -> u64 {
    let mut total = 0;

    input
        .lines()
        .for_each(|x| {
            let mut cs = x.chars();
            if let Some(theirs) = input_to_shape_part1(cs.nth(0).unwrap()) {
                if let Some(mine) = input_to_shape_part1(cs.nth(1).unwrap()) {
                    total = total + rock_paper_scissor_points(&mine, &theirs); 
                }
            }
        });

    total
}

#[derive(PartialEq)]
enum Result {
    Win,
    Lose,
    Draw
}

fn result_to_points(result : &Result) -> u64 {
    match result {
        Result::Win => 6,
        Result::Draw => 3,
        Result::Lose => 0,
    }
}

fn input_to_result_part2(input : char) -> Option<Result> {
    match input {
        'X' => Some(Result::Lose),
        'Y' => Some(Result::Draw),
        'Z' => Some(Result::Win),
        _ => None
    }
}

fn get_shape_for_result(result : &Result, input : &Shape) -> Shape {
    match result {
        Result::Draw => input.clone(),
        Result::Win => {
            match input {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissor,
                Shape::Scissor => Shape::Rock,
            }
        },
        Result::Lose => {
            match input {
                Shape::Scissor => Shape::Paper,
                Shape::Paper => Shape::Rock,
                Shape::Rock => Shape::Scissor,
            }
        }
    }
}

fn process_part2(input : &str) -> u64 {
    let mut total = 0;
    input
        .lines()
        .for_each(|x| {
            let mut cs = x.chars();
            if let Some(theirs) = input_to_shape_part1(cs.nth(0).unwrap()) {
                if let Some(mine) = input_to_result_part2(cs.nth(1).unwrap()) {
                    total = total + result_to_points(&mine) + shape_to_points(&get_shape_for_result(&mine, &theirs));
                }
            }
        });

    total
}

fn main() {
    //part 1
    let content = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("part1 = {}", process_part1(&content));
    
    //part 2
    let content = std::fs::read_to_string("input.txt").expect("Error reading input.txt");
    println!("part2 = {}", process_part2(&content));
}

#[cfg(test)]
mod tests {

    #[test]
    fn correct() {
        use super::*;

        const INPUT : &str = 
"A Y
B X
C Z";
        let result = process_part1(&INPUT);
        assert_eq!(result, 15);

        let result =  rock_paper_scissor_points(&Shape::Scissor, &Shape::Rock);
        assert_eq!(result, 3);

        let result =  rock_paper_scissor_points(&Shape::Paper, &Shape::Paper);
        assert_eq!(result, 5);

        let result =  rock_paper_scissor_points(&Shape::Rock, &Shape::Scissor);
        assert_eq!(result, 7);

        let result = process_part2(&INPUT);
        assert_eq!(result, 12);
    }
}
