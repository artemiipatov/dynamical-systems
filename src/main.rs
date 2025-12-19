mod problem1;
mod problem2;
mod problem3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    problem1::solve().unwrap();
    problem2::solve().unwrap();
    problem3::solve()
}
