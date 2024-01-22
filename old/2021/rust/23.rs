fn main() {
    let inp = std::fs::read_to_string("../input/23").unwrap().trim().to_string();
    let map = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();


}
