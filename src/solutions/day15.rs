fn compute_sequence(sequence: &str) -> usize {
    let mut value: usize = 0;
    for c in sequence.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }
    value
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    input.split(",").map(compute_sequence).sum()
}

fn parse_sequence(sequence: &str) -> (&str, &str, Option<usize>) {
    if sequence.contains("=") {
        let parts = sequence.split("=").collect::<Vec<&str>>();
        return (parts[0], "=", parts[1].parse::<usize>().ok());
    } else {
        let parts = sequence.split("-").collect::<Vec<&str>>();
        return (parts[0], "-", None);
    }
}

fn get_label_index(b: &Vec<(&str, usize)>, label: &str) -> Option<usize> {
    for index in 0..b.len() {
        if label == b[index].0 {
            return Some(index);
        }
    }
    None
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    input
        .split(",")
        .map(parse_sequence)
        .for_each(|(label, op, focal)| {
            let box_index = compute_sequence(label);
            if op == "-" {
                // remove the target label
                if let Some(label_index) = get_label_index(&boxes[box_index], label) {
                    boxes[box_index].remove(label_index);
                }
            } else if op == "=" {
                // update target label
                if let Some(label_index) = get_label_index(&boxes[box_index], label) {
                    boxes[box_index][label_index].1 = focal.unwrap();
                } else {
                    // label does not exist, append a new one
                    boxes[box_index].push((label, focal.unwrap()));
                }
            }
        });

    boxes
        .into_iter() // loop over boxes
        .enumerate()
        .map(|(box_index, b)| {
            b.into_iter() // loop over labels
                .enumerate()
                .map(|(label_index, (_, focal))| (box_index + 1) * (label_index + 1) * focal)
                .sum::<usize>()
        })
        .sum()
}
