fn main() {
    let res = std::fs::read_to_string("input.txt");
    match res {
        Ok(content) => {
            let mut highest : u64 = 0;
            let mut cur : u64 = 0;
            for line in content.lines() {
                if line.len() == 0 {
                    if cur > highest {
                        highest = cur;
                    }
                    cur = 0;
                }else {
                    let cal : u64 = line.parse().unwrap();
                    cur = cur + cal;
                }
            }
            println!("Highest calories for an ELF {}", highest);
        },
        Err(e) => {
            println!("Error opening input.txt");
        }
    };
}
