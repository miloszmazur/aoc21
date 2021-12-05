mod aoc1;
mod aoc2;

fn main() -> std::io::Result<()> {
    aoc1::part1().ok();
    aoc1::part2().ok();
    aoc2::part1().ok();
    aoc2::part2().ok();
    Ok(())
}
