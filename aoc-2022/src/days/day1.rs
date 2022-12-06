
pub fn run() {
   let content = crate::helpers::read_input(1);
   
    let lines = content.split("\n");
    let mut elves: [i32; 500] = [0; 500];

    let mut index = 0;
    let mut count = 0;
    for line in lines {
        if line == "" {
            elves[index] = count;
            index += 1;
            count = 0;
            continue
        } else {
            count += String::from(line).parse::<i32>().unwrap();
        }
    }

    // Part 1

    let mut max = 0;
    for count in elves {
        if count > max {
            max = count;
        }
    }
    println!("{}", max);

    // Part 2
    
    let mut top_three: [i32; 3] = [0; 3];

    for count in elves {
        for (i, c) in top_three.iter().enumerate() {
            if count > *c {
                top_three[i] = count;
                break
            }
        }
    }

    let mut total = 0;
    for count in top_three {
        total += count;
    }
    println!("{}", total);

}