use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    fmt::{Debug, Display},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Pulse {
    High,
    Low,
}

impl Pulse {
    /// Inverts the pulse and returns it.
    /// High -> Low
    /// Low -> High
    pub fn inv(self) -> Self {
        match self {
            Self::High => Self::Low,
            Self::Low => Self::High,
        }
    }
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::High => write!(f, "high"),
            Self::Low => write!(f, "low"),
        }
    }
}

impl From<bool> for Pulse {
    fn from(value: bool) -> Self {
        match value {
            true => Self::High,
            false => Self::Low,
        }
    }
}

impl From<Pulse> for bool {
    fn from(value: Pulse) -> Self {
        match value {
            Pulse::High => true,
            Pulse::Low => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Module<'a> {
    /// Logic D Flip-Flop.
    FlipFlop { state: Pulse, outputs: Vec<&'a str> },
    /// Logic NAND-gate.
    Conjunction {
        inputs: BTreeMap<&'a str, Pulse>,
        outputs: Vec<&'a str>,
    },
    /// Logic OR-gate.
    Broadcaster { outputs: Vec<&'a str> },
}

impl<'a> Module<'a> {
    pub fn get_outputs(&self) -> &[&'a str] {
        match self {
            Module::FlipFlop { outputs, .. } => outputs.as_slice(),
            Module::Conjunction { outputs, .. } => outputs.as_slice(),
            Module::Broadcaster { outputs } => outputs.as_slice(),
        }
    }

    pub fn pulse(
        &mut self,
        queue: &mut VecDeque<(&'a str, &'a str, Pulse)>,
        src: &'a str,
        dst: &'a str,
        pulse: Pulse,
    ) {
        match self {
            Module::Broadcaster { outputs } => {
                for output in outputs {
                    System::send_pulse(queue, dst, output, pulse);
                }
            }
            Module::FlipFlop { state, outputs } => {
                if pulse == Pulse::High {
                    return; // Ignore high pulses.
                }

                *state = state.inv();
                for output in outputs {
                    System::send_pulse(queue, dst, output, *state);
                }
            }
            Module::Conjunction { inputs, outputs } => {
                inputs
                    .entry(src)
                    .and_modify(|p| *p = pulse)
                    .or_insert_with(|| unreachable!("input 'src' should be predefined"));

                for output in outputs {
                    let and_pulse: Pulse = inputs.values().copied().all(From::from).into();
                    System::send_pulse(queue, dst, output, and_pulse.inv());
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct System<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    pulse_queue: VecDeque<(&'a str, &'a str, Pulse)>,
}

impl<'a> System<'a> {
    pub fn new(mut modules: HashMap<&'a str, Module<'a>>) -> Self {
        modules.shrink_to_fit();

        let mut system = System {
            modules: modules.clone(),
            pulse_queue: VecDeque::new(),
        };

        for (name, module) in system.modules.iter_mut() {
            match module {
                Module::FlipFlop { .. } | Module::Broadcaster { .. } => continue,
                Module::Conjunction { inputs, .. } => {
                    *inputs = modules
                        .iter()
                        .filter(|(_, m)| m.get_outputs().iter().any(|output| output == name))
                        .map(|(name, _)| (*name, Pulse::Low))
                        .collect()
                }
            }
        }

        system
    }

    fn send_pulse(
        queue: &mut VecDeque<(&'a str, &'a str, Pulse)>,
        src: &'a str,
        dst: &'a str,
        pulse: Pulse,
    ) {
        // println!("{} -{}-> {}", src, pulse, dst); // Debug
        queue.push_back((src, dst, pulse));
    }

    /// Sends a single button pulse and forwards the simulation until it stops.
    ///
    /// ## Return
    /// A tuple containing the number of low pulses and high pulses sent, in
    /// that order.
    pub fn button_pulse(&mut self) -> (usize, usize) {
        System::send_pulse(&mut self.pulse_queue, "button", "broadcaster", Pulse::Low);

        let mut high_count = 0;
        let mut low_count = 0;

        while let Some((src, dst, pulse)) = self.pulse_queue.pop_front() {
            match pulse {
                Pulse::High => high_count += 1,
                Pulse::Low => low_count += 1,
            }
            if let Some(module) = self.modules.get_mut(dst) {
                module.pulse(&mut self.pulse_queue, src, dst, pulse);
            }
        }

        (low_count, high_count)
    }
}
