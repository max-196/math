use math::Rational;
use math::numbers::big_integer::wide_multiply;
use math::BigUint;
use rand::Rng;

fn main() {
    let r = Rational::new(15, 30);

    println!("{}", (r * Rational::new(1, 2)).reduce());

    let big: BigUint = (u64::MAX as u64).into();
    let big2: BigUint = (16 as u64).into();

//    println!("{}", (big + big2).to_binary());
//    println!("{:128b}", 15 + 16);
//
    let (n1, n2) = (u64::MAX as u128, 2 as u128);
//    let (n1, n2) = (u128::MAX, u128::MAX);
    let b = BigUint::from(n1);
    let b2 = BigUint::from(n2);
    println!("{}", (b * b2).to_binary());
    println!("{:0128b}", (n1 * n2));
    let mut rng = rand::thread_rng();
    for i in 0..10000 {
        let (n1, n2): (u64, u64) = (rng.gen(), rng.gen());
        let b = BigUint::from(n1);
        let b2 = BigUint::from(n2);
        assert_eq!(format!("{:0128b}", (n1 as u128 * n2 as u128)), (b.clone() * b2.clone()).to_binary());
        println!("{}\n{}\n\n", 
 format!("{:0128b}", (n1 as u128 * n2 as u128)), (b * b2).to_binary());
    }
    println!("oll korrekt");

//    println!("{:064b}{:064b}", wide_multiply(n1 as u64, n2 as u64).0, wide_multiply(n1 as u64, n2 as u64).1);
}
