use util::Loader;

type Input = Vec<i32>;
type Output = i32;

fn main() -> anyhow::Result<()> {
    let input = Loader::new("example.txt")?.numbers::<i32>()?;
    // let input = Loader::new("input.txt")?.numbers::<i32>()?;

    println!("Part 1: {}", solve_one(&input));
    // println!("Part 2: {}", solve_two(&input));

    Ok(())
}

fn solve_one(input: &Input) -> Output {
    todo!()
}

fn solve_two(input: &Input) -> Output {
    todo!()
}
