use crate::circumference::Circumference;
use std::range::{Range, RangeIter};
pub struct Circle {
    x0: u16,
    y0: u16,
    octant: u8,
    dx: u16,
    dy: u16,
    y: u16,
    range: RangeIter<u16>,
    circumference: Circumference,
}
impl Iterator for Circle {
    type Item = (u16, u16);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.range.next() {
            Some((x, self.y))
        } else if self.octant < 3 {
            self.octant += 1;
            match self.octant {
                1 => {
                    self.range = Range::from(self.x0 - self.dx..self.x0 + self.dx).into_iter();
                    self.y = self.y0 - self.dy;
                    Some((self.x0 + self.dx, self.y))
                }
                2 => {
                    self.range = Range::from(self.x0 - self.dy..self.x0 + self.dy).into_iter();
                    self.y = self.y0 + self.dx;
                    Some((self.x0 + self.dy, self.y))
                }
                3 => {
                    self.range = Range::from(self.x0 - self.dy..self.x0 + self.dy).into_iter();
                    self.y = self.y0 - self.dx;
                    Some((self.x0 + self.dy, self.y))
                }
                _ => unreachable!(),
            }
        } else if let Some((dx, dy)) = self.circumference.next() {
            self.dx = dx;
            self.dy = dy;
            self.range = Range::from(self.x0 - dx..self.x0 + dx).into_iter();
            self.y = self.y0 + dy;
            self.octant = 0;
            Some((self.x0 + dx, self.y))
        } else {
            None
        }
    }
}
impl Circle {
    #[inline]
    #[must_use]
    pub fn new(x0: u16, y0: u16, r: u16) -> Self {
        Self {
            x0,
            y0,
            dx: 0,
            dy: 0,
            octant: u8::MAX,
            y: 0,
            #[allow(clippy::reversed_empty_ranges)]
            range: Range::from(1..0).into_iter(),
            circumference: Circumference::new(r),
        }
    }
}
