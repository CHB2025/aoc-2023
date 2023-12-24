use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    println!("Part one: {}", part_one(input)?);
    println!("Part two: {}", part_two(input)?);

    Ok(())
}

fn part_one(input: &str) -> Result<usize> {
    _ = input;
    Ok(0)
}

fn part_two(input: &str) -> Result<usize> {
    _ = input;
    Ok(0)
}
