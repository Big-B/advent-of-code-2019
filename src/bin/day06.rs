use advent_of_code_2019::*;
use std::collections::HashMap;

fn main() {
    let input = get_input();
    let mut map: HashMap<&str, (&str, Option<usize>)> = HashMap::new();

    for s in input
        .split_whitespace()
        .map(|s| s.split(')').collect::<Vec<&str>>())
    {
        // Insert the thing it's orbiting around and the initial count
        map.insert(s[1], (s[0], None));
    }

    println!("Num Orbits: {}", get_number_of_orbits(&mut map));
    println!(
        "Orbital Transfers: {}",
        get_number_of_transfers("YOU", "SAN", &map)
    );
}

fn get_number_of_transfers(
    start: &str,
    end: &str,
    map: &HashMap<&str, (&str, Option<usize>)>,
) -> usize {

    let mut start = start;
    let mut end = end;
    let mut distance = 0;

    // These two while loops will make sure we start at the same distance from the root
    while map.get(start).unwrap().1.unwrap() > map.get(end).unwrap().1.unwrap() {
        distance += 1;
        start = map.get(start).unwrap().0;
    }

    while map.get(end).unwrap().1.unwrap() > map.get(start).unwrap().1.unwrap() {
        distance += 1;
        end = map.get(end).unwrap().0;
    }

    // Now that we know we're the same distance away from the root, we can walk down
    // the orbits on each branch together. Once we get to the common node, we've
    // found a path.
    while map.get(end).unwrap().0 != map.get(start).unwrap().0 {
        distance += 2;
        start = map.get(start).unwrap().0;
        end = map.get(end).unwrap().0;
    }

    distance
}

fn get_all_planets<'a>(map: &mut HashMap<&'a str, (&'a str, Option<usize>)>) -> Vec<&'a str> {
    let mut planet_values: Vec<&str> = map.values().map(|(s, _)| *s).collect();
    let mut keys: Vec<&str> = map.keys().map(|&x| x).collect();
    planet_values.append(&mut keys);
    planet_values.dedup();
    planet_values
}

fn get_number_of_orbits<'a>(map: &mut HashMap<&'a str, (&'a str, Option<usize>)>) -> usize {
    let mut stack: Vec<&str> = Vec::new();
    let planets = get_all_planets(map);

    // For every possible value, we need to check to see if it's orbiting something.
    //
    // If it is orbiting something, then the value of its orbit is the value of the
    // orbit of the thing it is orbiting plus 1. This means we need to search down until we
    // find the bottom, and then start counting back up.
    //
    // If it is not orbiting something, then we're at the bottom, which is 0.
    for planet in planets {
        let mut curr = planet;

        // Push onto the stack until we hit either a planet that already has an orbit value
        // or the bottom.
        while let Some((val, None)) = map.get(curr) {
            stack.push(curr);
            curr = *val;
        }

        // Figure out if we hit the bottom, or an entry that already has the distance
        // calculated and use that value to start counting from
        let mut distance = if let Some((_, Some(dis))) = map.get(curr) {
            *dis
        } else {
            0
        };

        // We have a bunch of things on the stack, potentially, to add distances
        // to.
        while let Some(curr) = stack.pop() {
            distance += 1;
            // Everything in the vector came from the map, so we can unwrap.
            map.get_mut(curr).unwrap().1 = Some(distance);
        }
    }

    // Calculate the sum of all the orbit values
    map.values().map(|x| x.1.unwrap_or(0)).sum()
}
