use super::{ComplexNumber, _e, bit_reverse_order, Sign};

const PI: f32 = 3.14159265358979323846264338327950288;

fn create_complex_array(a: &[u8]) -> Vec<ComplexNumber> {
    let mut ca = Vec::new();
    for x in a {
        ca.push(ComplexNumber::from(*x as f32))
    }
    ca
}

fn pad_data(data: &mut Vec<ComplexNumber>) {
    let mut x = 2;
    while x <= usize::MAX / 2 {
        if data.len() <= x {
            break;
        }
        x *= 2;
    }
    for _ in 0..(x - data.len()) {
        data.push(ComplexNumber::default());
    }
}

fn _w(k: f32, n: f32) -> ComplexNumber {
    _e(Sign::Negative, 2.0 * PI * k / n)
}

fn butterflies(a: &mut [ComplexNumber]) {
    let mut w = vec![ComplexNumber::default(); a.len()];
    let (mut xhight, mut k) = (1, a.len() / 2);
    while k > 0 {
        for i in 0..a.len() {
            if i + xhight >= a.len() {
                break;
            }

            let (p, q) = (a[i], a[i + xhight]);
            let (x, y) = (((k * i) % a.len()), (k * i + xhight) % a.len());
            if w[x] == ComplexNumber::default() {
                w[x] = _e(Sign::Negative, 2.0 * PI * x as f32 / a.len() as f32);
            }
            if w[y] == ComplexNumber::default() {
                w[y] = _e(Sign::Negative, 2.0 * PI * y as f32 / a.len() as f32);
            }

            a[i] = p.add(&w[x].mul(&q));
            a[i + xhight] = p.add(&w[y].mul(&q));

            xhight *= 2;
            k /= 2;
        }
    }
}

pub fn fast_fourier_transform(data: &[u8]) -> Vec<ComplexNumber> {
    let mut _data = create_complex_array(data);
    pad_data(&mut _data);
    bit_reverse_order(&mut _data);
    butterflies(&mut _data);
    _data
}

mod tests {
    #![allow(unused)]
    use super::*;

    #[test]
    fn pad_data_test() {
        let mut arr1 = vec![ComplexNumber::default(); 1000];
        pad_data(&mut arr1);
        assert_eq!(arr1.len(), 1024);
        let mut arr2 = vec![ComplexNumber::default(); 15837];
        pad_data(&mut arr2);
        assert_eq!(arr2.len(), 16384);
    }
}