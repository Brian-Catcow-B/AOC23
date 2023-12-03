fn main() {
    let input = String::from(include_str!("input.txt"));
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input));
}

struct CubeSet {
    pub numred: usize,
    pub numgreen: usize,
    pub numblue: usize,
}

enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err("invalid input string"),
        }
    }
}

type GameId = usize;

fn get_cube_sets(input: &String) -> Vec<(GameId, Vec<CubeSet>)> {
    let mut vec_cube_sets: Vec<(GameId, Vec<CubeSet>)> = Vec::new();
    let mut outer_index: usize = 0;
    let skip_first = "Game ".len();
    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }
        let mut line_index = 0;
        // skip over "Game "
        line_index += skip_first;
        let first_semicolon_index = line
            .get(line_index..)
            .expect("")
            .find(':')
            .expect("no ':' character after \"Game (num)\"")
            + line_index;
        let id: GameId = line
            .get(line_index..first_semicolon_index)
            .expect("")
            .parse::<usize>()
            .expect("couldn't get GameId number");
        vec_cube_sets.push((id, Vec::new()));
        line_index = first_semicolon_index + 2;

        'outer: loop {
            let mut color_set = CubeSet {
                numred: 0,
                numgreen: 0,
                numblue: 0,
            };
            loop {
                let separating_space = line
                    .get(line_index..)
                    .expect("")
                    .find(' ')
                    .expect("no ' ' character after number")
                    + line_index;
                let number = line
                    .get(line_index..separating_space)
                    .expect("")
                    .parse::<usize>()
                    .expect("");
                line_index = separating_space + 1;
                let separating_comma = match line.get(line_index..).expect("").find(',') {
                    Some(i) => i + line_index,
                    None => line.len(),
                };
                let separating_semicolon = match line.get(line_index..).expect("").find(';') {
                    Some(i) => i + line_index,
                    None => line.len(),
                };
                let next_separator = std::cmp::min(separating_comma, separating_semicolon);
                let color: Color =
                    Color::try_from(line.get(line_index..next_separator).expect("")).expect("");
                line_index = next_separator + 2;
                match color {
                    Color::Red => color_set.numred = number,
                    Color::Green => color_set.numgreen = number,
                    Color::Blue => color_set.numblue = number,
                }
                if separating_comma > separating_semicolon {
                    vec_cube_sets[outer_index].1.push(color_set);
                    break;
                } else if separating_comma == separating_semicolon {
                    vec_cube_sets[outer_index].1.push(color_set);
                    break 'outer;
                }
            }
        }

        outer_index += 1;
    }

    vec_cube_sets
}

fn solve1(input: &String) -> usize {
    let mut sum = 0;

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let cube_sets = get_cube_sets(input);

    for cube_set in cube_sets.iter() {
        let mut possible = true;
        for cube in cube_set.1.iter() {
            if cube.numred > max_red || cube.numgreen > max_green || cube.numblue > max_blue {
                possible = false;
                break;
            }
        }
        if possible {
            sum += cube_set.0;
        }
    }

    sum
}

fn solve2(input: &String) -> usize {
    let mut sum = 0;

    let cube_sets = get_cube_sets(input);

    for cube_set in cube_sets.iter() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for cube in cube_set.1.iter() {
            max_red = std::cmp::max(cube.numred, max_red);
            max_green = std::cmp::max(cube.numgreen, max_green);
            max_blue = std::cmp::max(cube.numblue, max_blue);
        }
        let power = max_red * max_green * max_blue;
        sum += power;
    }

    sum
}
