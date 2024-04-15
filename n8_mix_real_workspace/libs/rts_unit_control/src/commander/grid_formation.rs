use std::f32::consts::PI;
use bevy::math::*;

// Could be expanded into a seperate lib
// Grid utils or something
// The formation generated is kinda strange, but it don't really matter

#[derive(Clone, Copy)]
enum PosNegXY {
    PXPY, // 1
    PXNY, // 2
    NXPY, // 3
    NXNY, // 4
}
impl PosNegXY {
    fn step(v: Self) -> Self {
        match v {
            PosNegXY::PXPY => return PosNegXY::PXNY, // 1 -> 2
            PosNegXY::PXNY => return PosNegXY::NXPY, // 2 -> 3
            PosNegXY::NXPY => return PosNegXY::NXNY, // 3 -> 4
            PosNegXY::NXNY => return PosNegXY::PXPY, // 4 -> 1
        }
    }
}

/// x, y iter.
pub struct XYIter {
    iter: u8,
    step: PosNegXY,
    a1_px_py: XYCornerIter,
    b2_px_ny: XYCornerIter,
    c3_nx_py: XYCornerIter,
    d4_nx_ny: XYCornerIter,
} 
impl XYIter {
    pub fn new() -> Self {
        let mut new = Self {
            iter: 0,
            step: PosNegXY::PXPY,
            a1_px_py: XYCornerIter::new(), // positive positive
            b2_px_ny: XYCornerIter::new(), // negative positive
            c3_nx_py: XYCornerIter::new(), // positive negative
            d4_nx_ny: XYCornerIter::new(), // negative negative
        };
        new.d4_nx_ny.next();
        return new;
    }

    pub fn next(
        &mut self
    ) -> I64Vec2 {
        match self.step {
            PosNegXY::PXPY => {
                return self.px_py_next();
            },
            PosNegXY::PXNY => {
                return self.px_ny_next();
            },
            PosNegXY::NXPY => {
                return self.nx_py_next();
            },
            PosNegXY::NXNY => {
                return self.nx_ny_next();
            },
        }
    }

    fn px_py_next(&mut self) -> I64Vec2 { // Happens twice
        let vec = self.a1_px_py.next().as_i64vec2();
        self.iter = self.iter + 1;

        if self.iter == 2 {
            self.iter = 0;
            self.step = PosNegXY::step(self.step);
        }

        return vec;
    }

    fn px_ny_next(&mut self) -> I64Vec2 { // Happens once
        let vec = self.b2_px_ny.next().as_i64vec2();
        let vec = i64vec2(vec.x, -vec.y); // rotate/re-align
        let vec = vec + i64vec2(1, -1); // offset to avoid overlap

        self.step = PosNegXY::step(self.step);

        return vec;
    }

    fn nx_py_next(&mut self) -> I64Vec2 { // Happens once
        let vec = self.c3_nx_py.next().as_i64vec2();
        let vec = i64vec2(-vec.x, vec.y); // rotate/re-align
        let vec = vec + i64vec2(-1, 1); // offset to avoid overlap

        self.step = PosNegXY::step(self.step);

        return vec;
    }

    fn nx_ny_next(&mut self) -> I64Vec2 { // Happens twice
        let vec = self.d4_nx_ny.next().as_i64vec2();
        let vec = -vec; // rotate/re-align

        self.iter = self.iter + 1;
        if self.iter == 2 {
            self.iter = 0;
            self.step = PosNegXY::step(self.step);
        }

        return vec;
    }

}

#[derive(Clone, Copy)]
enum XY {
    X,
    Y,
}
impl XY {
    fn invert(v: Self) -> Self {
        match v {
            XY::X => return XY::Y,
            XY::Y => return XY::X,
        }
    }
}

/// Positive x, positive y iter.
/// 4
/// 2 3
/// 0 1 5
pub struct XYCornerIter{
    base: u16,
    x: u16,
    y: u16,
    flip_flop: XY,
} 
impl XYCornerIter {
    pub fn new() -> Self {
        return Self {
            base: 0,
            x: 0,
            y: 0,
            flip_flop: XY::X
        }
    }

    /// XY base.
    /// X base + Y iter.
    /// Y base + X iter.
    /// When Y iter == base.
    /// Base + 1, reset iters.
    pub fn next(
        &mut self
    ) -> U16Vec2 {
        match self.flip_flop {
            XY::X => self.next_x(),
            XY::Y => self.next_y(),
        }
    }

    fn next_x(
        &mut self
    ) -> U16Vec2 {
        let vec = U16Vec2::new(self.base, self.y);

        self.y = self.y + 1;
        self.flip_flop = XY::invert(self.flip_flop);

        return vec;
    } 

    fn next_y(
        &mut self
    ) -> U16Vec2 {
        if self.x == self.base {
            self.base = self.base + 1;
            self.x = 0;
            self.y = 0;
        }

        let vec = U16Vec2::new(self.x, self.base);

        self.x = self.x + 1;
        self.flip_flop = XY::invert(self.flip_flop);

        return vec;
    }
}