use tincture::{Hue, Oklab, Oklch};

#[derive(Clone, Copy)]
pub(crate) enum Palette {
    Light,
    Dark,
}

impl Palette {
    pub(crate) fn emphasized(&self) -> Oklch {
        match self {
            Self::Light => self.base01(),
            Self::Dark => self.base1(),
        }
    }

    pub(crate) fn foreground(&self) -> Oklch {
        match self {
            Self::Light => self.base00(),
            Self::Dark => self.base0(),
        }
    }

    pub(crate) fn deemphasized(&self) -> Oklch {
        match self {
            Self::Light => self.base1(),
            Self::Dark => self.base01(),
        }
    }

    pub(crate) fn background_highlights(&self) -> Oklch {
        match self {
            Self::Light => self.base2(),
            Self::Dark => self.base02(),
        }
    }

    pub(crate) fn background(&self) -> Oklch {
        match self {
            Self::Light => self.base3(),
            Self::Dark => self.base03(),
        }
    }

    fn base03(&self) -> Oklch {
        oklch(0.26733097, 0.048632655, 219.83849)
    }

    fn base02(&self) -> Oklch {
        oklch(0.30920133, 0.05178271, 219.67508)
    }

    fn base01(&self) -> Oklch {
        oklch(0.5230095, 0.028337372, 219.20393)
    }

    fn base00(&self) -> Oklch {
        oklch(0.5681621, 0.028546046, 221.96794)
    }

    fn base0(&self) -> Oklch {
        oklch(0.6536782, 0.019755276, 205.40446)
    }

    fn base1(&self) -> Oklch {
        oklch(0.69789714, 0.015926683, 196.99657)
    }

    fn base2(&self) -> Oklch {
        oklch(0.9306106, 0.025954645, 92.4335)
    }

    fn base3(&self) -> Oklch {
        oklch(0.9735303, 0.025970219, 90.12118)
    }

    const L: f32 = 0.65;
    const C: f32 = 0.15;

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(Self::L, 0.125, 85.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(Self::L, Self::C, 40.0)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(Self::L, Self::C, 25.0)
    }

    pub(crate) fn magenta(&self) -> Oklch {
        oklch(Self::L, Self::C, 355.0)
    }

    pub(crate) fn violet(&self) -> Oklch {
        oklch(Self::L, Self::C, 280.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(Self::L, Self::C, 245.0)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(Self::L, 0.11, 185.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(Self::L, Self::C, 120.0)
    }
}

fn oklab(l: f32, a: f32, b: f32) -> Oklab {
    Oklab { l, a, b }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}
