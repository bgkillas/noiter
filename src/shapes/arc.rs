use crate::line::LineIter;
#[derive(Debug, Clone)]
pub struct ArcIter {
    low_line: LineIter,
    high_line: LineIter,
    range_x: u16,
    range_y_start: u16,
    range_y_end: u16,
    x0: u16,
    y0: u16,
    hx: u16,
    r2: u16,
    steep: bool,
}
impl Iterator for ArcIter {
    type Item = (u16, u16);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.range_y_end >= self.range_y_start {
            Some(self.next_range())
        } else if let Some((_, _, hy)) = self.high_line.next() {
            let (_, lx, ly) = self.low_line.next().unwrap();
            self.range_x = lx;
            if self.high_line.dy_neg {
                self.range_y_end = ly;
                self.range_y_start = hy;
            } else {
                self.range_y_start = ly;
                self.range_y_end = hy;
            }
            Some(self.next_range())
        } else if let Some((_, lx, ly)) = self.low_line.next() {
            if self.low_line.dx_neg {
                self.hx -= 1;
            } else {
                self.hx += 1;
            }
            let yy = self.r2 - self.hx.abs_diff(self.x0).pow(2);
            self.range_x = lx;
            if self.high_line.dy_neg {
                self.range_y_end = ly;
                self.range_y_start = self.y0 - yy.isqrt();
            } else {
                self.range_y_start = ly;
                self.range_y_end = self.y0 + yy.isqrt();
            }
            Some(self.next_range())
        } else {
            None
        }
    }
}
impl ArcIter {
    #[inline]
    #[must_use]
    pub fn new(
        mut x0: u16,
        mut y0: u16,
        mut x1: u16,
        mut y1: u16,
        mut x2: u16,
        mut y2: u16,
        r2: u16,
    ) -> Self {
        let steep = y2.abs_diff(y0) > x2.abs_diff(x0) || y1.abs_diff(y0) > x1.abs_diff(x0);
        if steep {
            (x0, y0) = (y0, x0);
            (x1, y1, x2, y2) = (y2, x2, y1, x1);
        }
        if (x1 < x0 || x2 < x0) ^ (y1 < y0 || y2 < y0) {
            (x1, y1, x2, y2) = (x2, y2, x1, y1);
        }
        Self {
            low_line: LineIter::new(x0, y0, x1, y1),
            high_line: LineIter::new(x0, y0, x2, y2),
            range_x: 0,
            range_y_start: 1,
            range_y_end: 0,
            x0,
            y0,
            hx: x2,
            r2,
            steep,
        }
    }
    fn next_range(&mut self) -> (u16, u16) {
        let y = self.range_y_start;
        self.range_y_start += 1;
        let x = self.range_x;
        if self.steep { (y, x) } else { (x, y) }
    }
}
