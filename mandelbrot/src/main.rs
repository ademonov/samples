extern crate num;

use num::Complex;

use std::str::FromStr;

fn main() {}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: &str) -> Option<(T, T)> {
    let index = s.find(separator)?;
    let l = T::from_str(&s[..index]).ok()?;
    let r = T::from_str(&s[index + 1..]).ok()?;
    Some((l, r))
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    let v = parse_pair(s, ",")?;
    Some(Complex { re: v.0, im: v.1 })
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("", ","), None);
        assert_eq!(parse_pair::<i32>("10,", ","), None);
        assert_eq!(parse_pair::<i32>(",10", ","), None);
        assert_eq!(parse_pair::<i32>("10,20", ","), Some((10, 20)));
        assert_eq!(parse_pair::<i32>("10,20xy", ","), None);
        assert_eq!(parse_pair::<f64>("0.5x", "x"), None);
        assert_eq!(parse_pair::<f64>("0.5x1.5", "x"), Some((0.5, 1.5)));
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
        assert_eq!(parse_complex(",-0.0625"), None);
    }

    #[test]
    fn test_pixel_to_point() {

    }
}