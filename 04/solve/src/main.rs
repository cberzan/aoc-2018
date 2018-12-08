#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Ord)]
#[derive(PartialOrd)]
enum Action {
    WakesUp,
    FallsAsleep,
    BeginsShift,
}

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Ord)]
#[derive(PartialOrd)]
struct LogLine {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    guard_id: Option<i32>,
    action: Action,
}

// Parses a log line from a string.
fn parse_log_line(line_str: &str) -> Option<LogLine> {
    lazy_static! {
        static ref RE: Regex = Regex::new(concat!(
            r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] ",
            r"(Guard #(\d+) begins shift|falls asleep|wakes up)")).unwrap();
    }
    // println!("re: {:?}", *RE);
    // println!("line_str: {}", line_str);
    // println!("captures: {:?}", RE.captures(line_str));
    match RE.captures(line_str) {
        Some(captures) => Some(LogLine{
            year: captures[1].parse().unwrap(),
            month: captures[2].parse().unwrap(),
            day: captures[3].parse().unwrap(),
            hour: captures[4].parse().unwrap(),
            minute: captures[5].parse().unwrap(),
            guard_id: captures.get(7).map_or(None, |m| Some(m.as_str().parse().unwrap())),
            action: if captures[6].contains("begins") {
                        Action::BeginsShift
                    } else if captures[6].contains("falls") {
                        Action::FallsAsleep
                    } else {
                        Action::WakesUp
                    }
            }),
        None => None
    }
}

#[test]
fn test_parse_log_line() {
    assert_eq!(parse_log_line("[1518-11-01 23:58] Guard #99 begins shift"),
               Some(LogLine{year: 1518, month: 11, day: 1, hour: 23, minute: 58,
                            guard_id: Some(99), action: Action::BeginsShift}));
    assert_eq!(parse_log_line("[1518-11-02 00:40] falls asleep"),
               Some(LogLine{year: 1518, month: 11, day: 2, hour: 0, minute: 40,
                            guard_id: None, action: Action::FallsAsleep}));
    assert_eq!(parse_log_line("[1518-11-02 00:50] wakes up"),
               Some(LogLine{year: 1518, month: 11, day: 2, hour: 0, minute: 50,
                            guard_id: None, action: Action::WakesUp}));
}

// Returns (guard_id, minute, guard_id * minute).
fn solve_part1(log_lines: &Vec<LogLine>) -> (i32, i32, i32) {
    let mut guard_id_to_minutes_asleep : HashMap<i32, i32> = HashMap::new();
    let mut guard_id_to_minute_to_days_asleep :
        HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut current_guard_id = None;
    let mut asleep_since_minute = None;
    for log_line in log_lines {
        println!("state: {:?} {:?} log_line: {:?}",
            current_guard_id, asleep_since_minute, log_line);
        match log_line.action {
            Action::FallsAsleep => {
                assert!(asleep_since_minute == None);  // inception-style sleep
                asleep_since_minute = Some(log_line.minute);
            },
            Action::WakesUp => {
                assert!(current_guard_id != None);
                assert!(asleep_since_minute != None);
                *(guard_id_to_minutes_asleep.entry(
                    current_guard_id.unwrap()).or_insert(0)) += 
                        log_line.minute - asleep_since_minute.unwrap();
                let minute_to_days_asleep = guard_id_to_minute_to_days_asleep.entry(
                    current_guard_id.unwrap()).or_insert(HashMap::new());
                for minute in asleep_since_minute.unwrap()..log_line.minute {
                    *(minute_to_days_asleep.entry(minute).or_insert(0)) += 1;
                }
                asleep_since_minute = None;
            },
            Action::BeginsShift => {
                assert!(asleep_since_minute == None);
                current_guard_id = Some(log_line.guard_id.unwrap());
            },
        }
    }
    let chosen_guard_id = *(guard_id_to_minutes_asleep.iter()
            .max_by_key(|e| e.1).unwrap().0);
    let chosen_minute = *(guard_id_to_minute_to_days_asleep[&chosen_guard_id].iter()
            .max_by_key(|e| e.1).unwrap().0);
    (chosen_guard_id, chosen_minute, chosen_guard_id * chosen_minute)
}

#[test]
fn test_solve_part1() {
    let lines = r"[1518-11-01 00:00] Guard #10 begins shift
                  [1518-11-01 00:05] falls asleep
                  [1518-11-01 00:25] wakes up
                  [1518-11-01 00:30] falls asleep
                  [1518-11-01 00:55] wakes up
                  [1518-11-01 23:58] Guard #99 begins shift
                  [1518-11-02 00:40] falls asleep
                  [1518-11-02 00:50] wakes up
                  [1518-11-03 00:05] Guard #10 begins shift
                  [1518-11-03 00:24] falls asleep
                  [1518-11-03 00:29] wakes up
                  [1518-11-04 00:02] Guard #99 begins shift
                  [1518-11-04 00:36] falls asleep
                  [1518-11-04 00:46] wakes up
                  [1518-11-05 00:03] Guard #99 begins shift
                  [1518-11-05 00:45] falls asleep
                  [1518-11-05 00:55] wakes up";
    let log_lines = lines.split('\n')
        .map(|line| parse_log_line(line.trim()).unwrap())
        .collect();
    assert_eq!(solve_part1(&log_lines), (10, 24, 240));
}

fn main() {
    // Read log lines.
    let mut log_lines: Vec<LogLine> = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        log_lines.push(parse_log_line(line.trim()).unwrap());
    }

    // Sort log lines, and fill in guard_id.
    log_lines.sort_unstable();
    let mut current_guard_id = None;
    for log_line in &mut log_lines {
        if log_line.guard_id == None {
            assert!(current_guard_id != None);
            log_line.guard_id = current_guard_id;
        } else {
            current_guard_id = log_line.guard_id;
        }
    }

    println!("part 1: {:?}", solve_part1(&log_lines));
    // println!("part 2: {:?}", solve_part2(&log_lines));
}
