pub struct Circumference {
    x: u16,
    y: u16,
    error: u16,
}
impl Iterator for Circumference {
    type Item = (u16, u16);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x.cast_signed() < self.y.cast_signed() {
            None
        } else {
            let (x, y) = (self.x, self.y);
            self.y += 1;
            self.error += self.y;
            if let Some(e) = self.error.checked_sub(self.x) {
                self.error = e;
                self.x = self.x.overflowing_sub(1).0;
            }
            Some((x, y))
        }
    }
}
impl Circumference {
    #[inline]
    #[must_use]
    pub fn new(r: u16) -> Self {
        Self {
            x: r,
            y: 0,
            error: r / 16,
        }
    }
}
