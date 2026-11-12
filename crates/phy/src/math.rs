//! Minimal floating-point math for `no_std`.
//!
//! `core` doesn't expose `sin`, `cos`, `ln`, `powf`, `round` as stable
//! inherent methods. Implementasi numerik sederhana yang cukup untuk DFT dan
//! kanal AWGN.

use core::f64::consts::PI;

/// Sinus via deret Taylor dengan reduksi rentang ke [-pi, pi].
#[must_use]
#[allow(clippy::while_float, clippy::cast_precision_loss)]
pub fn sin(x: f64) -> f64 {
    let mut x = x;
    while x > PI {
        x -= 2.0 * PI;
    }
    while x < -PI {
        x += 2.0 * PI;
    }
    let mut result = x;
    let mut term = x;
    let x2 = x * x;
    for n in 1..14usize {
        term *= -x2 / ((2 * n) * (2 * n + 1)) as f64;
        result += term;
    }
    result
}

/// Kosinus via sin(x + pi/2).
#[must_use]
pub fn cos(x: f64) -> f64 {
    sin(x + PI / 2.0)
}

/// Akar kuadrat via metode Newton.
#[must_use]
pub fn sqrt(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    let mut guess = x;
    for _ in 0..30 {
        let next = 0.5 * (guess + x / guess);
        if abs(next - guess) < 1e-15 {
            break;
        }
        guess = next;
    }
    guess
}

/// Logaritma natural via atanh: ln(x) = 2 * atanh((x-1)/(x+1)).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn ln(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NEG_INFINITY;
    }
    let z = (x - 1.0) / (x + 1.0);
    let z2 = z * z;
    let mut result = z;
    let mut term = z;
    for n in 1..24usize {
        term *= z2;
        result += term / (2 * n + 1) as f64;
    }
    2.0 * result
}

/// Eksponensial via deret Taylor.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn exp(x: f64) -> f64 {
    let mut result = 1.0;
    let mut term = 1.0;
    for n in 1..44usize {
        term *= x / n as f64;
        result += term;
        if abs(term) < 1e-18 {
            break;
        }
    }
    result
}

/// x^y = exp(y * ln(x)).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn powf(x: f64, y: f64) -> f64 {
    exp(y * ln(x))
}

/// Pembulatan ke i32 (setengah menjauh dari nol).
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn round_i32(x: f64) -> i32 {
    if x >= 0.0 {
        (x + 0.5) as i32
    } else {
        -((-x + 0.5) as i32)
    }
}

/// Nilai mutlak.
#[must_use]
pub fn abs(x: f64) -> f64 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

#[cfg(test)]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::uninlined_format_args
)]
mod tests {
    use super::*;

    #[test]
    fn sin_cos_basic() {
        assert!((sin(0.0) - 0.0).abs() < 1e-10);
        assert!((sin(PI / 2.0) - 1.0).abs() < 1e-8);
        assert!((cos(0.0) - 1.0).abs() < 1e-10);
        assert!((cos(PI) - -1.0).abs() < 1e-8);
    }

    #[test]
    fn sqrt_basic() {
        assert!((sqrt(4.0) - 2.0).abs() < 1e-10);
        assert!((sqrt(2.0) * sqrt(2.0) - 2.0).abs() < 1e-8);
    }

    #[test]
    fn ln_exp_basic() {
        assert!((ln(1.0) - 0.0).abs() < 1e-10);
        assert!((exp(0.0) - 1.0).abs() < 1e-10);
        assert!((ln(exp(1.0)) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn powf_basic() {
        assert!((powf(2.0, 3.0) - 8.0).abs() < 1e-6);
    }

    #[test]
    fn round_i32_basic() {
        assert_eq!(round_i32(1.2), 1);
        assert_eq!(round_i32(1.5), 2);
        assert_eq!(round_i32(1.7), 2);
        assert_eq!(round_i32(-1.2), -1);
        assert_eq!(round_i32(-1.5), -2);
        assert_eq!(round_i32(-1.7), -2);
    }
}
