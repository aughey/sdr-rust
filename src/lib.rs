use num_complex::Complex;
use std::f64::consts::PI;

/// Creates a base complex number for degree angle calculations
pub fn create_degrees_base() -> Complex<f64> {
    //    base = cmath.e ** (1j * tau / 360)

    const TAU: f64 = 2.0 * PI;
    Complex::new(0.0, TAU / 360.0).exp()
}

fn angle_mag_to_complex((angle, magnitude): &(f64, f64), base: Complex<f64>) -> Complex<f64> {
    base.powf(*angle) * *magnitude
}

/// Calculates the average of complex numbers represented as (angle, magnitude) pairs
/// Returns a tuple of (average_angle, average_magnitude)
pub fn average(readings: &[(f64, f64)]) -> (f64, f64) {
    let base = create_degrees_base();
    let total: Complex<f64> = readings
        .iter()
        .map(|angle_magnitude| angle_mag_to_complex(angle_magnitude, base))
        .sum();

    let result = total / readings.len() as f64;
    let angle = result.ln() / base.ln();
    (angle.re, result.norm())
}

fn reading_to_axis((angle, magnitude): &(f64, f64)) -> (f64, f64) {
    let angle_radians = angle * PI / 180.0;
    let x = angle_radians.cos();
    let y = angle_radians.sin();
    (x * magnitude, y * magnitude)
}

pub fn average_with_trig(readings: &[(f64, f64)]) -> (f64, f64) {
    let axis_readings = readings.iter().map(reading_to_axis);
    let (sum_x, sum_y) =
        axis_readings.fold((0.0, 0.0), |(sum_x, sum_y), (x, y)| (sum_x + x, sum_y + y));
    let sum_magnitude = (sum_x.powi(2) + sum_y.powi(2)).sqrt();
    let (sum_x, sum_y) = (sum_x / sum_magnitude, sum_y / sum_magnitude);

    let angle = sum_y.atan2(sum_x);
    let magnitude = (sum_x.powi(2) + sum_y.powi(2)).sqrt();
    let angle_degrees = angle * 180.0 / PI;
    (angle_degrees, magnitude)
}

pub fn average_optimized(readings: &[(f64, f64)]) -> (f64, f64) {
    // Calculate constants once at runtime
    let base = create_degrees_base();

    // Single pass accumulation with direct complex multiplication
    let total: Complex<f64> = readings
        .iter()
        .fold(Complex::new(0.0, 0.0), |acc, &(angle, magnitude)| {
            acc + magnitude * base.powf(angle)
        });

    let result = total / readings.len() as f64;
    let angle = result.ln() / base.ln();
    (angle.re, result.norm())
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_average_1() {
        let readings = vec![
            (12.0, 1.0),
            (15.0, 1.0),
            (13.0, 1.0),
            (9.0, 1.0),
            (16.0, 1.0),
        ];
        let (angle, magnitude) = average(&readings);
        assert_relative_eq!(angle, 13.0, epsilon = 0.1);
        assert_relative_eq!(magnitude, 1.0, epsilon = 0.1);

        let (angle, magnitude) = average_with_trig(&readings);
        assert_relative_eq!(angle, 13.0, epsilon = 0.1);
        assert_relative_eq!(magnitude, 1.0, epsilon = 0.1);
    }

    #[test]
    fn test_average_2() {
        let readings = vec![
            (358.0, 1.0),
            (1.0, 1.0),
            (359.0, 1.0),
            (355.0, 1.0),
            (2.0, 1.0),
        ];
        let (angle, magnitude) = average(&readings);
        assert_relative_eq!(angle, -1.0, epsilon = 0.1);
        assert_relative_eq!(magnitude, 1.0, epsilon = 0.1);
    }

    #[test]
    fn test_average_3() {
        let readings = vec![
            (210.0, 1.0),
            (290.0, 1.0),
            (10.0, 1.0),
            (90.0, 1.0),
            (170.0, 1.0),
        ];
        let (angle, magnitude) = average(&readings);
        assert_relative_eq!(angle, -170.0, epsilon = 0.1);
        assert_relative_eq!(magnitude, 0.106, epsilon = 0.1);
    }

    #[test]
    fn test_average_optimized() {
        let readings = vec![
            (12.0, 1.0),
            (15.0, 1.0),
            (13.0, 1.0),
            (9.0, 1.0),
            (16.0, 1.0),
        ];
        let (angle, magnitude) = average_optimized(&readings);
        assert_relative_eq!(angle, 13.0, epsilon = 0.1);
        assert_relative_eq!(magnitude, 1.0, epsilon = 0.1);

        let readings = vec![
            (358.0, 1.0),
            (1.0, 1.0),
            (359.0, 1.0),
            (355.0, 1.0),
            (2.0, 1.0),
        ];
        let (angle, magnitude) = average_optimized(&readings);
        assert_relative_eq!(angle, -1.0, epsilon = 0.1);
        assert_relative_eq!(magnitude, 1.0, epsilon = 0.1);

        let readings = vec![
            (210.0, 1.0),
            (290.0, 1.0),
            (10.0, 1.0),
            (90.0, 1.0),
            (170.0, 1.0),
        ];
        let (angle, magnitude) = average_optimized(&readings);
        assert_relative_eq!(angle, -170.0, epsilon = 0.1);
        assert_relative_eq!(magnitude, 0.106, epsilon = 0.1);
    }

    #[test]
    fn test_reading_to_axis_and_angle_mag_to_complex_are_the_same() {
        let base = create_degrees_base();
        let readings = vec![
            (12.0, 1.0),
            (15.0, 1.0),
            (13.0, 1.0),
            (9.0, 1.0),
            (16.0, 1.0),
        ];
        let complex_readings = readings
            .iter()
            .map(|angle_magnitude| angle_mag_to_complex(angle_magnitude, base));
        let axis_readings = readings.iter().map(reading_to_axis);
        for (complex_reading, axis_reading) in complex_readings.zip(axis_readings) {
            assert_relative_eq!(complex_reading.re, axis_reading.0, epsilon = 0.1);
            assert_relative_eq!(complex_reading.im, axis_reading.1, epsilon = 0.1);
        }
    }
}
