#[derive(Debug, Clone)]
pub(super) enum Step<'i> {
    GoTo(&'i str),
    Open,
}

impl std::fmt::Display for Step<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::GoTo(s) => write!(f, "->{}", s),
            Step::Open => write!(f, "OPEN"),
        }
    }
}
