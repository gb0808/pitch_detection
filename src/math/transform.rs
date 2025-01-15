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

fn butterflies(a: &[ComplexNumber], k: f32, n: f32) -> Vec<ComplexNumber> {
    if a.len() == 2 {
        let p = a[0];
        let q = a[1];
        return vec![p.add(&_w(0.0, n).mul(&q)), p.sub(&_w(k, n).mul(&q))]
    }

    let a1 = butterflies(&a[0..a.len()/2], k * 2.0, n);
    let a2 = butterflies(&a[a.len()/2..], k * 2.0, n);

    let mut ap1: Vec<ComplexNumber> = Vec::new();
    let mut ap2: Vec<ComplexNumber> = Vec::new();

    for i in 0..a1.len() {
        let p = a1[i];
        let q = a2[i];
        ap1.push(p.add(&_w(k * i as f32, n).mul(&q)));
        ap2.push(p.sub(&_w(k * i as f32, n).mul(&q)));
    }
    
    [&ap1[..], &ap2[..]].concat()
}

pub fn fast_fourier_transform(data: &[u8]) -> Vec<ComplexNumber> {
    let mut _data = create_complex_array(data);
    pad_data(&mut _data);
    bit_reverse_order(&mut _data);
    butterflies(&_data, 1.0, _data.len() as f32)
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