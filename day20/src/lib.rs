use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
enum Role {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
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
    //fn build_modules_map(text: &str, conj_srcs: HashMap<&str, Vec<&str>>) -> HashMap<&str, Module> {
    let mut modules: HashMap<&str, Module> = HashMap::new(); // object label, object

    for line in text.lines() {
        let (mut src, dests) = line.split_once(" -> ").unwrap();
        let dests = dests.split(", ").collect::<Vec<&str>>();

        if src == "broadcaster" {
            modules.insert(
                src,
                Module {
                    name: src,
                    role: Role::Broadcast,
                    srcs: vec![],
                    dests: dests,
                    is_prev_sent_pulse_high: false,
                    is_turned_on: true,
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
                        name: src,
                        role: Role::Conjunction,
                        srcs: conj_srcs.get(src).unwrap().clone(),
                        dests: dests,
                        //prev_sigs: vec!['l'; conj_srcs.get(src).unwrap().len()],
                        is_prev_sent_pulse_high: false,
                        is_turned_on: true,
                    },
                );
            } else {
                modules.insert(
                    src,
                    Module {
                        name: src,
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

        let dests = dests.split(' ');
        for dest in dests {
            if conj_srcs.contains_key(dest) {
                conj_srcs.get_mut(dest).map(|srcs| srcs.push(src));
            }
        }
    }
    return conj_srcs;
}

pub fn day20_1(text: &str) -> u32 {
    let conj_srcs = find_conjunction_srcs(text);
    let mut modules = build_modules_map(text, conj_srcs);

    for key in modules.keys() {
        println!("{:?}", modules[key]);
    }

    // SRC sends PULSE-TYPE to DEST
    let mut q: VecDeque<(&str, &str, &str)> = VecDeque::new();

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    for _ in 0..1000 {
        q.push_back(("button", "low", "broadcaster"));

        while q.len() > 0 {
            let transmission = q.pop_front().unwrap();
            println!("{:?}", transmission);
            let src = transmission.0;
            let pulse_type = transmission.1;
            let dest = transmission.2;

            if pulse_type == "low" {
                low_pulse_count += 1;
            } else if pulse_type == "high" {
                high_pulse_count += 1;
            } else {
                panic!();
            }

            // TODO: finish pulse receiving and sending
            if !modules.contains_key(dest) {
                continue;
            }
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
                        q.push_back((dest, pulse_type, new_dest))
                    }
                }
                Role::Conjunction => {
                    // update connected_inputs with the new pulse type
                    let mut pulse_to_emit = "low";
                    for src in &modules[dest].srcs {
                        if !modules[src].is_prev_sent_pulse_high {
                            pulse_to_emit = "high";
                        }
                    }

                    for new_dest in &modules[dest].dests {
                        q.push_back((dest, pulse_to_emit, new_dest))
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
                                q.push_back((dest, "high", new_dest))
                            }
                        } else {
                            // emit low after turning off
                            modules
                                .get_mut(dest)
                                .map(|v| v.is_prev_sent_pulse_high = false);
                            for new_dest in &modules[dest].dests {
                                q.push_back((dest, "low", new_dest))
                            }
                        }
                    }
                }
                _ => {
                    panic!();
                }
            }
        }
    }

    println!("complete run\n");
    return low_pulse_count * high_pulse_count;
}
