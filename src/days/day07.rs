use std::collections::HashMap;

#[derive(Debug)]
pub enum Gate<'a> {
    Wire(&'a str),
    Not(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, u16),
    RShift(&'a str, u16),
}

pub fn parse<'a>(data: &'a str) -> HashMap<&'a str, Gate<'a>> {
    let mut circuit = HashMap::new();

    let mut words = data.split_ascii_whitespace();
    while let (Some(first), Some(second)) = (words.next(), words.next()) {
        let gate = if first == "NOT" {
            words.next(); // consume the "->"
            Gate::Not(second)
        } else if second == "->" {
            Gate::Wire(first)
        } else {
            let third = words.next().unwrap();
            words.next(); // consume the "->"

            match second {
                "AND" => Gate::And(first, third),
                "OR" => Gate::Or(first, third),
                "LSHIFT" => Gate::LShift(first, third.parse().unwrap()),
                "RSHIFT" => Gate::RShift(first, third.parse().unwrap()),
                _ => unreachable!(),
            }
        };

        let out = words.next().unwrap();
        circuit.insert(out, gate);
    }

    circuit
}

fn signal<'a>(
    wire: &'a str,
    circuit: &HashMap<&'a str, Gate<'a>>,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(val) = cache.get(wire) {
        return *val;
    }

    let val = if let Ok(val) = wire.parse() {
        val
    } else {
        match circuit[wire] {
            Gate::Wire(x) => signal(x, circuit, cache),
            Gate::Not(wire) => !signal(wire, circuit, cache),
            Gate::And(left, right) => signal(left, circuit, cache) & signal(right, circuit, cache),
            Gate::Or(left, right) => signal(left, circuit, cache) | signal(right, circuit, cache),
            Gate::LShift(left, right) => signal(left, circuit, cache) << right,
            Gate::RShift(left, right) => signal(left, circuit, cache) >> right,
        }
    };

    cache.insert(wire, val);
    val
}

pub fn part1(circuit: &HashMap<&str, Gate>) -> u16 {
    let mut cache = HashMap::new();
    signal("a", circuit, &mut cache)
}

pub fn part2(circuit: &HashMap<&str, Gate>) -> u16 {
    let mut cache = HashMap::new();
    let a_val = signal("a", circuit, &mut cache);

    cache = HashMap::new();
    cache.insert("b", a_val);
    signal("a", circuit, &mut cache)
}
