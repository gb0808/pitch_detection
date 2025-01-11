use super::ComplexNumber;

pub fn bit_reverse_order(arr: &mut Vec<ComplexNumber>) {
    let arr_length = arr.len();
    let bit_length = format!("{:b}", arr_length - 1).len();
    for j in 0..arr_length {
        let nj = reverse_bits(j, bit_length);
        if j < nj {
            let temp = arr[j];
            arr[j] = arr[nj];
            arr[nj] = temp;
        }
    }
}

fn reverse_string(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut _string = String::new();
    for i in (0..s.len()).rev() {
        _string.push(bytes[i] as char);
    }
    _string
}

fn reverse_bits(x: usize, bit_length: usize) -> usize {
    let s = format!("{:0bit_length$b}", x);
    let _s = reverse_string(&s);
    usize::from_str_radix(&_s, 2).unwrap()
}

mod tests {
    #![allow(unused)]
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn reverse_bits_test() {
        assert_eq!(reverse_bits(3, 3), 6);
        assert_eq!(reverse_bits(4, 3), 1);
        assert_eq!(reverse_bits(8, 4), 1);
        assert_eq!(reverse_bits(2, 3), 2);
        assert_eq!(reverse_bits(3, 4), 12);
    }

    #[test]
    fn bit_reverse_order_test() {
        let mut arr = vec![
            ComplexNumber::from(0.0), ComplexNumber::from(1.0),
            ComplexNumber::from(2.0), ComplexNumber::from(3.0),
            ComplexNumber::from(4.0), ComplexNumber::from(5.0),
            ComplexNumber::from(6.0), ComplexNumber::from(7.0)
        ];
        bit_reverse_order(&mut arr);
        assert_eq!(arr[0], ComplexNumber::from(0.0));
        assert_eq!(arr[1], ComplexNumber::from(4.0));
        assert_eq!(arr[2], ComplexNumber::from(2.0));
        assert_eq!(arr[3], ComplexNumber::from(6.0));
        assert_eq!(arr[4], ComplexNumber::from(1.0));
        assert_eq!(arr[5], ComplexNumber::from(5.0));
        assert_eq!(arr[6], ComplexNumber::from(3.0));
        assert_eq!(arr[7], ComplexNumber::from(7.0));
    }
}