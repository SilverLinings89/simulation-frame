

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    real: f64,
    imag: f64,
}

/// An implementation of complex numbers that is limited to the requirements of this codebase and meant to be an exercise for me.
/// <div class="warning"> This is not a full implementation of complex numbers - neither are the operations optimized.</div>
impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    /// I don't yet know how to oberload the plus operator in Rust so I'm using a method instead.
    pub fn add(&self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }

    pub fn multiply(&self, other: Complex) -> Complex {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        Complex::new(real, imag)
    }

    pub fn norm(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn square(&self) -> Complex {
        self.multiply(*self)
    }
}
