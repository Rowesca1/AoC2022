fn outer_join(first : &str, second : &str) -> char
{
    let mut hseta = vec!(0; 256);
    let mut hsetb = vec!(0; 256);
    let mut found : char = '+';

    for cc in first.chars() {
        let key : usize = cc as usize;
        hseta[key] = hseta[key] + 1;
    }

    for cc in second.chars() {
        let key : usize = cc as usize;
        hsetb[key] = hsetb[key] + 1;
    }

    for i in 0..hseta.len() {
        if (hseta[i] >= 1) && (hsetb[i] >= 1) {
            found = char::from_u32(i.try_into().unwrap()).unwrap();
            break;
        }
    }

    found
}

fn to_number(c : char) -> u8 
{
    match c {
        'a' ..='z' => ((c as u8 - b'a')+1).into(),
        'A' ..='Z' => ((c as u8 - b'A')+27).into(),
        _ => {println!("error for {}",c); 0}
    }
}

fn process_part1(input : &str) -> u64 {
    input.lines()
    .map(|line| {
        let (first, second) = line.split_at(line.len()/2);
        to_number(outer_join(first, second)) as u64
        })
    .sum()
}

fn process_part2(input : &str) -> u64 {
    0
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

        const INPUT : &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = process_part1(&INPUT);
        assert_eq!(result, 157);

       // let result = process_part2(&INPUT);
       // assert_eq!(result, 15);
    }
}

