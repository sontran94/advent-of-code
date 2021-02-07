use std::collections::HashMap;
use std::collections::VecDeque;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt").lines();
    let gates = input
        .clone()
        .map(|line| parse_gates_1(line))
        .collect::<Result<Vec<Gate>, _>>()?;

    let mut wires = Wires::new();
    wires.operate(&gates);
    println!("The value of wire a: {:?}", wires.values.get("a"));

    let gates_2 = input
        .map(|line| parse_gates_2(line, *wires.values.get("a").unwrap()))
        .collect::<Result<Vec<Gate>, _>>()?;
    let mut wires_2 = Wires::new();
    wires_2.operate(&gates_2);
    dbg!(wires_2.values.get("b"));
    println!(
        "The value of wire a after changing b: {:?}",
        wires_2.values.get("a")
    );

    Ok(())
}

#[derive(Debug)]
enum Gate {
    INIT(Value, Wire),
    ASSIGN(Wire, Wire),
    NOT(Wire, Wire),
    AND(Wire, Wire, Wire),
    OR(Wire, Wire, Wire),
    RSHIFT(Wire, Value, Wire),
    LSHIFT(Wire, Value, Wire),
}

type Wire = String;
type Value = u16;

#[derive(Debug)]
struct Wires {
    values: HashMap<Wire, Value>,
}

impl Wires {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn operate(&mut self, gates: &[Gate]) {
        let mut queue = VecDeque::new();
        for g in gates.iter() {
            queue.push_back(g);
        }
        self.values.insert("one".to_string(), 1);
        while let Some(gate) = queue.pop_front() {
            match gate {
                Gate::INIT(value, w) => {
                    self.values.insert(w.clone(), *value);
                }
                Gate::ASSIGN(w1, w2) => {
                    if self.values.contains_key(w1) {
                        self.values
                            .insert(w2.clone(), *self.values.get(w1).unwrap());
                    } else {
                        queue.push_back(gate);
                    }
                }
                Gate::NOT(w1, w2) => {
                    if self.values.contains_key(w1) {
                        self.values
                            .insert(w2.clone(), !self.values.get(w1).unwrap());
                    } else {
                        queue.push_back(gate);
                    }
                }
                Gate::AND(w1, w2, w3) => {
                    if self.values.contains_key(w1) & self.values.contains_key(w2) {
                        self.values.insert(
                            w3.clone(),
                            self.values.get(w1).unwrap() & self.values.get(w2).unwrap(),
                        );
                    } else {
                        queue.push_back(gate);
                    }
                }
                Gate::OR(w1, w2, w3) => {
                    if self.values.contains_key(w1) & self.values.contains_key(w2) {
                        self.values.insert(
                            w3.clone(),
                            self.values.get(w1).unwrap() | self.values.get(w2).unwrap(),
                        );
                    } else {
                        queue.push_back(gate);
                    }
                }
                Gate::RSHIFT(w1, value, w2) => {
                    if self.values.contains_key(w1) {
                        self.values
                            .insert(w2.clone(), self.values.get(w1).unwrap() >> value);
                    } else {
                        queue.push_back(gate);
                    }
                }
                Gate::LSHIFT(w1, value, w2) => {
                    if self.values.contains_key(w1) {
                        self.values
                            .insert(w2.clone(), self.values.get(w1).unwrap() << value);
                    } else {
                        queue.push_back(gate);
                    }
                }
            }
        }
    }
}
fn parse_gates_1(s: &str) -> anyhow::Result<Gate> {
    peg::parser! {
        grammar parser() for str {
            pub(crate) rule line() -> Gate
                = init() / assign() / not() / and() / and_one() / or() / rshift() / lshift()

            rule init() -> Gate
                = v:value() " -> " w:wire()
                { Gate::INIT(v, w) }

            rule assign() -> Gate
                = w1:wire() " -> " w2:wire()
                { Gate::ASSIGN(w1, w2) }

            rule not() -> Gate
                = "NOT " w1:wire() " -> " w2:wire()
                { Gate::NOT(w1, w2) }

            rule and() -> Gate
                = w1:wire() " AND " w2:wire() " -> " w3:wire()
                { Gate::AND(w1, w2, w3) }

            rule and_one() -> Gate
                = "1 AND " w2:wire() " -> " w3:wire()
                { Gate::AND("one".to_string(), w2, w3) }

            rule or() -> Gate
                = w1:wire() " OR " w2:wire() " -> " w3:wire()
                { Gate::OR(w1, w2, w3) }

            rule rshift() -> Gate
                = w1:wire() " RSHIFT " v:value() " -> " w2:wire()
                { Gate::RSHIFT(w1, v, w2) }

            rule lshift() -> Gate
                = w1:wire() " LSHIFT " v:value() " -> " w2:wire()
                { Gate::LSHIFT(w1, v, w2) }

            rule wire() -> Wire
                = w:$(['a'..='z']+) { w.to_string() }

            rule value() -> u16
                = v:$(['0'..='9']+) { v.parse().unwrap() }

        }
    }
    Ok(parser::line(s)?)
}

fn parse_gates_2(s: &str, b: Value) -> anyhow::Result<Gate> {
    peg::parser! {
        grammar parser() for str {
            pub(crate) rule line(b: Value) -> Gate
                = init_b(b) / init() / assign() / not() / and() / and_one() / or() / rshift() / lshift()
            rule init_b(b: Value) -> Gate
                = ['a'..='z' | 'A'..='Z' | '0'..='9']* " -> " "b"
                { Gate::INIT(b, "b".to_string()) }

            rule init() -> Gate
                = v:value() " -> " w:wire()
                { Gate::INIT(v, w) }

            rule assign() -> Gate
                = w1:wire() " -> " w2:wire()
                { Gate::ASSIGN(w1, w2) }

            rule not() -> Gate
                = "NOT " w1:wire() " -> " w2:wire()
                { Gate::NOT(w1, w2) }

            rule and() -> Gate
                = w1:wire() " AND " w2:wire() " -> " w3:wire()
                { Gate::AND(w1, w2, w3) }

            rule and_one() -> Gate
                = "1 AND " w2:wire() " -> " w3:wire()
                { Gate::AND("one".to_string(), w2, w3) }

            rule or() -> Gate
                = w1:wire() " OR " w2:wire() " -> " w3:wire()
                { Gate::OR(w1, w2, w3) }

            rule rshift() -> Gate
                = w1:wire() " RSHIFT " v:value() " -> " w2:wire()
                { Gate::RSHIFT(w1, v, w2) }

            rule lshift() -> Gate
                = w1:wire() " LSHIFT " v:value() " -> " w2:wire()
                { Gate::LSHIFT(w1, v, w2) }

            rule wire() -> Wire
                = w:$(['a'..='z']+) { w.to_string() }

            rule value() -> u16
                = v:$(['0'..='9']+) { v.parse().unwrap() }

        }
    }
    Ok(parser::line(s, b)?)
}
