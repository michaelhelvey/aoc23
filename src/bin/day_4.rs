use color_eyre::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/day_4.txt").trim();
    let sum = aoc::day4::sum_winning_scores(input);
    let card_count = aoc::day4::sum_recursive_won_scratchcards(input);

    println!("sum of part 1: {}", sum);
    println!("sum of part 2: {}", card_count);
    Ok(())
}
