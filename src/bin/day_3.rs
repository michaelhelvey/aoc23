use color_eyre::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/day_3.txt").trim();
    let grid = aoc::day3::Grid::from_input(input);

    println!("part 1 sum: {}", grid.sum_numbers_with_adjencent_symbols());
    println!("part 2 sum: {}", grid.sum_of_gear_ratios());
    Ok(())
}
