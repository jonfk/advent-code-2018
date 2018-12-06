#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::cmp::Ord;
use std::fmt;
use std::collections::HashMap;

fn main() {
    println!("Hello, Day 4!");


    let mut events = parse_input(INPUT);
    events.sort();

    // for event in events {
    //     println!("{}", event);
    // }

    let shifts = calculate_guard_shifts(events);

    let sleepiest_guard = shifts.iter().map(|(id, &shift)| {
        GuardAccumulatedShifts{
            guard_id: *id,
            shifts: shift,
            sleep: shift.iter().sum(),
        }
    }).max().expect("failed to find max");

    for (guard_id,shift) in shifts {
        let sum: u64 = shift.iter().sum();
        println!("guard_id: {} slept: {} shift: {:?}", guard_id, sum, &shift[..]);
    }

    let max = sleepiest_guard.shifts.iter().enumerate().fold((0 as usize,0), |max, (i,minute)| if minute > &max.1 {(i,*minute)} else {max});

    println!("{}", sleepiest_guard.guard_id);
    println!("{:?}", max);
    println!("{:?}", &sleepiest_guard.shifts[..]);
}

pub struct GuardAccumulatedShifts {
    pub guard_id: GuardId,
    pub shifts: GuardSleepCycle,
    pub sleep: u64,
}

impl Ord for GuardAccumulatedShifts {
    fn cmp(&self, other: &GuardAccumulatedShifts) -> Ordering {
        self.sleep.cmp(&other.sleep)
    }
}

impl PartialOrd for GuardAccumulatedShifts {
    fn partial_cmp(&self, other: &GuardAccumulatedShifts) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GuardAccumulatedShifts {
    fn eq(&self, other: &GuardAccumulatedShifts) -> bool {
        self.sleep == other.sleep
    }
}

impl Eq for GuardAccumulatedShifts{}

pub fn calculate_guard_shifts(events: Vec<Event>) -> HashMap<GuardId, GuardSleepCycle> {
    let mut cycles = HashMap::new();
    let mut current_guard: Option<GuardId> = None;
    let mut start_sleep_timestamp = events[0].timestamp.clone();
    for event in events {
        match event.action {
            Action::BeginShift(guard_id) => {
                current_guard = Some(guard_id);
            }
            Action::FallsAsleep => {
                start_sleep_timestamp = event.timestamp;
            }
            Action::WakesUp => {
                let wake_up_timestamp = event.timestamp;
                let mut acc_cycles = cycles.entry(current_guard.expect("failed no current guard")).or_insert([0; 60]);
                add_guard_shift(&mut acc_cycles, &start_sleep_timestamp, &wake_up_timestamp);
            }
        }
    }
    cycles
}

pub fn add_guard_shift(accumulated_cycle: &mut GuardSleepCycle, start: &Timestamp, end: &Timestamp) {
    let start_index = if start.hour == 23 {
        0//((start.minute as i64 - 60 as i64) + 60) as usize
    } else {
        start.minute// + 60
    };
    let end_index = if end.hour == 23 {
        0
    }else {
        end.minute// + 60;
    };
    //println!("shift {} start: {},  end: {}", start, start_index, end.minute);

    for i in start.minute..end.minute {
        accumulated_cycle[i] += 1;
    }
}

pub fn parse_input(input: &str) -> Vec<Event> {
    lazy_static! {
        static ref re_input: Regex = Regex::new(r#"(?x)\[(\d{4}) # year
\-
(\d{2}) # month
\-
(\d{2}) # day
\p{White_Space}
(\d{2}):(\d{2}) # time
\]
\p{White_Space}
(.+) # rest of string
"#).expect("failed to create regex");

        static ref re_guard: Regex = Regex::new(r#"Guard\p{White_Space}\#(\d+)\p{White_Space}begins\p{White_Space}shift"#).expect("failed to create regex");
    }

    re_input.captures_iter(INPUT).map(|cap| {
        let year = &cap[1];
        let month = &cap[2];
        let day = &cap[3];
        let hour = &cap[4];
        let minute = &cap[5];
        let timestamp = Timestamp {
            year: year.parse().expect("failed to parse year"),
            month: month.parse().expect("failed to parse month"),
            day: day.parse().expect("failed to parse day"),
            hour: hour.parse().expect("failed to parse hour"),
            minute: minute.parse().expect("failed to parse minute"),
        };

        let action = &cap[6];
        match action {
            "falls asleep" =>
                Event {
                    timestamp: timestamp,
                    action: Action::FallsAsleep,
                },
            "wakes up" =>
                Event {
                    timestamp: timestamp,
                    action: Action::WakesUp,
                },
            _ => {
                let guard_cap = re_guard.captures(action).expect("failed to capture guard");
                Event {
                    timestamp: timestamp,
                    action: Action::BeginShift(guard_cap[1].parse::<u64>().expect("failed to parse guard id")),
                }
            }
        }
    }).collect()
}

#[derive(Clone, Debug)]
pub struct Event {
    pub timestamp: Timestamp,
    pub action: Action,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.timestamp, self.action)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Timestamp {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}-{}-{} {}:{}]", self.year, self.month, self.day, self.hour, self.minute)
    }
}

impl Timestamp {
    pub fn day(&self) -> String {
        format!("{}-{}-{}", self.year, self.month, self.day)
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    BeginShift(GuardId),
    FallsAsleep,
    WakesUp,
}

type GuardId = u64;
type GuardSleepCycle = [u64; 60];

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        let year_ordering = self.timestamp.year.cmp(&other.timestamp.year);
        let month_ordering = self.timestamp.month.cmp(&other.timestamp.month);
        let day_ordering = self.timestamp.day.cmp(&other.timestamp.day);
        // hour is inverted since the only hours are 23 and 00
        let hour_ordering = other.timestamp.hour.cmp(&self.timestamp.hour);
        let minute_ordering = self.timestamp.minute.cmp(&other.timestamp.minute);

        let orderings = [year_ordering, month_ordering, day_ordering, hour_ordering, minute_ordering];
        *orderings.iter()
            .find(|ordering| **ordering != Ordering::Equal)
            .unwrap_or(&Ordering::Equal)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for Event {}

static INPUT: &str = "[1518-05-30 00:27] wakes up
[1518-11-02 00:00] Guard #433 begins shift
[1518-07-05 23:56] Guard #2593 begins shift
[1518-03-31 00:55] falls asleep
[1518-06-20 00:17] falls asleep
[1518-09-10 00:00] Guard #277 begins shift
[1518-11-14 00:39] falls asleep
[1518-09-08 23:59] Guard #1709 begins shift
[1518-08-23 00:35] falls asleep
[1518-06-03 00:46] wakes up
[1518-02-27 00:59] wakes up
[1518-03-18 00:10] falls asleep
[1518-09-28 00:49] wakes up
[1518-05-19 00:55] wakes up
[1518-03-20 00:42] wakes up
[1518-06-03 00:00] Guard #1871 begins shift
[1518-07-04 00:38] wakes up
[1518-06-08 00:32] wakes up
[1518-08-13 00:04] Guard #1871 begins shift
[1518-05-24 00:00] Guard #739 begins shift
[1518-03-02 00:03] Guard #103 begins shift
[1518-04-09 00:44] wakes up
[1518-08-27 00:22] falls asleep
[1518-11-07 00:50] wakes up
[1518-05-30 00:16] falls asleep
[1518-09-12 00:49] wakes up
[1518-02-17 00:38] falls asleep
[1518-10-05 00:49] wakes up
[1518-05-28 00:01] falls asleep
[1518-03-31 00:52] wakes up
[1518-07-01 23:57] Guard #1051 begins shift
[1518-11-02 00:21] falls asleep
[1518-08-03 00:59] wakes up
[1518-03-15 00:57] falls asleep
[1518-08-20 00:34] falls asleep
[1518-08-19 00:59] wakes up
[1518-02-24 00:15] falls asleep
[1518-02-16 00:28] falls asleep
[1518-08-21 00:00] Guard #2251 begins shift
[1518-08-02 00:49] falls asleep
[1518-03-16 00:26] falls asleep
[1518-05-31 00:30] falls asleep
[1518-06-19 00:46] falls asleep
[1518-08-12 00:58] wakes up
[1518-10-31 00:53] falls asleep
[1518-02-23 00:54] wakes up
[1518-11-04 00:44] wakes up
[1518-07-12 00:00] Guard #239 begins shift
[1518-07-05 00:51] falls asleep
[1518-08-28 00:55] wakes up
[1518-03-13 00:00] Guard #727 begins shift
[1518-04-13 00:49] falls asleep
[1518-07-29 00:32] falls asleep
[1518-07-30 23:56] Guard #277 begins shift
[1518-03-26 00:46] falls asleep
[1518-11-21 00:03] Guard #1447 begins shift
[1518-11-14 00:04] falls asleep
[1518-09-09 00:55] wakes up
[1518-03-16 00:03] Guard #727 begins shift
[1518-05-08 23:46] Guard #1249 begins shift
[1518-06-28 00:53] wakes up
[1518-06-04 00:32] wakes up
[1518-10-08 00:55] wakes up
[1518-02-23 00:14] falls asleep
[1518-11-04 00:11] falls asleep
[1518-03-14 00:49] wakes up
[1518-11-19 00:02] Guard #1171 begins shift
[1518-11-01 00:12] wakes up
[1518-09-29 00:58] wakes up
[1518-10-02 00:56] wakes up
[1518-07-26 00:39] wakes up
[1518-10-13 00:59] wakes up
[1518-09-23 00:57] wakes up
[1518-03-14 00:59] wakes up
[1518-07-19 00:54] wakes up
[1518-05-03 00:57] wakes up
[1518-02-24 00:57] wakes up
[1518-10-03 00:41] falls asleep
[1518-08-09 00:03] Guard #2423 begins shift
[1518-09-05 00:40] wakes up
[1518-10-29 00:23] falls asleep
[1518-09-26 00:03] Guard #239 begins shift
[1518-10-31 00:32] falls asleep
[1518-04-13 00:18] falls asleep
[1518-10-02 00:12] falls asleep
[1518-06-11 00:54] falls asleep
[1518-05-21 00:38] falls asleep
[1518-09-18 00:59] wakes up
[1518-04-16 00:00] Guard #433 begins shift
[1518-09-20 00:47] wakes up
[1518-09-20 00:52] falls asleep
[1518-09-19 00:32] falls asleep
[1518-10-26 00:30] falls asleep
[1518-06-21 00:44] falls asleep
[1518-10-23 00:01] Guard #1249 begins shift
[1518-06-15 23:56] Guard #2689 begins shift
[1518-04-19 00:04] falls asleep
[1518-09-29 00:35] falls asleep
[1518-04-18 00:53] wakes up
[1518-11-15 00:29] falls asleep
[1518-06-05 00:58] wakes up
[1518-10-15 00:56] falls asleep
[1518-04-24 00:00] falls asleep
[1518-09-06 23:48] Guard #2689 begins shift
[1518-08-25 00:47] falls asleep
[1518-03-27 00:00] falls asleep
[1518-11-13 23:53] Guard #1097 begins shift
[1518-03-14 00:08] falls asleep
[1518-04-02 00:52] wakes up
[1518-06-20 00:58] wakes up
[1518-02-24 00:40] falls asleep
[1518-09-07 00:04] falls asleep
[1518-04-20 00:03] Guard #2593 begins shift
[1518-04-29 00:57] wakes up
[1518-10-13 00:29] falls asleep
[1518-08-18 00:19] falls asleep
[1518-02-27 00:46] wakes up
[1518-03-30 00:15] falls asleep
[1518-05-23 00:40] wakes up
[1518-02-22 00:56] falls asleep
[1518-06-28 00:35] falls asleep
[1518-06-11 00:48] wakes up
[1518-03-22 23:56] Guard #433 begins shift
[1518-04-02 23:58] Guard #2251 begins shift
[1518-07-24 23:58] Guard #2593 begins shift
[1518-09-22 00:13] falls asleep
[1518-10-06 00:17] falls asleep
[1518-10-26 00:00] Guard #109 begins shift
[1518-09-20 00:24] falls asleep
[1518-03-06 00:39] falls asleep
[1518-07-02 00:45] wakes up
[1518-06-30 00:44] falls asleep
[1518-11-12 23:59] Guard #2251 begins shift
[1518-05-31 00:54] wakes up
[1518-06-30 00:56] wakes up
[1518-11-07 00:15] wakes up
[1518-04-09 23:50] Guard #103 begins shift
[1518-03-23 00:34] wakes up
[1518-06-27 00:59] wakes up
[1518-03-08 00:43] falls asleep
[1518-03-03 00:54] wakes up
[1518-04-07 00:14] falls asleep
[1518-11-10 00:43] wakes up
[1518-09-30 00:52] wakes up
[1518-11-21 00:58] wakes up
[1518-05-18 00:39] falls asleep
[1518-03-09 23:57] Guard #1871 begins shift
[1518-05-05 00:46] falls asleep
[1518-03-25 00:46] wakes up
[1518-04-23 00:28] falls asleep
[1518-06-19 00:54] wakes up
[1518-08-28 23:58] Guard #1447 begins shift
[1518-04-20 00:53] wakes up
[1518-04-05 00:51] wakes up
[1518-04-18 23:50] Guard #2423 begins shift
[1518-02-28 23:56] Guard #677 begins shift
[1518-09-06 00:02] Guard #1097 begins shift
[1518-09-30 00:45] wakes up
[1518-08-15 00:07] falls asleep
[1518-10-10 00:04] Guard #1051 begins shift
[1518-10-16 00:38] wakes up
[1518-07-02 00:41] falls asleep
[1518-06-11 00:56] wakes up
[1518-10-14 00:13] falls asleep
[1518-05-06 00:32] wakes up
[1518-10-04 00:42] wakes up
[1518-09-05 00:12] falls asleep
[1518-05-03 00:55] falls asleep
[1518-11-15 00:18] falls asleep
[1518-08-11 00:02] Guard #103 begins shift
[1518-05-08 00:40] falls asleep
[1518-08-19 00:53] wakes up
[1518-08-06 23:57] Guard #2887 begins shift
[1518-05-08 00:04] Guard #2251 begins shift
[1518-11-18 00:27] falls asleep
[1518-04-01 00:26] falls asleep
[1518-03-28 00:28] wakes up
[1518-05-02 00:28] falls asleep
[1518-10-01 00:26] falls asleep
[1518-07-28 00:59] wakes up
[1518-07-15 00:00] Guard #727 begins shift
[1518-07-10 00:49] falls asleep
[1518-07-26 00:12] falls asleep
[1518-07-13 00:01] falls asleep
[1518-08-25 00:57] wakes up
[1518-09-26 23:52] Guard #2593 begins shift
[1518-11-22 23:59] Guard #1097 begins shift
[1518-09-21 00:44] wakes up
[1518-07-24 00:51] wakes up
[1518-10-24 00:24] falls asleep
[1518-09-25 00:00] Guard #109 begins shift
[1518-09-16 00:01] Guard #1709 begins shift
[1518-08-19 00:56] falls asleep
[1518-09-29 00:54] wakes up
[1518-08-01 00:00] Guard #239 begins shift
[1518-06-22 00:31] falls asleep
[1518-11-11 00:01] falls asleep
[1518-07-27 00:01] Guard #2887 begins shift
[1518-03-15 00:45] wakes up
[1518-03-04 00:26] falls asleep
[1518-03-10 00:50] falls asleep
[1518-02-21 00:17] wakes up
[1518-10-19 00:14] falls asleep
[1518-10-02 23:57] Guard #739 begins shift
[1518-05-16 00:02] Guard #727 begins shift
[1518-05-05 00:30] falls asleep
[1518-08-17 23:56] Guard #109 begins shift
[1518-04-14 00:51] falls asleep
[1518-05-18 00:22] wakes up
[1518-04-01 00:57] wakes up
[1518-10-14 00:02] Guard #2251 begins shift
[1518-09-03 00:47] wakes up
[1518-08-23 00:02] Guard #433 begins shift
[1518-05-04 00:52] falls asleep
[1518-02-18 00:57] wakes up
[1518-06-12 00:00] Guard #727 begins shift
[1518-06-03 23:59] Guard #433 begins shift
[1518-05-02 23:59] Guard #2251 begins shift
[1518-09-10 00:25] falls asleep
[1518-06-06 00:01] Guard #2423 begins shift
[1518-10-15 00:28] wakes up
[1518-07-18 00:37] falls asleep
[1518-04-09 00:00] Guard #2887 begins shift
[1518-08-04 00:01] Guard #2423 begins shift
[1518-07-21 00:19] falls asleep
[1518-08-08 00:40] wakes up
[1518-07-14 00:49] wakes up
[1518-07-01 00:54] wakes up
[1518-04-16 00:33] falls asleep
[1518-08-03 00:43] wakes up
[1518-10-17 00:24] falls asleep
[1518-09-04 00:03] falls asleep
[1518-10-22 00:58] wakes up
[1518-11-05 00:37] wakes up
[1518-06-09 00:01] Guard #677 begins shift
[1518-09-27 00:22] falls asleep
[1518-04-23 00:47] wakes up
[1518-03-14 23:51] Guard #2689 begins shift
[1518-04-21 00:38] falls asleep
[1518-05-13 00:54] wakes up
[1518-05-26 00:40] falls asleep
[1518-07-05 00:00] Guard #2423 begins shift
[1518-04-16 23:58] Guard #2423 begins shift
[1518-06-15 00:59] wakes up
[1518-11-02 00:23] wakes up
[1518-05-27 23:53] Guard #1871 begins shift
[1518-09-26 00:20] falls asleep
[1518-03-30 00:38] wakes up
[1518-04-02 00:26] falls asleep
[1518-02-27 00:04] Guard #1171 begins shift
[1518-02-19 00:34] wakes up
[1518-08-10 00:32] falls asleep
[1518-08-30 00:01] Guard #1447 begins shift
[1518-08-26 00:02] Guard #727 begins shift
[1518-11-14 00:48] wakes up
[1518-04-17 00:37] wakes up
[1518-04-25 00:56] wakes up
[1518-06-27 00:41] falls asleep
[1518-07-16 00:40] falls asleep
[1518-08-01 00:08] falls asleep
[1518-08-08 00:30] falls asleep
[1518-03-16 00:57] wakes up
[1518-07-07 00:42] wakes up
[1518-08-27 00:10] falls asleep
[1518-02-23 00:42] falls asleep
[1518-04-10 00:35] falls asleep
[1518-11-11 00:39] wakes up
[1518-07-03 00:47] wakes up
[1518-11-07 00:12] falls asleep
[1518-07-11 00:46] falls asleep
[1518-09-07 00:53] wakes up
[1518-05-26 00:18] falls asleep
[1518-07-10 00:57] wakes up
[1518-03-17 00:02] Guard #109 begins shift
[1518-04-27 00:14] falls asleep
[1518-09-23 00:12] wakes up
[1518-07-10 00:05] falls asleep
[1518-08-06 00:38] falls asleep
[1518-07-15 23:59] Guard #677 begins shift
[1518-09-14 23:50] Guard #2251 begins shift
[1518-04-29 00:49] falls asleep
[1518-09-05 00:03] Guard #2689 begins shift
[1518-11-17 00:50] wakes up
[1518-04-20 00:56] falls asleep
[1518-02-21 00:08] falls asleep
[1518-08-20 00:56] wakes up
[1518-03-07 00:32] wakes up
[1518-07-15 00:18] falls asleep
[1518-05-14 00:43] wakes up
[1518-07-12 00:59] wakes up
[1518-10-30 00:34] falls asleep
[1518-10-23 00:56] wakes up
[1518-02-23 00:25] wakes up
[1518-03-12 00:08] falls asleep
[1518-08-21 00:37] wakes up
[1518-06-20 23:56] Guard #2689 begins shift
[1518-06-22 00:42] falls asleep
[1518-06-04 00:54] falls asleep
[1518-05-22 00:31] falls asleep
[1518-04-25 00:02] Guard #1097 begins shift
[1518-03-03 00:31] falls asleep
[1518-08-16 00:49] wakes up
[1518-11-07 00:26] falls asleep
[1518-11-09 00:26] falls asleep
[1518-09-17 00:17] falls asleep
[1518-04-06 00:17] falls asleep
[1518-04-11 00:18] falls asleep
[1518-10-19 00:00] Guard #2887 begins shift
[1518-02-22 00:48] wakes up
[1518-03-31 00:57] wakes up
[1518-07-08 00:38] wakes up
[1518-03-13 00:50] wakes up
[1518-03-01 00:58] wakes up
[1518-11-05 00:27] falls asleep
[1518-03-12 00:19] wakes up
[1518-07-09 00:31] falls asleep
[1518-05-05 00:40] wakes up
[1518-08-05 00:48] wakes up
[1518-08-12 00:23] wakes up
[1518-03-01 00:47] wakes up
[1518-05-14 00:56] falls asleep
[1518-05-18 23:46] Guard #2251 begins shift
[1518-07-13 00:34] wakes up
[1518-05-11 00:54] wakes up
[1518-05-12 00:53] falls asleep
[1518-10-10 00:49] falls asleep
[1518-10-19 23:57] Guard #103 begins shift
[1518-03-22 00:01] Guard #1447 begins shift
[1518-08-30 00:23] falls asleep
[1518-04-25 00:25] wakes up
[1518-07-09 00:42] wakes up
[1518-06-07 00:02] Guard #677 begins shift
[1518-09-06 00:10] falls asleep
[1518-10-31 00:56] wakes up
[1518-09-19 00:55] wakes up
[1518-08-19 00:48] falls asleep
[1518-03-04 00:13] falls asleep
[1518-05-17 23:59] Guard #2593 begins shift
[1518-06-21 23:59] Guard #2423 begins shift
[1518-08-18 00:57] wakes up
[1518-03-28 00:01] falls asleep
[1518-06-25 23:59] Guard #1171 begins shift
[1518-04-23 00:23] wakes up
[1518-10-12 00:56] wakes up
[1518-10-25 00:18] falls asleep
[1518-07-05 00:53] wakes up
[1518-11-02 00:57] wakes up
[1518-10-11 00:36] falls asleep
[1518-09-30 23:57] Guard #1097 begins shift
[1518-09-25 00:29] wakes up
[1518-07-18 00:40] wakes up
[1518-03-04 00:35] falls asleep
[1518-08-14 00:12] falls asleep
[1518-08-29 00:25] falls asleep
[1518-05-01 00:33] falls asleep
[1518-09-22 23:58] Guard #727 begins shift
[1518-11-18 00:04] Guard #1709 begins shift
[1518-11-01 00:54] wakes up
[1518-04-11 00:50] wakes up
[1518-11-03 00:49] wakes up
[1518-09-03 00:39] falls asleep
[1518-04-11 00:01] Guard #2593 begins shift
[1518-09-27 00:29] wakes up
[1518-03-24 23:59] Guard #2593 begins shift
[1518-06-28 00:28] wakes up
[1518-11-14 00:59] wakes up
[1518-04-04 00:48] wakes up
[1518-04-01 00:35] falls asleep
[1518-06-08 00:15] wakes up
[1518-05-24 00:28] falls asleep
[1518-06-03 00:25] falls asleep
[1518-09-15 00:58] wakes up
[1518-02-26 00:58] wakes up
[1518-06-07 23:57] Guard #2423 begins shift
[1518-06-05 00:08] falls asleep
[1518-06-22 00:58] wakes up
[1518-09-23 00:53] wakes up
[1518-02-28 00:53] wakes up
[1518-03-03 23:56] Guard #1447 begins shift
[1518-05-01 00:01] Guard #2251 begins shift
[1518-08-24 23:57] Guard #1709 begins shift
[1518-11-05 23:47] Guard #277 begins shift
[1518-05-11 00:44] wakes up
[1518-04-13 23:50] Guard #727 begins shift
[1518-05-20 00:44] wakes up
[1518-05-07 00:55] wakes up
[1518-03-17 00:24] falls asleep
[1518-06-18 00:08] wakes up
[1518-03-12 00:23] falls asleep
[1518-11-09 00:34] wakes up
[1518-11-03 00:00] Guard #433 begins shift
[1518-10-03 00:10] falls asleep
[1518-05-29 00:04] Guard #2251 begins shift
[1518-07-02 00:19] wakes up
[1518-03-04 00:18] wakes up
[1518-07-01 00:37] falls asleep
[1518-10-09 00:40] wakes up
[1518-11-10 00:02] falls asleep
[1518-05-14 00:53] wakes up
[1518-02-25 00:31] wakes up
[1518-04-24 00:50] wakes up
[1518-09-04 00:40] wakes up
[1518-03-28 00:55] wakes up
[1518-10-30 00:26] wakes up
[1518-10-06 00:54] wakes up
[1518-10-25 00:50] wakes up
[1518-03-22 00:23] falls asleep
[1518-10-11 00:55] wakes up
[1518-04-30 00:04] Guard #2251 begins shift
[1518-06-29 00:44] falls asleep
[1518-03-04 00:27] wakes up
[1518-02-27 00:57] falls asleep
[1518-03-06 00:54] wakes up
[1518-08-28 00:40] falls asleep
[1518-04-19 00:22] wakes up
[1518-06-05 00:30] wakes up
[1518-03-19 00:36] wakes up
[1518-11-09 23:46] Guard #2689 begins shift
[1518-09-12 00:45] falls asleep
[1518-03-24 00:00] Guard #2423 begins shift
[1518-10-28 00:58] wakes up
[1518-11-19 00:19] falls asleep
[1518-10-22 00:31] wakes up
[1518-04-20 00:58] wakes up
[1518-04-21 00:57] wakes up
[1518-10-14 00:55] wakes up
[1518-09-26 00:49] wakes up
[1518-05-17 00:50] falls asleep
[1518-07-03 00:28] falls asleep
[1518-11-22 00:03] Guard #1871 begins shift
[1518-04-04 00:29] falls asleep
[1518-08-22 00:58] wakes up
[1518-10-06 00:22] wakes up
[1518-07-28 00:55] falls asleep
[1518-08-12 00:55] falls asleep
[1518-08-13 23:59] Guard #433 begins shift
[1518-09-22 00:00] Guard #433 begins shift
[1518-08-15 00:13] wakes up
[1518-10-05 00:03] Guard #2689 begins shift
[1518-06-05 00:38] falls asleep
[1518-10-06 00:03] Guard #1097 begins shift
[1518-03-23 00:11] falls asleep
[1518-06-25 00:34] wakes up
[1518-11-21 00:27] wakes up
[1518-06-22 00:39] wakes up
[1518-08-22 00:41] wakes up
[1518-09-12 23:56] Guard #2689 begins shift
[1518-11-05 00:43] falls asleep
[1518-08-06 00:04] Guard #727 begins shift
[1518-05-04 00:02] Guard #1447 begins shift
[1518-10-21 00:57] wakes up
[1518-04-21 00:11] falls asleep
[1518-10-04 00:31] falls asleep
[1518-08-05 00:10] falls asleep
[1518-11-06 00:48] wakes up
[1518-05-25 00:03] Guard #1171 begins shift
[1518-02-24 00:44] falls asleep
[1518-05-06 00:05] falls asleep
[1518-05-09 00:26] wakes up
[1518-07-15 00:58] wakes up
[1518-11-07 00:57] falls asleep
[1518-10-31 00:46] wakes up
[1518-07-01 00:57] falls asleep
[1518-04-15 00:17] falls asleep
[1518-08-31 00:41] falls asleep
[1518-09-26 00:43] wakes up
[1518-08-09 00:47] wakes up
[1518-10-28 00:31] falls asleep
[1518-05-24 00:59] wakes up
[1518-02-15 23:57] Guard #433 begins shift
[1518-05-31 00:02] Guard #727 begins shift
[1518-05-25 00:56] wakes up
[1518-05-14 00:01] Guard #1709 begins shift
[1518-11-22 00:30] wakes up
[1518-07-28 00:27] wakes up
[1518-04-20 00:41] falls asleep
[1518-04-10 00:48] wakes up
[1518-08-19 00:08] falls asleep
[1518-03-29 00:45] wakes up
[1518-07-29 00:03] Guard #2423 begins shift
[1518-10-26 00:54] wakes up
[1518-07-21 00:48] falls asleep
[1518-07-05 00:30] falls asleep
[1518-04-15 00:26] wakes up
[1518-07-06 00:47] wakes up
[1518-11-07 00:58] wakes up
[1518-10-09 00:25] falls asleep
[1518-10-30 00:37] wakes up
[1518-09-18 00:03] Guard #2593 begins shift
[1518-10-28 23:57] Guard #1171 begins shift
[1518-02-24 00:41] wakes up
[1518-05-02 00:29] wakes up
[1518-10-17 00:40] wakes up
[1518-06-18 00:57] wakes up
[1518-06-14 00:59] wakes up
[1518-07-07 00:21] falls asleep
[1518-07-02 00:14] falls asleep
[1518-03-22 00:51] falls asleep
[1518-11-12 00:23] wakes up
[1518-08-21 00:33] falls asleep
[1518-07-11 00:52] wakes up
[1518-03-15 00:01] falls asleep
[1518-09-07 23:46] Guard #727 begins shift
[1518-07-04 00:20] falls asleep
[1518-09-13 00:35] wakes up
[1518-11-04 23:58] Guard #239 begins shift
[1518-07-03 00:00] Guard #2689 begins shift
[1518-05-11 00:15] falls asleep
[1518-08-12 00:04] Guard #1097 begins shift
[1518-11-13 00:55] wakes up
[1518-03-24 00:42] wakes up
[1518-07-22 00:16] falls asleep
[1518-07-27 00:46] wakes up
[1518-02-16 00:55] wakes up
[1518-09-23 00:08] falls asleep
[1518-11-12 00:00] Guard #277 begins shift
[1518-08-03 00:04] falls asleep
[1518-04-21 00:01] Guard #2689 begins shift
[1518-09-03 00:03] Guard #2423 begins shift
[1518-06-26 23:58] Guard #2689 begins shift
[1518-03-29 00:00] Guard #2251 begins shift
[1518-05-19 23:59] Guard #1171 begins shift
[1518-05-16 23:58] Guard #2251 begins shift
[1518-08-03 00:52] falls asleep
[1518-03-03 00:01] Guard #2887 begins shift
[1518-07-26 00:54] wakes up
[1518-09-03 23:46] Guard #1097 begins shift
[1518-04-26 00:03] Guard #821 begins shift
[1518-04-08 00:39] wakes up
[1518-05-30 00:00] Guard #1871 begins shift
[1518-11-12 00:18] falls asleep
[1518-06-04 00:10] falls asleep
[1518-07-12 23:53] Guard #2689 begins shift
[1518-07-30 00:00] Guard #239 begins shift
[1518-08-20 00:00] Guard #1871 begins shift
[1518-06-13 23:58] Guard #1171 begins shift
[1518-08-25 00:25] wakes up
[1518-06-23 00:00] Guard #433 begins shift
[1518-09-21 00:43] falls asleep
[1518-10-16 00:23] falls asleep
[1518-10-10 00:38] falls asleep
[1518-08-30 23:57] Guard #2887 begins shift
[1518-04-05 23:58] Guard #2887 begins shift
[1518-05-02 00:36] falls asleep
[1518-10-03 00:15] wakes up
[1518-09-02 00:58] wakes up
[1518-03-19 00:04] Guard #103 begins shift
[1518-04-19 00:39] falls asleep
[1518-10-01 23:58] Guard #1709 begins shift
[1518-03-11 00:13] wakes up
[1518-10-06 00:49] falls asleep
[1518-11-21 00:39] falls asleep
[1518-07-14 00:16] falls asleep
[1518-09-30 00:31] falls asleep
[1518-02-24 00:03] Guard #1097 begins shift
[1518-08-19 00:00] Guard #1051 begins shift
[1518-05-07 00:42] falls asleep
[1518-04-22 23:58] Guard #2593 begins shift
[1518-04-05 00:00] Guard #739 begins shift
[1518-11-08 00:23] falls asleep
[1518-03-12 00:01] Guard #109 begins shift
[1518-04-28 00:37] falls asleep
[1518-07-22 23:58] Guard #281 begins shift
[1518-07-27 00:43] falls asleep
[1518-06-16 00:59] wakes up
[1518-07-24 00:49] falls asleep
[1518-03-11 00:10] falls asleep
[1518-07-06 00:51] falls asleep
[1518-11-08 00:54] wakes up
[1518-11-21 00:24] falls asleep
[1518-07-21 00:53] wakes up
[1518-05-11 00:48] falls asleep
[1518-06-21 00:55] wakes up
[1518-09-24 00:03] Guard #1249 begins shift
[1518-07-14 00:00] Guard #1871 begins shift
[1518-05-29 00:10] falls asleep
[1518-03-14 00:40] falls asleep
[1518-03-09 00:01] Guard #2423 begins shift
[1518-07-26 00:48] falls asleep
[1518-04-03 00:44] falls asleep
[1518-05-14 00:38] falls asleep
[1518-05-28 00:48] falls asleep
[1518-06-02 00:54] wakes up
[1518-05-02 00:57] wakes up
[1518-06-19 23:58] Guard #727 begins shift
[1518-02-27 00:45] falls asleep
[1518-10-24 00:04] Guard #1447 begins shift
[1518-07-12 00:47] falls asleep
[1518-09-08 00:23] wakes up
[1518-10-26 00:57] falls asleep
[1518-08-06 00:58] wakes up
[1518-05-12 00:57] wakes up
[1518-03-07 00:53] wakes up
[1518-09-04 00:55] wakes up
[1518-07-28 00:12] falls asleep
[1518-03-16 00:52] wakes up
[1518-09-03 00:31] wakes up
[1518-09-24 00:58] wakes up
[1518-09-15 00:56] falls asleep
[1518-09-25 00:08] falls asleep
[1518-08-07 00:36] falls asleep
[1518-04-27 00:02] Guard #739 begins shift
[1518-08-18 00:30] wakes up
[1518-08-31 00:44] wakes up
[1518-09-06 00:52] wakes up
[1518-06-06 00:38] wakes up
[1518-02-20 00:02] Guard #239 begins shift
[1518-03-07 23:57] Guard #2593 begins shift
[1518-08-02 00:11] falls asleep
[1518-08-16 00:47] falls asleep
[1518-09-04 00:47] falls asleep
[1518-09-16 00:56] wakes up
[1518-02-18 00:02] Guard #277 begins shift
[1518-04-25 00:55] falls asleep
[1518-11-19 00:31] wakes up
[1518-02-25 00:18] falls asleep
[1518-06-16 00:07] falls asleep
[1518-03-18 00:00] Guard #2423 begins shift
[1518-10-24 00:41] falls asleep
[1518-02-26 00:57] falls asleep
[1518-08-05 00:02] Guard #2593 begins shift
[1518-05-27 00:00] Guard #1871 begins shift
[1518-04-18 00:47] falls asleep
[1518-04-14 00:00] falls asleep
[1518-03-09 00:16] falls asleep
[1518-04-01 00:38] wakes up
[1518-05-16 00:59] wakes up
[1518-11-01 00:02] falls asleep
[1518-04-04 00:18] wakes up
[1518-05-31 23:57] Guard #281 begins shift
[1518-02-17 00:42] wakes up
[1518-09-19 00:50] wakes up
[1518-05-01 00:50] wakes up
[1518-09-05 00:28] falls asleep
[1518-11-22 00:11] falls asleep
[1518-04-28 00:00] Guard #433 begins shift
[1518-06-12 00:58] wakes up
[1518-10-01 00:39] wakes up
[1518-04-29 00:01] Guard #2251 begins shift
[1518-09-11 00:28] falls asleep
[1518-05-28 00:57] wakes up
[1518-09-01 00:57] wakes up
[1518-06-13 00:47] wakes up
[1518-05-16 00:50] falls asleep
[1518-11-16 00:03] Guard #677 begins shift
[1518-06-08 00:55] wakes up
[1518-09-08 00:16] falls asleep
[1518-06-24 00:40] falls asleep
[1518-09-21 00:27] wakes up
[1518-07-06 00:57] wakes up
[1518-11-10 23:50] Guard #739 begins shift
[1518-03-13 00:49] falls asleep
[1518-10-15 00:04] Guard #1249 begins shift
[1518-07-10 23:57] Guard #239 begins shift
[1518-02-19 00:01] Guard #109 begins shift
[1518-06-16 23:58] Guard #2251 begins shift
[1518-10-27 23:56] Guard #1051 begins shift
[1518-09-13 00:59] wakes up
[1518-09-14 00:17] falls asleep
[1518-04-30 00:56] wakes up
[1518-08-16 00:21] falls asleep
[1518-05-10 00:55] wakes up
[1518-08-10 00:44] wakes up
[1518-09-08 00:00] falls asleep
[1518-04-10 00:22] wakes up
[1518-09-14 00:40] wakes up
[1518-09-26 00:58] wakes up
[1518-10-31 00:00] Guard #103 begins shift
[1518-07-07 00:00] Guard #2689 begins shift
[1518-06-26 00:24] wakes up
[1518-11-01 00:36] falls asleep
[1518-09-16 00:43] falls asleep
[1518-08-18 00:33] falls asleep
[1518-09-09 00:45] falls asleep
[1518-03-30 00:49] wakes up
[1518-09-05 00:13] wakes up
[1518-07-20 00:51] wakes up
[1518-06-29 00:04] Guard #433 begins shift
[1518-08-27 00:12] wakes up
[1518-06-14 00:43] falls asleep
[1518-07-06 00:39] falls asleep
[1518-10-10 00:44] wakes up
[1518-10-13 00:00] Guard #1871 begins shift
[1518-06-06 00:16] falls asleep
[1518-05-12 00:32] falls asleep
[1518-07-16 23:50] Guard #2423 begins shift
[1518-04-07 00:52] wakes up
[1518-09-26 00:55] falls asleep
[1518-09-29 00:57] falls asleep
[1518-07-20 00:15] falls asleep
[1518-06-26 00:44] wakes up
[1518-04-06 23:59] Guard #1097 begins shift
[1518-03-30 00:02] Guard #739 begins shift
[1518-08-02 00:30] wakes up
[1518-05-07 00:01] Guard #2887 begins shift
[1518-03-21 00:01] Guard #281 begins shift
[1518-07-08 23:56] Guard #1871 begins shift
[1518-07-05 00:40] wakes up
[1518-08-02 23:51] Guard #109 begins shift
[1518-08-08 00:01] Guard #677 begins shift
[1518-03-27 23:49] Guard #1447 begins shift
[1518-09-01 00:17] falls asleep
[1518-03-04 23:59] Guard #239 begins shift
[1518-06-29 00:45] wakes up
[1518-05-14 00:52] falls asleep
[1518-06-02 00:04] Guard #1871 begins shift
[1518-09-23 00:42] falls asleep
[1518-08-17 00:28] falls asleep
[1518-08-23 00:50] wakes up
[1518-09-10 23:51] Guard #1871 begins shift
[1518-04-04 00:05] falls asleep
[1518-10-04 00:00] Guard #2593 begins shift
[1518-10-12 00:42] falls asleep
[1518-04-25 00:44] falls asleep
[1518-06-23 00:42] wakes up
[1518-03-14 00:53] falls asleep
[1518-08-19 00:15] wakes up
[1518-08-12 00:18] falls asleep
[1518-10-24 00:47] wakes up
[1518-10-27 00:11] falls asleep
[1518-10-10 23:57] Guard #739 begins shift
[1518-09-03 00:28] falls asleep
[1518-05-05 23:51] Guard #1171 begins shift
[1518-03-20 00:37] falls asleep
[1518-07-31 00:13] falls asleep
[1518-07-25 00:47] falls asleep
[1518-05-26 00:47] wakes up
[1518-08-22 00:19] falls asleep
[1518-10-22 00:44] wakes up
[1518-04-27 00:58] wakes up
[1518-08-09 00:32] falls asleep
[1518-06-08 00:28] falls asleep
[1518-10-15 00:43] falls asleep
[1518-09-08 00:12] wakes up
[1518-05-09 00:02] falls asleep
[1518-03-24 00:30] falls asleep
[1518-03-26 23:48] Guard #2689 begins shift
[1518-02-22 00:47] falls asleep
[1518-08-04 00:58] wakes up
[1518-07-31 00:36] wakes up
[1518-08-12 00:51] wakes up
[1518-04-19 00:11] wakes up
[1518-06-30 00:00] Guard #2593 begins shift
[1518-07-25 00:53] wakes up
[1518-03-01 00:54] falls asleep
[1518-05-21 00:57] wakes up
[1518-05-10 00:02] Guard #239 begins shift
[1518-04-21 00:19] wakes up
[1518-02-20 00:13] falls asleep
[1518-06-15 00:36] falls asleep
[1518-05-13 00:35] falls asleep
[1518-02-25 23:56] Guard #2251 begins shift
[1518-10-06 00:25] falls asleep
[1518-03-17 00:56] wakes up
[1518-10-05 00:41] falls asleep
[1518-07-19 00:53] falls asleep
[1518-05-04 23:57] Guard #2593 begins shift
[1518-03-26 00:57] wakes up
[1518-10-23 00:50] falls asleep
[1518-03-01 00:45] falls asleep
[1518-07-30 00:57] wakes up
[1518-05-29 00:48] wakes up
[1518-08-26 23:58] Guard #727 begins shift
[1518-07-21 00:24] wakes up
[1518-09-15 00:16] wakes up
[1518-10-25 00:03] Guard #2251 begins shift
[1518-03-11 00:40] wakes up
[1518-06-29 00:25] falls asleep
[1518-03-13 00:36] wakes up
[1518-05-18 00:50] wakes up
[1518-02-22 00:18] falls asleep
[1518-09-28 00:02] Guard #1171 begins shift
[1518-09-20 23:59] Guard #2423 begins shift
[1518-03-27 00:44] wakes up
[1518-11-13 00:51] falls asleep
[1518-03-07 00:38] falls asleep
[1518-04-14 00:26] wakes up
[1518-03-13 23:59] Guard #1097 begins shift
[1518-03-26 00:36] wakes up
[1518-08-16 00:26] wakes up
[1518-06-05 00:00] Guard #1447 begins shift
[1518-11-15 00:02] Guard #1097 begins shift
[1518-05-14 00:25] falls asleep
[1518-09-13 00:29] falls asleep
[1518-06-29 00:33] wakes up
[1518-10-24 00:29] wakes up
[1518-05-23 00:02] Guard #1871 begins shift
[1518-11-12 00:52] wakes up
[1518-10-20 23:59] Guard #727 begins shift
[1518-06-14 23:58] Guard #2251 begins shift
[1518-11-04 00:20] falls asleep
[1518-05-04 00:56] wakes up
[1518-06-14 00:40] wakes up
[1518-10-20 00:17] falls asleep
[1518-03-04 00:54] wakes up
[1518-06-09 00:59] wakes up
[1518-09-22 00:51] wakes up
[1518-11-12 00:26] falls asleep
[1518-09-11 00:45] wakes up
[1518-05-08 00:46] wakes up
[1518-05-14 00:32] wakes up
[1518-06-07 00:10] falls asleep
[1518-09-28 00:20] falls asleep
[1518-05-02 00:00] Guard #1171 begins shift
[1518-08-10 00:49] falls asleep
[1518-11-14 00:24] wakes up
[1518-09-20 00:57] wakes up
[1518-02-24 00:27] wakes up
[1518-07-08 00:03] Guard #677 begins shift
[1518-08-10 00:53] wakes up
[1518-03-22 00:54] wakes up
[1518-02-21 00:21] falls asleep
[1518-03-15 00:58] wakes up
[1518-05-25 00:55] falls asleep
[1518-10-22 00:43] falls asleep
[1518-08-04 00:33] falls asleep
[1518-07-22 00:00] Guard #1249 begins shift
[1518-08-01 00:53] wakes up
[1518-04-06 00:55] wakes up
[1518-05-20 00:58] wakes up
[1518-10-20 00:55] wakes up
[1518-04-13 00:03] Guard #1447 begins shift
[1518-09-29 23:59] Guard #2593 begins shift
[1518-06-25 00:30] falls asleep
[1518-08-22 00:46] falls asleep
[1518-05-14 00:59] wakes up
[1518-09-27 00:00] falls asleep
[1518-10-21 00:34] falls asleep
[1518-07-09 23:48] Guard #1249 begins shift
[1518-06-13 00:00] Guard #739 begins shift
[1518-11-08 00:04] Guard #739 begins shift
[1518-11-09 00:02] Guard #103 begins shift
[1518-08-24 00:23] falls asleep
[1518-10-08 00:45] falls asleep
[1518-07-21 00:00] Guard #1097 begins shift
[1518-06-19 00:23] falls asleep
[1518-05-15 00:43] wakes up
[1518-04-25 00:20] falls asleep
[1518-08-02 00:01] Guard #239 begins shift
[1518-07-03 00:25] wakes up
[1518-10-18 00:01] Guard #281 begins shift
[1518-06-24 00:06] falls asleep
[1518-07-26 00:00] Guard #677 begins shift
[1518-10-06 00:38] wakes up
[1518-10-10 00:57] wakes up
[1518-04-01 00:44] falls asleep
[1518-05-17 00:57] wakes up
[1518-04-27 00:48] wakes up
[1518-05-20 23:56] Guard #727 begins shift
[1518-11-04 00:15] wakes up
[1518-09-24 00:52] falls asleep
[1518-11-02 00:55] falls asleep
[1518-04-08 00:17] falls asleep
[1518-06-25 00:00] Guard #677 begins shift
[1518-11-17 00:02] Guard #739 begins shift
[1518-04-25 00:52] wakes up
[1518-09-10 00:52] wakes up
[1518-03-13 00:07] falls asleep
[1518-09-23 00:18] falls asleep
[1518-06-10 00:25] falls asleep
[1518-10-07 00:55] wakes up
[1518-10-27 00:16] wakes up
[1518-11-10 00:10] wakes up
[1518-10-31 23:50] Guard #109 begins shift
[1518-09-19 00:54] falls asleep
[1518-06-18 00:16] falls asleep
[1518-08-25 00:23] falls asleep
[1518-10-15 00:52] wakes up
[1518-07-20 00:02] Guard #1097 begins shift
[1518-02-16 00:45] wakes up
[1518-10-30 00:00] Guard #2887 begins shift
[1518-04-01 23:56] Guard #1447 begins shift
[1518-05-15 00:12] falls asleep
[1518-08-17 00:34] wakes up
[1518-05-19 00:01] falls asleep
[1518-10-16 00:02] Guard #727 begins shift
[1518-08-29 00:49] wakes up
[1518-04-12 00:23] falls asleep
[1518-10-17 00:02] Guard #2689 begins shift
[1518-03-28 00:38] falls asleep
[1518-06-18 00:01] falls asleep
[1518-09-23 00:33] wakes up
[1518-07-08 00:35] falls asleep
[1518-09-01 00:01] Guard #727 begins shift
[1518-02-22 00:38] wakes up
[1518-08-14 00:55] wakes up
[1518-11-07 00:04] Guard #2251 begins shift
[1518-07-17 00:05] falls asleep
[1518-02-20 00:48] wakes up
[1518-09-14 00:00] Guard #103 begins shift
[1518-11-10 00:58] wakes up
[1518-08-15 00:01] Guard #433 begins shift
[1518-05-16 00:17] falls asleep
[1518-07-28 00:00] Guard #1249 begins shift
[1518-04-03 23:52] Guard #1871 begins shift
[1518-10-22 00:02] Guard #2887 begins shift
[1518-09-27 00:19] wakes up
[1518-04-05 00:35] falls asleep
[1518-11-03 00:39] falls asleep
[1518-03-16 00:55] falls asleep
[1518-09-21 00:16] falls asleep
[1518-08-22 00:04] Guard #2887 begins shift
[1518-07-18 23:59] Guard #1709 begins shift
[1518-07-10 00:43] wakes up
[1518-03-02 00:38] falls asleep
[1518-05-27 00:52] falls asleep
[1518-02-25 00:00] Guard #109 begins shift
[1518-11-03 23:56] Guard #1447 begins shift
[1518-06-17 00:17] falls asleep
[1518-05-28 00:37] wakes up
[1518-06-26 00:40] falls asleep
[1518-06-09 00:53] falls asleep
[1518-07-03 00:22] falls asleep
[1518-04-13 00:45] wakes up
[1518-04-09 00:36] falls asleep
[1518-09-17 00:00] Guard #677 begins shift
[1518-09-26 00:47] falls asleep
[1518-02-27 23:59] Guard #2887 begins shift
[1518-10-22 00:54] falls asleep
[1518-09-23 00:56] falls asleep
[1518-10-19 00:46] wakes up
[1518-03-25 00:40] falls asleep
[1518-06-11 00:02] Guard #1051 begins shift
[1518-05-16 00:22] wakes up
[1518-08-15 00:47] falls asleep
[1518-05-18 00:15] falls asleep
[1518-09-11 00:18] wakes up
[1518-04-30 00:43] falls asleep
[1518-08-28 00:01] Guard #433 begins shift
[1518-06-19 00:00] Guard #103 begins shift
[1518-10-08 23:57] Guard #103 begins shift
[1518-04-19 00:19] falls asleep
[1518-11-23 00:12] falls asleep
[1518-08-15 00:55] wakes up
[1518-10-22 00:25] falls asleep
[1518-03-26 00:00] Guard #1871 begins shift
[1518-07-30 00:40] falls asleep
[1518-02-23 00:04] Guard #2251 begins shift
[1518-03-02 00:54] wakes up
[1518-02-21 00:45] wakes up
[1518-08-24 00:03] Guard #277 begins shift
[1518-06-26 00:15] falls asleep
[1518-02-17 00:00] Guard #677 begins shift
[1518-04-12 00:50] wakes up
[1518-03-05 23:56] Guard #2251 begins shift
[1518-03-09 00:57] wakes up
[1518-07-23 23:56] Guard #727 begins shift
[1518-06-08 00:10] falls asleep
[1518-09-02 00:56] falls asleep
[1518-04-23 00:19] falls asleep
[1518-08-10 00:01] Guard #2423 begins shift
[1518-08-12 00:40] falls asleep
[1518-09-18 00:20] falls asleep
[1518-05-20 00:31] falls asleep
[1518-02-20 23:56] Guard #1097 begins shift
[1518-04-19 00:48] wakes up
[1518-05-25 23:57] Guard #109 begins shift
[1518-03-08 00:51] wakes up
[1518-04-01 00:01] Guard #433 begins shift
[1518-08-08 00:19] wakes up
[1518-09-13 00:39] falls asleep
[1518-03-14 00:31] wakes up
[1518-02-28 00:25] falls asleep
[1518-03-10 00:54] wakes up
[1518-08-26 00:07] falls asleep
[1518-06-24 00:37] wakes up
[1518-04-23 23:46] Guard #109 begins shift
[1518-08-16 00:58] wakes up
[1518-06-02 00:25] falls asleep
[1518-03-05 00:34] falls asleep
[1518-02-22 00:58] wakes up
[1518-09-17 00:32] falls asleep
[1518-09-15 00:05] falls asleep
[1518-07-29 00:55] wakes up
[1518-03-12 00:27] wakes up
[1518-10-15 00:17] falls asleep
[1518-06-10 00:39] wakes up
[1518-06-27 23:47] Guard #277 begins shift
[1518-06-13 00:35] falls asleep
[1518-05-12 00:04] Guard #2593 begins shift
[1518-05-20 00:52] falls asleep
[1518-11-15 00:22] wakes up
[1518-05-13 00:04] Guard #677 begins shift
[1518-10-07 00:54] falls asleep
[1518-07-17 00:54] wakes up
[1518-06-28 00:02] falls asleep
[1518-11-19 23:58] Guard #821 begins shift
[1518-08-11 00:18] falls asleep
[1518-04-17 23:58] Guard #2593 begins shift
[1518-10-15 00:58] wakes up
[1518-11-02 00:47] falls asleep
[1518-11-18 00:47] wakes up
[1518-06-10 00:04] Guard #103 begins shift
[1518-07-01 00:58] wakes up
[1518-06-07 00:47] wakes up
[1518-10-29 00:33] wakes up
[1518-10-30 00:13] falls asleep
[1518-03-26 00:16] falls asleep
[1518-09-29 00:03] Guard #103 begins shift
[1518-08-02 00:51] wakes up
[1518-11-14 00:55] falls asleep
[1518-03-19 00:30] falls asleep
[1518-06-08 00:50] falls asleep
[1518-11-02 00:52] wakes up
[1518-08-27 00:39] wakes up
[1518-05-27 00:53] wakes up
[1518-09-19 00:02] Guard #2593 begins shift
[1518-10-07 23:56] Guard #1051 begins shift
[1518-06-11 00:06] falls asleep
[1518-04-14 00:54] wakes up
[1518-08-13 00:34] falls asleep
[1518-03-07 00:00] Guard #103 begins shift
[1518-11-05 00:50] wakes up
[1518-11-06 00:02] falls asleep
[1518-07-01 00:02] Guard #109 begins shift
[1518-05-22 00:00] Guard #2689 begins shift
[1518-08-16 23:57] Guard #1249 begins shift
[1518-09-17 00:37] wakes up
[1518-03-31 00:00] Guard #1871 begins shift
[1518-02-21 23:57] Guard #677 begins shift
[1518-04-15 00:03] Guard #1447 begins shift
[1518-04-22 00:00] Guard #3121 begins shift
[1518-09-12 00:03] Guard #433 begins shift
[1518-06-19 00:43] wakes up
[1518-07-18 00:00] Guard #1709 begins shift
[1518-11-10 00:26] falls asleep
[1518-04-01 00:30] wakes up
[1518-06-17 00:57] wakes up
[1518-06-23 00:40] falls asleep
[1518-03-11 00:00] Guard #739 begins shift
[1518-04-16 00:47] wakes up
[1518-11-17 00:24] falls asleep
[1518-06-17 23:54] Guard #277 begins shift
[1518-05-10 23:59] Guard #1447 begins shift
[1518-07-22 00:52] wakes up
[1518-04-10 00:03] falls asleep
[1518-08-26 00:50] wakes up
[1518-05-26 00:20] wakes up
[1518-05-14 23:57] Guard #2423 begins shift
[1518-05-22 00:40] wakes up
[1518-05-10 00:26] falls asleep
[1518-08-24 00:56] wakes up
[1518-06-04 00:58] wakes up
[1518-07-16 00:58] wakes up
[1518-05-12 00:48] wakes up
[1518-03-22 00:46] wakes up
[1518-03-20 00:00] Guard #739 begins shift
[1518-04-17 00:24] falls asleep
[1518-02-18 00:06] falls asleep
[1518-09-11 00:05] falls asleep
[1518-08-17 00:46] falls asleep
[1518-10-27 00:04] Guard #1051 begins shift
[1518-03-18 00:41] wakes up
[1518-09-17 00:19] wakes up
[1518-08-17 00:53] wakes up
[1518-06-14 00:17] falls asleep
[1518-11-10 00:51] falls asleep
[1518-03-29 00:44] falls asleep
[1518-08-08 00:16] falls asleep
[1518-05-05 00:55] wakes up
[1518-11-15 00:38] wakes up
[1518-03-07 00:12] falls asleep
[1518-03-31 00:48] falls asleep
[1518-06-24 00:55] wakes up
[1518-10-11 23:57] Guard #1447 begins shift
[1518-07-27 00:20] falls asleep
[1518-09-30 00:48] falls asleep
[1518-04-08 00:03] Guard #109 begins shift
[1518-11-16 00:51] wakes up
[1518-05-23 00:15] falls asleep
[1518-08-16 00:57] falls asleep
[1518-11-16 00:20] falls asleep
[1518-06-24 00:03] Guard #109 begins shift
[1518-02-16 00:50] falls asleep
[1518-07-27 00:35] wakes up
[1518-08-16 00:04] Guard #1097 begins shift
[1518-10-26 00:59] wakes up
[1518-09-20 00:03] Guard #109 begins shift
[1518-10-07 00:01] Guard #2251 begins shift
[1518-03-05 00:51] wakes up
[1518-08-30 00:45] wakes up
[1518-08-11 00:49] wakes up
[1518-10-03 00:58] wakes up
[1518-06-12 00:46] falls asleep
[1518-03-30 00:44] falls asleep
[1518-09-01 23:59] Guard #109 begins shift
[1518-03-11 00:31] falls asleep
[1518-04-12 00:03] Guard #677 begins shift
[1518-08-13 00:59] wakes up
[1518-11-23 00:30] wakes up
[1518-02-19 00:21] falls asleep
[1518-08-07 00:53] wakes up
[1518-04-13 00:54] wakes up
[1518-07-04 00:02] Guard #109 begins shift
[1518-04-28 00:57] wakes up
[1518-04-27 00:52] falls asleep
[1518-04-03 00:48] wakes up";
