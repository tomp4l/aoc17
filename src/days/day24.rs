use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let components = lines
            .iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<Component>, _>>()?;

        let (part1, part2) = strongest_bridge(components);

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Component {
    port_a: u8,
    port_b: u8,
}

impl FromStr for Component {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/');
        let port_a = parts
            .next()
            .ok_or("missing port a")?
            .parse()
            .map_err(|e| format!("invalid port a: {}", e))?;
        let port_b = parts
            .next()
            .ok_or("missing port b")?
            .parse()
            .map_err(|e| format!("invalid port b: {}", e))?;

        Ok(Component { port_a, port_b })
    }
}

fn strongest_bridge(components: Vec<Component>) -> (u32, u32) {
    let mut component_map = HashMap::new();

    for component in &components {
        component_map
            .entry(component.port_a)
            .or_insert(vec![])
            .push(component);
        component_map
            .entry(component.port_b)
            .or_insert(vec![])
            .push(component);
    }

    struct State<'a> {
        components: HashSet<&'a Component>,
        port: u8,
        strength: u32,
    }

    fn build_bridge(
        component_map: &HashMap<u8, Vec<&Component>>,
        state: &State,
        strongest: &mut u32,
        longest: &mut (u32, u32),
    ) {
        let mut found = false;

        for component in component_map.get(&state.port).unwrap_or(&vec![]) {
            if state.components.contains(component) {
                continue;
            }

            let next_port = if component.port_a == state.port {
                component.port_b
            } else {
                component.port_a
            };

            let mut next_state = State {
                components: state.components.clone(),
                port: next_port,
                strength: state.strength + component.port_a as u32 + component.port_b as u32,
            };

            next_state.components.insert(component);

            build_bridge(component_map, &next_state, strongest, longest);

            found = true;
        }

        if !found {
            *strongest = std::cmp::max(*strongest, state.strength);
            if state.components.len() > longest.1 as usize {
                *longest = (state.strength, state.components.len() as u32);
            } else if state.components.len() == longest.1 as usize {
                longest.0 = std::cmp::max(longest.0, state.strength);
            }
        }
    }

    let mut strongest = 0;
    let mut longest = (0, 0);
    build_bridge(
        &component_map,
        &State {
            components: HashSet::new(),
            port: 0,
            strength: 0,
        },
        &mut strongest,
        &mut longest,
    );
    (strongest, longest.0)
}
