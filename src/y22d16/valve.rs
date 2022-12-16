#[derive(Debug)]
pub struct Valve {
    pub rate: u32,
    pub paths: Vec<usize>,
    pub name: usize,
}
