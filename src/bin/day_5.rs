fn main() {
    let input = include_str!("../../input/day_5.txt");

    println!("part 1: {}", aoc::day5::find_lowest_location_number(input));
    println!(
        "part 2: {}",
        aoc::day5::find_lowest_location_for_seed_ranges(input)
    );
}
