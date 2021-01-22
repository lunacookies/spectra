use tincture::{Hue, Oklab, Oklch};

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

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.65448976, 0.13395809, 85.721306)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(0.58083504, 0.17318209, 39.491676)
    }

    pub(crate) fn red(&self) -> Oklch {
        oklch(0.58628607, 0.20640738, 27.108946)
    }

    pub(crate) fn magenta(&self) -> Oklch {
        oklch(0.5923808, 0.20247188, 355.8826)
    }

    pub(crate) fn violet(&self) -> Oklch {
        oklch(0.58230686, 0.1262059, 279.09988)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.6148633, 0.139451, 244.93845)
    }

    pub(crate) fn cyan(&self) -> Oklch {
        oklch(0.64365506, 0.101929754, 187.41685)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(0.6443937, 0.1508053, 118.61133)
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
