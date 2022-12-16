#[derive(Debug)]
pub struct Valve<'i> {
    pub rate: u32,
    pub paths: Vec<&'i str>,
    pub name: &'i str,
}
