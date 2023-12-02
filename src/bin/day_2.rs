use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("../../input/day_2.txt").trim();

    println!("part 1: {}", aoc::day2::find_possible_games(input));
    println!(
        "part 2: {}",
        aoc::day2::sum_of_powers_of_fewest_cubes(input)
    );

    Ok(())
}
