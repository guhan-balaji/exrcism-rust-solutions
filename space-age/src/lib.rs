// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;
    const ORBITAL_PERIOD: f64; // orbital period relative to earth (in years)
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::EARTH_YEAR_IN_SECONDS / Self::ORBITAL_PERIOD
    }
}

macro_rules! planets {
    ( $( ($i:ident, $e:expr) ),* ) => {
        $(
            pub struct $i;
            impl Planet for $i {
                const ORBITAL_PERIOD: f64 = $e;
            }
        )*
    };
}

planets! {
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
}
