use super::figure::Figure;

pub fn get_figures() -> Vec<Figure> {
    let mut res: Vec<Figure> = Vec::new();

    // ####
    res.push(vec![(0, 0), (0, 1), (0, 2), (0, 3)].into());

    // .#.
    // ###
    // .#.
    res.push(vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)].into());

    // ..#
    // ..#
    // ###
    res.push(vec![(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)].into());

    // #
    // #
    // #
    // #
    res.push(vec![(0, 0), (1, 0), (2, 0), (3, 0)].into());

    // ##
    // ##
    res.push(vec![(0, 0), (0, 1), (1, 0), (1, 1)].into());

    res
}
