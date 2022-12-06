fn main() {
    //Part 1
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

    //Part 2
    let content = std::fs::read_to_string("input.txt").expect("Error opening input.txt");
    let mut top : Vec<u64> = vec![0; 3];
    let mut cur : u64 = 0;
    for l in content.lines() {
        if l.len() == 0 {
            let mut tmp : u64;
            if cur > top[0] {
                if cur > top[1] {
                    if cur > top[2] {
                        tmp = top[2];
                        top[2] = cur;
                        cur = tmp;
                    }
                    tmp = top[1];
                    top[1] = cur;
                    cur = tmp;
                }
                top[0] = cur;
            }
            cur = 0;
        } else {
            let cal : u64 = l.parse().unwrap();
            cur = cur + cal;
        }
    }
    println!("total = {}" , top[0] + top[1] + top[2]);

}
