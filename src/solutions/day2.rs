use regex::Regex;

struct Game {
    id: u32,
    groups: Vec<String>,
}

impl Game {
    // Create a new game struct,
    fn new(_input: &str) -> Self {
        let re = Regex::new(r"Game (?<id>\d+): (?<groups>.+)").unwrap();
        let Some(caps) = re.captures(_input) else { panic!("Game::new {}", _input) };

        let _groups: Vec<String> = caps["groups"].split("; ").map(|x| x.to_string()).collect();

        Game {
            id: caps["id"].parse::<u32>().unwrap(),
            groups: _groups,
        }
    }

    // Return the game id
    fn game_id(game: Game) -> u32 {
        return game.id
    }

    // parse each round into (i red, j green k blue) (order irrelevant) into (i, j, k)
    // and takes the max (i, j, k) appeared.
    fn parse_groups(game: &Game) -> (u32, u32, u32) {
        let re = Regex::new(r"(?<red>\d+ red)|(?<green>\d+ green)|(?<blue>\d+ blue)").unwrap();

        game.groups
            .iter   ()
            .map(|group| -> (u32, u32, u32) {
                let (mut _red, mut _green, mut _blue) = (0, 0, 0);

                for capture in re.captures_iter(group.as_str()) {
                    if let Some(m) = capture.name("red") {
                        _red = m
                            .as_str()
                            .replace(" red", "") // remove " red"
                            .parse::<u32>()
                            .unwrap();
                    } else if let Some(m) = capture.name("green") {
                        _green = m
                            .as_str()
                            .replace(" green", "") // remove " green"
                            .parse::<u32>()
                            .unwrap();
                    } else if let Some(m) = capture.name("blue") {
                        _blue = m
                            .as_str()
                            .replace(" blue", "") // remove " blue"
                            .parse::<u32>()
                            .unwrap();
                    }
                }
                return (_red, _green, _blue);
            })
            .reduce(|(red1, green1, blue1), (red2, green2, blue2)| {
                return (
                    std::cmp::max(red1, red2),      // red
                    std::cmp::max(green1, green2),  // green
                    std::cmp::max(blue1, blue2),    // blue
                );
            }).unwrap()
    }
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> u32 {
    _input
        .split("\n")
        .map(Game::new)
        .filter(|game| -> bool {
            let group = Game::parse_groups(game);
            return group.0 <= 12 && group.1 <= 13 && group.2 <= 14;
        })
        .map(Game::game_id)
        .sum()
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> u32 {
    _input
        .split("\n")
        .map(Game::new)
        .map(|game| Game::parse_groups(&game))
        .map(|group| (group.0 * group.1 * group.2))
        .sum()
}
