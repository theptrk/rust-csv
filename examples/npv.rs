extern crate csv;
extern crate test;

use test::Bencher;
use std::path::Path;
use std::iter::AdditiveIterator;

fn main() {
    let fp = &Path::new("./data/npv.csv");
    let mut rdr = csv::Decoder::from_file(fp);
    let mut vec = Vec::new();

    for (s1, dist) in rdr.decode_iter::<(uint, f64)>() {
        vec.push((s1,dist));
        println!("Year {}: {}", s1, dist);
    }
    println!("vec: {}", vec);
    
    let forecast: Vec<(uint, f64)> = vec.iter().map(|&(a, b)| (a,b)).collect();
    println!("forecast: {}", forecast);

    let mut cfs = Vec::new();
    for i in forecast.iter() {
        let mytup: (uint, f64) = *i;
        match mytup {
            (a, b) => {
                let value = discount(a,b,wacc());
                cfs.push(value);
                println!("Year {}: {}", a, value)
            }
        }
    }

    let val_cfs = cfs.iter().map(|&x| x).sum();
    let last_cf = *cfs.last().unwrap() as f64;
    let val_term = terminal(last_cf, wacc(), 0.03);

    println!("Sum of cash flows: {}", val_cfs);
    println!("Terminal value: {}", val_term);
    println!("Net Present Value: {}", val_cfs + val_term);

    // let discounted: Vec<uint, f64> = forecast.iter().map(|(a,b)|.map(discount).collect();
    // println!("{}", discounted)
    
    // let new: Vec<(uint, f64)> = vec.iter().map(|&p| p).map(discount).enumerate().collect();
    // println!("{}", new);

}

fn wacc() -> f64 {
    // d% * cost of debt + e% * cost of equity 
    // cost of equity === kd + (beta * (mrp - rf))
    // 50% * 6/100 + 50% * 9/100
    // 3/100 + 4.5/100
    (3.0/100.0) + (4.5/100.0)
}

fn discount(year: uint, val: f64, wacc: f64) -> (f64) {
    let factor = (1.0 + wacc).powf(year as f64);
    let discounted = val / factor;
    discounted
}

fn terminal(cf: f64, wacc: f64, growth:f64) -> f64{
    (cf * (1.0 + wacc)) / (wacc - growth)
}

#[bench]
fn bench_add_ten(b: &mut Bencher) {
    b.iter(|| test::black_box(add_ten(5.0)));
}


// ===== Reserve =====

// let reverse: Vec<(f64, uint)> = vec.iter()
//     .map(|&p| p) // we need to dereference what is return from vec.iter();
//     .map(add_ten)
//     .enumerate()
//     .map(|(a, b)| (b, a))
//     .collect();
// println!("{}", reverse);


// for i in vec.iter() {
//     println!("{}", i)
// }


// fn print_vec(v: Vec<f64>) {
//     for i in v.iter() {
//         println!("{}", i)
//     }
// }


// let forecast: Vec<(uint, f64)> = vec.iter().map(|&p| p).enumerate().collect();
// let mut cfs = Vec::new();
// for i in forecast.iter() {
//     let mytup: (uint, f64) = *i;
//     match mytup {
//         (a, b) => {
//             let value = discount((a+1),b,wacc());
//             cfs.push(value);
//             println!("Year {}: {}", (a+1), value)
//         }
//     }
// }