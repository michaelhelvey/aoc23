use color_eyre::Result;

pub mod day1;

pub fn read_input_for_day(day: u8) -> Result<String> {
    let input = std::fs::read_to_string(format!("input/day_{}.txt", day))?;
    Ok(input)
}
