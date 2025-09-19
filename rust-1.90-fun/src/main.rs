fn main() {
  reverse();
  floor();
  ceil();
  trunc();
  fract();
  round();
  round_ties_even();
}

fn round_ties_even(){
    let f = 3.3_f32;
    let g = -3.3_f32;
    let h = 3.5_f32;
    let i = 4.5_f32;
    assert_eq!(f.round_ties_even(), 3.0);
    assert_eq!(g.round_ties_even(), -3.0);
    assert_eq!(h.round_ties_even(), 4.0);
    assert_eq!(i.round_ties_even(), 4.0);
    println!("{} {} {} {}", f.round_ties_even(), g.round_ties_even(), h.round_ties_even(), i.round_ties_even());
}

fn round() {
    let f = 3.3_f32;
    let g = -3.3_f32;
    let h = -3.7_f32;
    let i = 3.5_f32;
    let j = 4.5_f32;
    assert_eq!(f.round(), 3.0);
    assert_eq!(g.round(), -3.0);
    assert_eq!(h.round(), -4.0);
    assert_eq!(i.round(), 4.0);
    assert_eq!(j.round(), 5.0);
    println!("{} {} {} {} {}", f.round(), g.round(), h.round(), i.round(), j.round());
}

fn fract(){
    let x = 3.6_f32;
    let y = -3.6_f32;
    let abs_difference_x = (x.fract() - 0.6).abs();
    let abs_difference_y = (y.fract() - (-0.6)).abs();
    assert!(abs_difference_x <= f32::EPSILON);
    assert!(abs_difference_y <= f32::EPSILON);
    println!("{} {}", x.fract(), y.fract());
}

fn trunc(){
    let f = 3.7_f32;
    let g = 3.0_f32;
    let h = -3.7_f32;
    assert_eq!(f.trunc(), 3.0);
    assert_eq!(g.trunc(), 3.0);
    assert_eq!(h.trunc(), -3.0);
    println!("{} {} {}", f.trunc(), g.trunc(), h.trunc());
}

fn ceil(){
    let f = 3.01_f32;
    let g = 4.0_f32;
    assert_eq!(f.ceil(), 4.0);
    assert_eq!(g.ceil(), 4.0);
    println!("{} {}", f.ceil(), g.ceil());
}

fn reverse(){
    let mut v = [1, 2, 3];
    v.reverse();
    assert!(v == [3, 2, 1]);
    println!("{:?}", v);
}

fn floor(){
    let f = 3.7_f32;
    let g = 3.0_f32;
    let h = -3.7_f32;
    assert_eq!(f.floor(), 3.0);
    assert_eq!(g.floor(), 3.0);
    assert_eq!(h.floor(), -4.0);
    println!("{} {} {}", f.floor(), g.floor(), h.floor());
}
