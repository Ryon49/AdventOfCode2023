use std::collections::{HashMap, VecDeque};

// For Conjunction module, it needs to know the source of the input in order to modify its memory.
type PulseInput = (String, Pulse);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop {
        switch: bool,
        targets: Vec<String>,
    },
    Conjunction {
        memory: HashMap<String, Pulse>,
        targets: Vec<String>,
    },
    Broadcast {
        targets: Vec<String>,
    },
}

impl Module {
    // index is used for the naming of broadcast module
    fn from_input(input: &str) -> (String, Self) {
        if input.starts_with("broadcaster") {
            let targets = input
                .strip_prefix("broadcaster -> ")
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            return (
                "Broadcaster".to_string(),
                Module::Broadcast { targets: targets },
            );
        } else {
            let parts = input[1..].split(" -> ").collect::<Vec<&str>>();
            let name = parts[0].to_string();
            let targets = parts[1]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            if input.starts_with("%") {
                return (
                    name,
                    Module::FlipFlop {
                        switch: false,
                        targets: targets,
                    },
                );
            } else {
                return (
                    name,
                    Module::Conjunction {
                        memory: HashMap::new(),
                        targets: targets,
                    },
                );
            }
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let modules = input
        .lines()
        .map(Module::from_input)
        .collect::<Vec<(String, Module)>>();

    let mut memory = modules
        .clone()
        .into_iter()
        .collect::<HashMap<String, Module>>();

    for (name, module) in &modules {
        // println!("{:?}", module);
        if let Module::FlipFlop { switch: _, targets } = module {
            for target in targets {
                if let Module::Conjunction { memory, targets: _ } = memory.get_mut(target).unwrap()
                {
                    memory.insert(name.clone(), Pulse::Low);
                }
            }
        }
    }
    memory
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    let mut memory = parse_input(input);

    let mut memo: HashMap<String, (i32, i32, HashMap<String, Module>)> = HashMap::new();

    let (mut total_low, mut total_high) = (0, 0);
    for _ in 0..1000 {
        // Use String representation of modules in memory as key
        let memo_key = memory
            .values()
            .map(|m| format!("{:?}", m))
            .collect::<Vec<String>>()
            .join("\n");
        if let Some((low_count, high_count, next_memory)) = memo.get(&memo_key) {
            total_low += low_count;
            total_high += high_count;
            memory = next_memory.clone();
            continue;
        }

        // Just for simplicity, I will use (record_output.join("\n"), low_count, high_count) as key for memo.
        // low count starts with 1 because starting pulse is Low
        let (mut low_count, mut high_count) = (0, 0);

        let mut instructions: VecDeque<(String, PulseInput)> = VecDeque::new();
        instructions.push_back((
            "Broadcaster".to_string(),
            ("button_pushed".to_string(), Pulse::Low),
        ));

        while let Some((module_name, (source, pulse))) = instructions.pop_front() {
            if pulse == Pulse::Low {
                low_count += 1;
            } else {
                high_count += 1;
            }

            if let Some(module) = memory.get_mut(&module_name) {
                match module {
                    Module::Broadcast { targets } => {
                        // simply broadcast the pulse
                        for target in targets {
                            if pulse == Pulse::High {
                                instructions.push_back((
                                    target.to_string(),
                                    (module_name.clone(), Pulse::High),
                                ));
                            } else {
                                instructions.push_back((
                                    target.to_string(),
                                    (module_name.clone(), Pulse::Low),
                                ));
                            }
                        }
                    }
                    Module::FlipFlop { switch, targets } => {
                        if pulse == Pulse::Low {
                            if *switch {
                                // switch is currently on, send high pulse
                                for target in targets {
                                    instructions.push_back((
                                        target.to_string(),
                                        (module_name.clone(), Pulse::Low),
                                    ));
                                }
                            } else {
                                for target in targets {
                                    instructions.push_back((
                                        target.to_string(),
                                        (module_name.clone(), Pulse::High),
                                    ));
                                }
                            }
                            *switch = !(*switch);
                        }
                    }
                    Module::Conjunction { memory, targets } => {
                        // update the memory.
                        memory.insert(source, pulse);
                        // compute the output pulse
                        let output_pulse =
                            if memory.values().all(|pulse: &Pulse| pulse == &Pulse::High) {
                                Pulse::Low
                            } else {
                                Pulse::High
                            };
                        for target in targets {
                            if output_pulse == Pulse::High {
                                instructions.push_back((
                                    target.to_string(),
                                    (module_name.clone(), Pulse::High),
                                ));
                            } else {
                                instructions.push_back((
                                    target.to_string(),
                                    (module_name.clone(), Pulse::Low),
                                ));
                            }
                        }
                    }
                }
            }
        }

        total_low += low_count;
        total_high += high_count;

        memo.insert(memo_key, (low_count, high_count, memory.clone()));
    }

    total_low * total_high
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i32 {
    0
}
