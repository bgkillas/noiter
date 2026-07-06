use crate::cell::CellVelocity;
use crate::world_data::FullIndex;
#[derive(Debug, Clone, Default)]
pub struct LineFullIter {
    pub line: LineIter,
}
impl Iterator for LineFullIter {
    type Item = (StepCase, FullIndex);
    fn next(&mut self) -> Option<Self::Item> {
        self.line
            .next()
            .map(|(case, x2, y2)| (case, (x2, y2).into()))
    }
}
impl LineFullIter {
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn back(&mut self, case: StepCase) {
        self.line.back(case);
    }
    #[inline]
    #[must_use]
    pub fn new(i0: FullIndex, i1: FullIndex) -> Self {
        let (x0, y0) = i0.to_u16();
        let (x1, y1) = i1.to_u16();
        Self {
            line: LineIter::new(x0, y0, x1, y1),
        }
    }
    #[inline]
    #[must_use]
    pub fn new_vel(i0: FullIndex, vel: CellVelocity) -> Self {
        let (x0, y0) = i0.to_u16();
        let x1 = if vel.x > 0 {
            x0 + vel.x.strict_cast::<u16>()
        } else {
            x0 - vel.x.unsigned_abs().strict_cast::<u16>()
        };
        let y1 = if vel.y > 0 {
            y0 + vel.y.strict_cast::<u16>()
        } else {
            y0 - vel.y.unsigned_abs().strict_cast::<u16>()
        };
        Self {
            line: LineIter::new(x0, y0, x1, y1),
        }
    }
}
#[derive(Debug, Clone)]
pub struct LineIter {
    pub x0: u16,
    pub y0: u16,
    pub x1: u16,
    pub y1: u16,
    pub dx_neg: bool,
    pub dy_neg: bool,
    pub dx_abs: i16,
    pub dy_abs: i16,
    pub error_test: i16,
    pub error: i16,
    pub first: bool,
}
impl Default for LineIter {
    #[inline]
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}
#[derive(Debug)]
#[repr(u8)]
pub enum StepCase {
    Start = 0,
    Dx = 1,
    Dy = 2,
    Both = 3,
}
impl Iterator for LineIter {
    type Item = (StepCase, u16, u16);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return Some((StepCase::Start, self.x0, self.y0));
        }
        match (!self.error.is_negative(), self.error <= self.error_test) {
            (true, true) => {
                if self.x0 == self.x1 || self.y0 == self.y1 {
                    return None;
                }
                self.error += self.dx_abs + self.dy_abs;
                self.sx();
                self.sy();
                Some((StepCase::Both, self.x0, self.y0))
            }
            (true, false) => {
                if self.x0 == self.x1 {
                    return None;
                }
                self.error += self.dy_abs;
                self.sx();
                Some((StepCase::Dx, self.x0, self.y0))
            }
            (false, true) => {
                if self.y0 == self.y1 {
                    return None;
                }
                self.error += self.dx_abs;
                self.sy();
                Some((StepCase::Dy, self.x0, self.y0))
            }
            (false, false) => unreachable!(),
        }
    }
}
impl LineIter {
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn back(&mut self, case: StepCase) {
        match case {
            StepCase::Start => {
                self.first = true;
            }
            StepCase::Dx => {
                self.nsx();
                self.error -= self.dy_abs;
            }
            StepCase::Dy => {
                self.nsy();
                self.error -= self.dx_abs;
            }
            StepCase::Both => {
                self.nsx();
                self.nsy();
                self.error -= self.dx_abs + self.dy_abs;
            }
        }
    }
    fn sy(&mut self) {
        if self.dy_neg {
            self.y0 -= 1;
        } else {
            self.y0 += 1;
        }
    }
    fn sx(&mut self) {
        if self.dx_neg {
            self.x0 -= 1;
        } else {
            self.x0 += 1;
        }
    }
    fn nsy(&mut self) {
        if self.dy_neg {
            self.y0 += 1;
        } else {
            self.y0 -= 1;
        }
    }
    fn nsx(&mut self) {
        if self.dx_neg {
            self.x0 += 1;
        } else {
            self.x0 -= 1;
        }
    }
    #[inline]
    #[must_use]
    pub fn new(x0: u16, y0: u16, x1: u16, y1: u16) -> Self {
        let dx = x1.cast_signed() - x0.cast_signed();
        let dy = y1.cast_signed() - y0.cast_signed();
        let dx_abs = dx.abs();
        let dy_abs = -dy.abs();
        Self {
            x0,
            y0,
            x1,
            y1,
            dx_neg: dx.is_negative(),
            dy_neg: dy.is_negative(),
            dx_abs,
            dy_abs,
            error_test: dx_abs / 2 - dy_abs / 2,
            error: dx_abs + dy_abs - dy_abs / 2,
            first: true,
        }
    }
}
#[test]
fn test_line() {
    let arr = [(3, 2), (4, 2), (5, 3), (6, 3), (7, 4), (8, 4)];
    let mut iter = arr.iter().copied();
    for (_, x, y) in LineIter::new(3, 2, 8, 4) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let mut iter = arr.iter().copied().rev();
    for (_, x, y) in LineIter::new(8, 4, 3, 2) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let arr = [(2, 3), (2, 4), (3, 5), (3, 6), (4, 7), (4, 8)];
    let mut iter = arr.iter().copied();
    for (_, x, y) in LineIter::new(2, 3, 4, 8) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    let mut iter = arr.iter().copied().rev();
    for (_, x, y) in LineIter::new(4, 8, 2, 3) {
        let (nx, ny) = iter.next().unwrap();
        assert_eq!(x, nx, "{x} {y} {nx} {ny}");
        assert_eq!(y, ny, "{x} {y} {nx} {ny}");
    }
    for i in 0..8 {
        for j in 0..8 {
            for k in 0..8 {
                if i == k || (j == 0 && k == 8) {
                    continue;
                }
                let mut iter_a = LineIter::new(i, 0, j, 8).map(|(_, x, y)| (x, y));
                let mut iter_b = LineIter::new(j, 8, 0, k).map(|(_, x, y)| (x, y));
                let mut iter_c = LineIter::new(0, k, i, 0).map(|(_, x, y)| (x, y));
                let start_a = iter_a.next();
                let start_b = iter_b.next();
                let start_c = iter_c.next();
                assert_eq!(start_a, Some((i, 0)), "{i} {j} {k}");
                assert_eq!(start_b, Some((j, 8)), "{i} {j} {k}");
                assert_eq!(start_c, Some((0, k)), "{i} {j} {k}");
                assert_eq!(iter_a.last(), start_b, "{i} {j} {k}");
                assert_eq!(iter_b.last(), start_c, "{i} {j} {k}");
                assert_eq!(iter_c.last(), start_a, "{i} {j} {k}");
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct LineIterCompact {
    pub x0: u16,
    pub y0: u16,
    pub dx_neg: bool,
    pub dy_neg: bool,
    pub dx_abs: i16,
    pub dy_abs: i16,
    pub error: i16,
    pub first: bool,
}
impl Default for LineIterCompact {
    #[inline]
    fn default() -> Self {
        LineIter::default().into()
    }
}
impl From<LineIter> for LineIterCompact {
    #[inline]
    fn from(value: LineIter) -> Self {
        Self {
            x0: value.x0,
            y0: value.y0,
            dx_neg: value.dx_neg,
            dy_neg: value.dy_neg,
            dx_abs: value.dx_abs,
            dy_abs: value.dx_abs,
            error: value.error,
            first: value.first,
        }
    }
}
impl From<LineIterCompact> for LineIter {
    #[inline]
    fn from(value: LineIterCompact) -> Self {
        let x1 = if value.dx_neg {
            value.x0 - value.dx_abs.cast_unsigned()
        } else {
            value.x0 + value.dx_abs.cast_unsigned()
        };
        let y1 = if value.dy_neg {
            value.y0 - value.dy_abs.cast_unsigned()
        } else {
            value.y0 + value.dy_abs.cast_unsigned()
        };
        Self {
            x0: value.x0,
            y0: value.y0,
            x1,
            y1,
            dx_neg: value.dx_neg,
            dy_neg: value.dy_neg,
            dx_abs: value.dx_abs,
            error_test: value.dx_abs / 2 - value.dy_abs / 2,
            dy_abs: value.dx_abs,
            error: value.error,
            first: value.first,
        }
    }
}
