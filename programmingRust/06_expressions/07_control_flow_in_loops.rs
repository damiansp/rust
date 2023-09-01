fn main() {
    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer: ") {
                break line;
            }
        } else {
            break "answer: nothing";
        }
    };

    for line in input_lines {
        let trimmed = trim_comments_and_whitespace(line);
        if trimmed.is_empty() {
            continue;
        }
    }

    'search:
    for room in apartment {
        for spot in room.hiding_spots() {
            if spot.contains(keys) {
                println!("Keys found in the {} in the {}", spot, room);
                break 'search;
            }
        }
    }

    let sqrt = 'outer: loop {
        let n = next_number();
        for i in 1.. {
            let sq = i * i;
            if sq == n {
                break 'outer i;  // return i (found sqrt)
            }
            if square > n {
                break; // not a square; try next
            }
        }
    }
}