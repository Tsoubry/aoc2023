const ARRAY_NUMBER_SIZE: usize = 19600; // 140^2, more than large enough

#[derive(Copy, Clone, Debug)]
pub enum Item {
    Symbol,
    Gear,
    Number(u16),
    Nothing,
}

#[derive(Debug)]
pub struct Grid<const X: usize, const Y: usize> {
    pub grid: [[Item; X]; Y],
}

impl<const X: usize, const Y: usize> Grid<X, Y> {
    pub fn new() -> Self {
        Grid {
            grid: [[Item::Nothing; X]; Y],
        }
    }
}
