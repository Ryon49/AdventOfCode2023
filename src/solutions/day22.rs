type Coordinate = (u32, u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Brick {
    start: Coordinate,
    end: Coordinate,
    z: usize,
}

impl Brick {
    // returns true if the line from (a,b)->(c,d) intersects with (p,q)->(r,s)
    fn is_intersect(&self, brick: Brick) -> bool {
        // let ((a, b), (c, d)) = (self.start, self.end);
        // let ((p, q), (r, s)) = (brick.start, brick.end);
        // let det = (c - a) * (s - q) - (r - p) * (d - b);
        // if det == 0 {
        //     return false;
        // } else {
        //     let lambda = ((s - q) * (r - a) + (p - r) * (s - b)) / det;
        //     let gamma = ((b - d) * (r - a) + (c - a) * (s - b)) / det;
        //     return (0 < lambda && lambda < 1) && (0 < gamma && gamma < 1);
        // }
        todo!()
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let parts = line.split("~").collect::<Vec<&str>>();

            let start_point = parts[0].split(",").collect::<Vec<&str>>();
            let z = start_point[2].parse::<usize>().unwrap();

            let x1 = start_point[0].parse::<u32>().unwrap();
            let y1 = start_point[1].parse::<u32>().unwrap();

            let end_point = parts[1].split(",").collect::<Vec<&str>>();

            let x2 = end_point[0].parse::<u32>().unwrap();
            let y2 = end_point[1].parse::<u32>().unwrap();

            Brick {
                start: (x1, y1),
                end: (x2, y2),
                z,
            }
        })
        .collect::<Vec<Brick>>()
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    let mut bricks_snapshot = parse_input(input);

    bricks_snapshot.sort_by(|b1, b2| b1.z.cmp(&b2.z));

    let max_height = bricks_snapshot[bricks_snapshot.len() - 1].z + 1; // the question is 1-indexed

    // sort snapshot brick by its height (z-index)
    let mut bricks_settled: Vec<Vec<Brick>> = vec![Vec::new(); max_height];

    // for each brick, find the level after stabled.
    for snapshot in bricks_snapshot {
        let mut current_level = snapshot.z - 1; // this is the row we are searching.

        // println!("current brick = {:?}", snapshot);
        'outer: while current_level > 0 {
            for brick in bricks_settled[current_level].clone() {
                if brick.is_intersect(snapshot) {
                    // find support
                    break 'outer;
                }
            }
            // continue to lower level to find if the brick fits.
            current_level -= 1;
        }
        // println!("pushed to {}", current_level);
        bricks_settled[current_level + 1].push(snapshot.clone());
    }

    let mut removable_bricks = 0;
    for level_index in 1..max_height-1 {
        'outer: for current_brick in bricks_settled[level_index].iter() {
            // run a nested loop to check if any bricks in the upper level can fit in the current level 
            // if current_brick is removed. 
            // if fits, it means current_brick is not removable.
            for brick_in_same_level in bricks_settled[level_index].iter() {
                if current_brick == brick_in_same_level {
                    continue;
                }
                for brick_in_upper_level in bricks_settled[level_index + 1].iter() {
                    if brick_in_same_level.is_intersect(*brick_in_upper_level) {
                        continue 'outer;
                    }
                }
            }
            removable_bricks += 1;
        }
    }
    removable_bricks
}
