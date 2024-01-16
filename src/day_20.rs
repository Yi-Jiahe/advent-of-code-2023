use std::collections::{HashMap, VecDeque};

const HIGH: bool = true;
const LOW: bool = false;
#[allow(dead_code)]
const ON: bool = true;
const OFF: bool = false;

trait Module {
    fn receive(&mut self, sender: &str, pulse: bool) -> Option<bool>;
}

struct FlipFlop {
    state: bool,
}

impl FlipFlop {
    fn new() -> FlipFlop {
        FlipFlop { state: OFF }
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, _sender: &str, pulse: bool) -> Option<bool> {
        // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
        if pulse {
            return None;
        }

        // However, if a flip-flop module receives a low pulse, it flips between on and off.
        self.state = !self.state;

        // If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
        Some(self.state)
    }
}

struct Conjunction {
    // Memory is initially empty, conjunction modules will only remember a module if it has previously sent a pulse (inferred)
    memory: HashMap<String, bool>,
}

// TODO: Conjunction modules need to know their inputs at time of creation, not when it receives a pulse from the input
impl Conjunction {
    fn new() -> Conjunction {
        Conjunction {
            memory: HashMap::new(),
        }
    }
}

impl Module for Conjunction {
    fn receive(&mut self, sender: &str, pulse: bool) -> Option<bool> {
        // When a pulse is received, the conjunction module first updates its memory for that input.
        self.memory.insert(String::from(sender), pulse);

        // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
        for remembered_pulse in self.memory.values() {
            if !remembered_pulse {
                return Some(HIGH);
            }
        }
        Some(LOW)
    }
}

fn parse_module_configuration(
    input: &str,
) -> (
    HashMap<String, Box<dyn Module>>,
    HashMap<String, Vec<String>>,
) {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut configuration: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.split("\n").map(|line| line.trim()) {
        let mut parts = line.split("->").map(|part| part.trim());

        let current_module = parts.next().unwrap();

        let (module_type, module_name) = if current_module == "broadcaster" {
            (None, String::from("broadcaster"))
        } else {
            (
                Some(&current_module[0..1]),
                (&current_module[1..]).to_string(),
            )
        };

        if let Some(module_type) = module_type {
            modules.insert(
                module_name.clone(),
                match module_type {
                    "%" => Box::new(FlipFlop::new()),
                    "&" => Box::new(Conjunction::new()),
                    _ => unreachable!(),
                },
            );
        }

        let destination_modules = parts.next().unwrap();

        configuration.insert(
            module_name,
            destination_modules
                .split(',')
                .map(|name| name.trim().to_string())
                .collect::<Vec<String>>(),
        );
    }

    (modules, configuration)
}

pub fn day_20_count_pulses(input: &str) -> usize {
    let (mut modules, configuration) = parse_module_configuration(input);
    
    let mut count = HashMap::from([
        (LOW, 0),
        (HIGH, 0)
    ]);

    for _ in 0..1000 {
        let mut stack = VecDeque::new();

        // Button to broadcaster
        count.insert(LOW, count.get(&LOW).unwrap() + 1);

        for module in configuration.get("broadcaster").expect("Broadcaster not found") {
            count.insert(LOW, count.get(&LOW).unwrap() + 1);
            stack.push_back((module, "broadcaster", LOW));
        }    

        while !stack.is_empty() {
            let (current_module_name, sender, input_pulse) = stack.pop_front().unwrap();

            if current_module_name == "output" {
                continue;
            }

            let module: &mut Box<dyn Module> = modules.get_mut(current_module_name).expect(&format!("{} not found", current_module_name));

            if let Some(pulse) = module.receive(sender, input_pulse) {
                for destination_module in configuration.get(current_module_name).expect(&format!("{} not found", current_module_name)) {
                    count.insert(pulse, count.get(&pulse).unwrap() + 1);
                    stack.push_back((destination_module, current_module_name, pulse));
                }
            };
        }
    }

    let (low_count, high_count) = (count.get(&LOW).unwrap(), count.get(&HIGH).unwrap());
    println!("LOW: {}, HIGH: {}", low_count, high_count);
    low_count * high_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_flop() {
        let mut flip_flop = FlipFlop::new();

        // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
        assert_eq!(None, flip_flop.receive("sender", HIGH));

        // However, if a flip-flop module receives a low pulse, it flips between on and off.
        let output = flip_flop.receive("sender", LOW);
        assert_eq!(ON, flip_flop.state);
        // If it was off, it turns on and sends a high pulse.
        assert_eq!(Some(HIGH), output);

        // However, if a flip-flop module receives a low pulse, it flips between on and off.
        let output = flip_flop.receive("sender", LOW);
        assert_eq!(OFF, flip_flop.state);
        // If it was off, it turns on and sends a high pulse.
        assert_eq!(Some(LOW), output);
    }

    #[test]
    fn test_conjunction() {
        let mut inv = Conjunction::new();

        assert_eq!(Some(LOW), inv.receive("a", HIGH));
        assert_eq!(1, inv.memory.len());
        assert_eq!(Some(&HIGH), inv.memory.get("a"));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(32000000, day_20_count_pulses(r#"broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a"#));

        assert_eq!(11687500, day_20_count_pulses(r#"broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"#));
    }
}
