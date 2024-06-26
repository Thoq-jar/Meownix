#[derive(Copy, Clone)]
pub struct LineNumbers {
    pub start: usize,

    pub total: usize,

    pub current: usize,

    pub relative: bool,
}

impl LineNumbers {
    pub fn new(start: usize, total: usize, current: usize, relative: bool) -> Self {
        Self {
            start,
            total,
            current,
            relative,
        }
    }

    pub fn width(self, height: usize) -> usize {
        self.width_number(height) + 2
    }

    pub fn width_number(self, height: usize) -> usize {
        if self.relative {
            let max_diff = height;

            let max = max_diff.max(self.total);

            (max as f64).log10() as usize + 1
        } else {
            (self.total as f64).log10() as usize + 1
        }
    }
}

pub struct TextLine<'a> {
	pub string: &'a str,
}

impl<'a> TextLine<'a> {
    pub fn new(string: &'a str) -> Self {
        Self { string }
    }
}