use ci_sandbox_rust::checked_add;

fn main() -> anyhow::Result<()> {
    let sum = checked_add(20, 22)?;
    println!("the answer is {sum}");
    Ok(())
}
