use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
enum Role {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug, Clone)]
struct Module<'a> {
    role: Role,
    srcs: Vec<&'a str>,
    dests: Vec<&'a str>,
    is_prev_sent_pulse_high: bool,
    is_turned_on: bool,
}

fn build_modules_map<'a>(
    text: &'a str,
    conj_srcs: HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, Module<'a>> {
    let mut modules: HashMap<&str, Module> = HashMap::new(); // object label, object

    for line in text.lines() {
        let (mut src, dests) = line.split_once(" -> ").unwrap();
        let dests = dests.split(", ").collect::<Vec<&str>>();

        if src == "broadcaster" {
            modules.insert(
                src,
                Module {
                    role: Role::Broadcast,
                    srcs: vec![],
                    dests: dests,
                    is_prev_sent_pulse_high: false,
                    is_turned_on: false,
                },
            );
        } else {
            let orig_src = src;
            src = src.trim_start_matches('%');
            if src == orig_src {
                src = src.trim_start_matches('&');
                modules.insert(
                    src,
                    Module {
                        role: Role::Conjunction,
                        srcs: conj_srcs.get(src).unwrap().clone(),
                        dests: dests,
                        is_prev_sent_pulse_high: false,
                        is_turned_on: false,
                    },
                );
            } else {
                modules.insert(
                    src,
                    Module {
                        role: Role::FlipFlop,
                        srcs: vec![],
                        dests: dests,
                        is_prev_sent_pulse_high: false,
                        is_turned_on: false,
                    },
                );
            }
        }
    }
    return modules;
}

fn find_conjunction_srcs(text: &str) -> HashMap<&str, Vec<&str>> {
    let mut conj_srcs: HashMap<&str, Vec<&str>> = HashMap::new();
    // Find the names of all conjunction modules
    for line in text.lines() {
        if !line.starts_with('&') {
            continue;
        }
        let (mut name, _) = line.split_once(' ').unwrap();
        name = name.trim_start_matches('&');
        conj_srcs.insert(name, vec![]);
    }
    // Add modules that send signals to conj. modules to their respective vector
    for line in text.lines() {
        let (mut src, dests) = line.split_once(" -> ").unwrap();

        src = src.trim_start_matches('%');
        src = src.trim_start_matches('&');

        let dests = dests.split(", ");
        for dest in dests {
            if conj_srcs.contains_key(dest) {
                conj_srcs.get_mut(dest).map(|srcs| srcs.push(src));
            }
        }
    }
    return conj_srcs;
}

pub fn day20_1(text: &str) -> u32 {
    // Return the product of total high pulses sent and total low pulses sent
    // if the button is pressed 1000 times
    let conj_srcs = find_conjunction_srcs(text);
    let mut modules = build_modules_map(text, conj_srcs);

    // In a queue tuple, SRC sends PULSE-TYPE to DEST
    let mut q: VecDeque<(&str, &str, &str)> = VecDeque::new();

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    for _ in 0..1000 {
        q.push_back(("button", "low", "broadcaster"));

        while q.len() > 0 {
            let transmission = q.pop_front().unwrap();

            let pulse_type = transmission.1;
            if pulse_type == "low" {
                low_pulse_count += 1;
            } else if pulse_type == "high" {
                high_pulse_count += 1;
            } else {
                panic!();
            }

            let dest = transmission.2;
            if !modules.contains_key(dest) {
                continue;
            }

            propogate(transmission, &mut q, &mut modules);
        }
    }

    return low_pulse_count * high_pulse_count;
}

fn propogate<'a>(
    transmission: (&str, &'a str, &'a str),
    q: &mut VecDeque<(&'a str, &'a str, &'a str)>,
    modules: &mut HashMap<&str, Module<'a>>,
) {
    let pulse_type = transmission.1;
    let dest = transmission.2;

    match modules[dest].role {
        Role::Broadcast => {
            if pulse_type == "high" {
                modules
                    .get_mut(dest)
                    .map(|v| v.is_prev_sent_pulse_high = true);
            } else if pulse_type == "low" {
                modules
                    .get_mut(dest)
                    .map(|v| v.is_prev_sent_pulse_high = false);
            } else {
                panic!();
            }

            for new_dest in &modules[dest].dests {
                q.push_back((dest.clone(), pulse_type.clone(), new_dest.clone()))
            }
        }
        Role::Conjunction => {
            let mut pulse_to_emit = "low";

            for src in &modules[dest].srcs {
                if !modules[src].is_prev_sent_pulse_high {
                    pulse_to_emit = "high";
                }
            }

            if pulse_to_emit == "high" {
                modules
                    .get_mut(dest)
                    .map(|v| v.is_prev_sent_pulse_high = true);
            } else if pulse_to_emit == "low" {
                modules
                    .get_mut(dest)
                    .map(|v| v.is_prev_sent_pulse_high = false);
            } else {
                panic!();
            }

            for new_dest in &modules[dest].dests {
                q.push_back((dest.clone(), pulse_to_emit, new_dest.clone()))
            }
        }
        Role::FlipFlop => {
            if pulse_type == "low" {
                modules.get_mut(dest).map(|v| v.is_turned_on ^= true);
                if modules[dest].is_turned_on {
                    modules
                        .get_mut(dest)
                        .map(|v| v.is_prev_sent_pulse_high = true);
                    // emit high after turning on
                    for new_dest in &modules[dest].dests {
                        q.push_back((dest.clone(), "high", new_dest.clone()))
                    }
                } else {
                    // emit low after turning off
                    modules
                        .get_mut(dest)
                        .map(|v| v.is_prev_sent_pulse_high = false);
                    for new_dest in &modules[dest].dests {
                        q.push_back((dest.clone(), "low", new_dest.clone()))
                    }
                }
            }
        }
    }
}

pub fn day20_2(text: &str) -> u64 {
    // fewest number of button presses required to deliver a single low pulse to rx?
    let conj_srcs = find_conjunction_srcs(text);
    let mut modules = build_modules_map(text, conj_srcs);

    let mut q: VecDeque<(&str, &str, &str)> = VecDeque::new();
    let mut button_press_count = 0;

    // conj. hp is only sender to rx
    // hp receives from &sr, &sn, &rf, &vq
    // rx can only receive a low pulse when all hp_srcs have sent a high pulse
    let hp_srcs = &modules["hp"].srcs.clone();
    let mut hp_srcs_button_counts = [0; 4];
    let mut saved_button_counts = [0; 4];

    while button_press_count < 1_000_000 {
        q.push_back(("button", "low", "broadcaster"));
        button_press_count += 1;

        while q.len() > 0 {
            let transmission = q.pop_front().unwrap();
            let pulse_type = transmission.1;
            let dest = transmission.2;

            let src = transmission.0;
            if pulse_type == "high" && dest == "hp" {
                for idx in 0..hp_srcs.len() {
                    if hp_srcs[idx] == src {
                        hp_srcs_button_counts[idx] += 1;
                        break;
                    }
                }
            }
            if !modules.contains_key(dest) {
                continue;
            }
            propogate(transmission, &mut q, &mut modules);
        }
        // check if each of hp_srcs have sent >0 high pulses && respective button_count == 0
        // if so, store button press count in respsective hp_srcs_button_counts idx
        for idx in 0..hp_srcs_button_counts.len() {
            if hp_srcs_button_counts[idx] > 0 && saved_button_counts[idx] == 0 {
                saved_button_counts[idx] = button_press_count;
            }
        }

        let mut found_zero = false;
        for count in hp_srcs_button_counts {
            if count == 0 {
                found_zero = true;
            }
        }
        if !found_zero {
            break;
        }
    }

    return saved_button_counts.iter().product::<u64>();
}
