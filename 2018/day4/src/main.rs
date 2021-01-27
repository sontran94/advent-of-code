use std::collections::HashMap;
type GuardID = u32;

fn main() -> anyhow::Result<()> {
    let mut record = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|val| Record::from(&val))
        .collect::<Result<Vec<Record>, _>>()?;

    // Sort by datetime
    record.sort_by(|r1, r2| r1.datetime.cmp(&r2.datetime));

    // Group actions into guard_id
    let mut guards = HashMap::<GuardID, Vec<Record>>::new();
    let mut current_guard: Option<u32> = None;
    for val in record {
        if let Action::BeginShift(id) = val.action {
            current_guard = Some(id);
        }
        match current_guard {
            Some(id) => guards.entry(id).or_default().push(val),
            None => return Err(anyhow::anyhow!("No guard id")),
        }
    }
    // Create frequency hashmap
    let mut guards_sleep_freq = HashMap::<GuardID, Vec<u32>>::new();
    let mut start: u32 = 0;
    for (id, record_vec) in guards.iter() {
        for record in record_vec.iter() {
            match record.action {
                Action::Asleep => start = record.datetime.minute,
                Action::WakeUp => {
                    let count = guards_sleep_freq.entry(*id).or_insert(vec![0; 60]);
                    count.iter_mut().zip(0..60).for_each(|(val, i)| {
                        if (start..record.datetime.minute).contains(&(i as u32)) {
                            *val += 1
                        }
                    });
                }
                _ => (),
            }
        }
    }
    dbg!(part1(&guards_sleep_freq));
    dbg!(part2(&guards_sleep_freq));
    Ok(())
}

fn part1(guards_sleep_freq: &HashMap<GuardID, Vec<u32>>) -> u32 {
    let (mut guard_id, mut index, mut longest_sleep) = (0, 0, 0);
    for (id, sleep_freq) in guards_sleep_freq.iter() {
        let sleep = sleep_freq.iter().sum::<u32>();
        if sleep > longest_sleep {
            longest_sleep = sleep;
            guard_id = *id;
            let max_freq = *sleep_freq.iter().max().unwrap();
            index = sleep_freq.iter().position(|&r| r == max_freq).unwrap() as u32;
        }
    }
    guard_id * index
}

fn part2(guards_sleep_freq: &HashMap<GuardID, Vec<u32>>) -> u32 {
    let (mut guard_id, mut max_freq, mut index) = (0, 0, 0);
    for (id, sleep_freq) in guards_sleep_freq.iter() {
        let freq = *sleep_freq.iter().max().unwrap();
        if freq > max_freq {
            max_freq = freq;
            guard_id = *id;
            index = sleep_freq.iter().position(|&r| r == max_freq).unwrap() as u32;
        }
    }
    guard_id * index
}

#[derive(Debug)]
enum Action {
    BeginShift(GuardID),
    Asleep,
    WakeUp,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

#[derive(Debug)]
struct Record {
    datetime: DateTime,
    action: Action,
}

impl Record {
    fn from(input: &str) -> anyhow::Result<Self> {
        peg::parser! {
                grammar parser() for str {
                    pub(crate) rule line() -> Record
                        = datetime:datetime() action:action() [_]*
                        { Record{ datetime, action } }

                    rule number() -> u32
                        = n:$(['0'..='9']+) { n.parse().unwrap() }

                    rule id() -> u32
                        = "#" id:number() { id }

                    rule datetime() -> DateTime
                        = "[" year:number() "-" month:number() "-" day:number() " "
                        hour:number() ":" minute:number() "]"
                        { DateTime { year, month, day, hour, minute } }

                    rule action() -> Action
                        = " Guard " id:id() { Action::BeginShift(id) }
                        / " falls " { Action::Asleep }
                        / " wakes " { Action::WakeUp }

            }
        }
        Ok(parser::line(input)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_record_from_wake() {
        let input = "[1518-05-28 00:59] wakes up";
        let record = Record::from(&input).unwrap();
        let datetime = DateTime {
            year: 1518,
            month: 5,
            day: 28,
            hour: 0,
            minute: 59,
        };
        assert_eq!(datetime, record.datetime);
        match record.action {
            Action::WakeUp => assert!(true),
            _ => assert!(false),
        }
    }
}
