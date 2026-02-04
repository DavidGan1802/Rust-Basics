pub trait Color {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;
    fn h(&self) -> u16;
    fn v(&self) -> f64;
    fn s(&self) -> f64;
}

pub trait ColorOps {
    fn chromatic(&self, n_steps: usize) -> Vec<Box<dyn Color>>;
    fn complement(&self) -> Box<dyn Color>;
    fn triad(&self) -> [Box<dyn Color>; 3];
}

pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    fn max_rgb(&self) -> u8 {
        self.0.max(self.1).max(self.2)
    }
    fn min_rgb(&self) -> u8 {
        self.0.min(self.1).min(self.2)
    }
    fn chroma(&self) -> u8 {
        self.max_rgb() - self.min_rgb()
    }
}

impl Color for Rgb {
    fn r(&self) -> u8 {
        self.0
    }
    fn g(&self) -> u8 {
        self.1
    } 
    fn b(&self) -> u8 {
        self.2
    }
    fn h(&self) -> u16 {
        let a = 0.5 * ((2.0 * self.r() as f64) - self.g() as f64 - self.b() as f64);
        let b = (3.0_f64.sqrt() / 2.0) * (self.g() as f64 - self.b() as f64);
        let h = (b.atan2(a)) * (180.0 / std::f64::consts::PI);
        if h >= 0.0 {
            h as u16
        } else {
            (h + 360.0) as u16
        }
    }
    fn s(&self) -> f64 {
        if self.v() == 0.0 {
            0.0
        } else {
            self.chroma() as f64/self.max_rgb() as f64
        }
    }
    fn v(&self) -> f64 {
        self.max_rgb() as f64/255.0 
    }
}

impl ColorOps for Rgb {
    fn chromatic(&self, n_steps: usize) -> Vec<Box<dyn Color>> {
        let mut colors: Vec<Box<dyn Color>> = Vec::new();
        for i in 0..n_steps {
            let k_r = (5.0 + (self.h() as f64 /60.0)) % 6.0;
            let k_g = (3.0 + (self.h() as f64 /60.0)) % 6.0;
            let k_b = (1.0 + (self.h() as f64 /60.0)) % 6.0;
            let r_chroma = ((255.0)*(i as f64 / (n_steps - 1) as f64 - (i as f64 / (n_steps - 1) as f64 * self.s()) * (0.0_f64.max(k_r.min(1.0).min(4.0 - k_r))))) as u8;
            let g_chroma = ((255.0)*(i as f64 / (n_steps - 1) as f64 - (i as f64 / (n_steps - 1) as f64 * self.s()) * (0.0_f64.max(k_g.min(1.0).min(4.0 - k_g))))) as u8;
            let b_chroma = ((255.0)*(i as f64 / (n_steps - 1) as f64 - (i as f64 / (n_steps - 1) as f64 * self.s()) * (0.0_f64.max(k_b.min(1.0).min(4.0 - k_b))))) as u8;
            colors.push(Box::new(Rgb(r_chroma, g_chroma, b_chroma)));
        }
        colors
    }
    fn complement(&self) -> Box<dyn Color> {
        let k_r = (5.0 + ((self.h()  as f64 + 180.0) % 360.0 /60.0)) % 6.0;
        let k_g = (3.0 + ((self.h()  as f64 + 180.0) % 360.0 /60.0)) % 6.0;
        let k_b = (1.0 + ((self.h()  as f64 + 180.0) % 360.0 /60.0)) % 6.0;
        let r_comp = ((255.0)*(self.v() - (self.v() * self.s()) * (0.0_f64.max(k_r.min(1.0).min(4.0 - k_r))))) as u8;
        let g_comp = ((255.0)*(self.v() - (self.v() * self.s()) * (0.0_f64.max(k_g.min(1.0).min(4.0 - k_g))))) as u8;
        let b_comp = ((255.0)*(self.v() - (self.v() * self.s()) * (0.0_f64.max(k_b.min(1.0).min(4.0 - k_b))))) as u8;
        Box::new(Rgb(r_comp, g_comp, b_comp))
    }
    
    fn triad(&self) -> [Box<dyn Color>; 3] {
        [
            Box::new(Rgb(self.r(), self.g(), self.b())),
            Box::new(Rgb(self.b(), self.r(), self.g())),
            Box::new(Rgb(self.g(), self.b(), self.r())),
        ]
    }
}

pub struct Hsv (pub u16, pub f64, pub f64);

impl Hsv {
    fn get_k(&self, n: f64) -> f64 {
        (n + (self.0  as f64/60.0)) % 6.0
    }
}

impl Color for Hsv {
    fn r(&self) -> u8 {
        ((255.0)*(self.2 - (self.2 * self.1) * (0.0_f64.max(self.get_k(5.0).min(1.0).min(4.0 - self.get_k(5.0)))))) as u8
    }
    fn g(&self) -> u8 {
        ((255.0)*(self.2 - (self.2 * self.1) * (0.0_f64.max(self.get_k(3.0).min(1.0).min(4.0 - self.get_k(3.0)))))) as u8
    } 
    fn b(&self) -> u8 {
        ((255.0)*(self.2 - (self.2 * self.1) * (0.0_f64.max(self.get_k(1.0).min(1.0).min(4.0 - self.get_k(1.0)))))) as u8
    }
    fn h(&self) -> u16 {
        self.0
    }
    fn s(&self) -> f64 {
        self.1
    } 
    fn v(&self) -> f64 {
        self.2
    }   
}

impl ColorOps for Hsv {
    fn chromatic(&self, n_steps: usize) -> Vec<Box<dyn Color>> {
        let mut colors: Vec<Box<dyn Color>> = Vec::new();
        for i in 0..n_steps {
            let v = i as f64 / (n_steps - 1) as f64;
            colors.push(Box::new(Hsv(self.0, self.1, v)));
        }
        colors
    }
    fn complement(&self) -> Box<dyn Color> {
        Box::new(Hsv((self.0 + 180) % 360, self.1, self.2))
    }
    fn triad(&self) -> [Box<dyn Color>; 3] {
        [
            Box::new(Hsv(self.0, self.1, self.2)),
            Box::new(Hsv((self.0 + 120) % 360, self.1, self.2)),
            Box::new(Hsv((self.0 + 240) % 360, self.1, self.2)),
        ]
    }
}


