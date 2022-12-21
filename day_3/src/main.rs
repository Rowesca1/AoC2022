fn inner_join(first : &str, second : &str) -> String
{
    let mut hseta = vec!(0; 256);
    let mut hsetb = vec!(0; 256);
    let mut found : String = "".to_string();

    for cc in first.chars() {
        let key : usize = cc as usize;
        hseta[key] = 1;
    }

    for cc in second.chars() {
        let key : usize = cc as usize;
        hsetb[key] = 1;
    }

    for i in 0..hseta.len() {
        if (hseta[i] == 1) && (hsetb[i] == 1) {
            found.push(char::from_u32(i as u32).unwrap()); 
        }
    }
    found
}

fn to_number(c : char) -> u64 
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

        inner_join(first, second)
        .chars()
        .map(|a| to_number(a as char))
        .sum::<u64>()
        })
    .sum()
}

fn process_part2(input : &str) -> u64 {
    let mut lines = input.lines();
    let mut total : u64 = 0;

    loop {
        let l1 = lines.next();
        if let None = l1 {
            break;
        }

        let l1 = l1.unwrap_or(""); 
        let l2 = lines.next().unwrap_or("");
        let l3 = lines.next().unwrap_or("");

        total = total + 
            inner_join(&inner_join(&l1, &l2), &l3)
            .chars()
            .map(|a| to_number(a))
            .sum::<u64>();
    }
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

        const INPUT : &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = process_part1(&INPUT);
        assert_eq!(result, 157);

        let result = process_part2(&INPUT);
        assert_eq!(result, 70);
    }
}

