fn parse_id(input: &str) -> (u32, &str) {
    let split: Vec<&str> = input.split(": ").collect();
    match split[..] {
        [id, game] => (id[5..].parse().unwrap_or(0), game),
        _ => panic!("could not split ': ' from input {}",input)
    }
}

fn parse_colours(input: &str) -> Vec<&str> {
    let sets = input.split("; ");
    let colours_revealed = sets.map(|s| s.split(", ")).flatten();
    colours_revealed.collect()
}


fn collect_colour(colour_list: &Vec<&str>, colour: &str) -> u32 {
    colour_list
        .into_iter()
        .filter(|s| s.contains(colour))
        .map(|s| s[..s.len()-colour.len()-1].parse().unwrap_or(0))
        .max()
        .unwrap_or(0)
}

fn parse_game(input: &str) -> (u32, Colours) {
    let (id, input) = parse_id(input);
    let colour_strings = parse_colours(input);
    let red = collect_colour(&colour_strings, "red");
    let green = collect_colour(&colour_strings, "green");
    let blue = collect_colour(&colour_strings, "blue");
    let colours = Colours::new(red, green, blue);
    (id, colours)
}

struct Colours {
    red: u32,
    green: u32,
    blue: u32,
}

impl Colours {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }

    fn exceeds(self: Self, maxes: &Self) -> bool {
        if self.red > maxes.red ||
           self.blue > maxes.blue ||
           self.green > maxes.green {
                true
        }
        else { false }
    }
}

fn main() {
    let maxes = Colours::new(12,13,14);
    let mut total: u32 = 0;
    let input = std::fs::read_to_string("./input.txt").expect("could not read input.txt in current directory");
    for game in input.lines() {
        let (id, colours) = parse_game(game);
        if !colours.exceeds(&maxes) {total += id}
    }
    println!("{}", total)
}
