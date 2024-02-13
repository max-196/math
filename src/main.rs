use math::Rational;
use math::BigInteger;

fn main() {
    let r = Rational::new(15, 30);

    println!("{}", (r * Rational::new(1, 2)).reduce());

    let big: BigInteger = (u64::MAX as u64).into();
    let big2: BigInteger = (16 as u64).into();

    println!("{}", (big + big2).to_binary());
    println!("{:128b}", 15 + 16);

    let mut t = Vec::new();
    for i in 0..(10000 as u128) {
        dbg!(i);
        for j in (u128::MAX - 30000)..( (u128::MAX - 20000)as u128) {
            let big: BigInteger = (i).into();
            let big2: BigInteger = (j).into();
            let s = std::time::Instant::now();
            let big = big + big2;
            // dbg!(s.elapsed());
            assert_eq!(format!("{:0128b}", i + j), big.to_binary());
        }


    }

    let mut sum = 0.0;
    for i in t.iter() {
        sum += i;
    }
    println!("{}s", sum / t.len() as f64);

}
