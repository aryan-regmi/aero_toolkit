// TODO: Draw NACA XXXX Airfoil

use std::error::Error;

pub struct NacaAirfoil<const N: usize> {
    // X-positions along the chord
    x: [f64; N],

    // Y-positions of the upper surface
    yl: [f64; N],

    // Y-positions of the lower surface
    yu: [f64; N],

    // Current index of the iterator
    idx: usize,
}

impl<const N: usize> Iterator for NacaAirfoil<{ N }> {
    type Item = (f64, f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == N {
            return None;
        }

        self.idx += 1;

        Some((
            self.x[self.idx - 1],
            self.yl[self.idx - 1],
            self.yu[self.idx - 1],
        ))
    }
}

pub fn naca_airfoil_series4<const RESOLUTION: usize>(
    naca_code: &str,
    chord_length: f64,
) -> Result<NacaAirfoil<RESOLUTION>, Box<dyn Error>> {
    // Get thickness from code
    let t = naca_code[2..].parse::<f64>()?;

    // Create x/c vector using RESOLUTION
    let c = chord_length;
    let x = linspace::<RESOLUTION>(0., c as f64);
    let xc = x.map(|val| val / c as f64);

    let mut yu = [0.; RESOLUTION];
    for (i, y) in yu.iter_mut().enumerate() {
        let xc0 = f64::sqrt(xc[i]);
        let xc1 = xc[i];
        let xc2 = xc[i] * xc[i];
        let xc3 = xc[i] * xc[i] * xc[i];
        let xc4 = xc[i] * xc[i] * xc[i] * xc[i];

        *y = 5.
            * t
            * c
            * ((0.2969 * xc0)
                + (-0.1260 * xc1)
                + (-0.3516 * xc2)
                + (0.2843 * xc3)
                + (-0.1036 * xc4));
    }

    let yl = yu.map(|y| -y);

    let airfoil = NacaAirfoil { x, yl, yu, idx: 0 };

    Ok(airfoil)
}

fn linspace<const N: usize>(start: f64, end: f64) -> [f64; N] {
    let mut ret = [start; N];

    let step_size = (end - start) / ((N - 1) as f64);

    for i in 1..N {
        ret[i] = ret[i - 1] + step_size;
    }

    ret[N - 1] = end;

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f32_eql(a: f32, b: f32) -> bool {
        if f32::abs(a - b) > f32::EPSILON {
            return false;
        }

        true
    }

    fn f64_eql(a: f64, b: f64) -> bool {
        if f64::abs(a - b) > f64::EPSILON {
            return false;
        }

        true
    }

    #[test]
    fn test_linspace() {
        const N: usize = 10;

        let result = linspace::<N>(0., 5.);
        dbg!(result);

        assert!(f64_eql(result[0], 0.));
        assert!(f64_eql(result[N - 1], 5.));
        assert_eq!(result.len(), N);
    }

    // #[ignore = "Sanity check"]
    #[test]
    fn it_can_create_naca_4_series_airfoil() {
        let naca_0015 = naca_airfoil_series4::<1000>("0015", 1.).unwrap();

        // Write to file
        let mut naca_0015_str = String::new();
        for (x, yl, yu) in naca_0015 {
            naca_0015_str += &format!("{},{},{}\n", x, yl, yu);
        }
        std::fs::write("OUT.csv", naca_0015_str).unwrap();
    }
}
