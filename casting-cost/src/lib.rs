#![feature(test)]

extern crate test;

pub fn cast_u8_to_usize (n: u8, times: usize) {
    for _ in 1..=times {
        let _x = n as usize;
    }
}

pub fn initialise_usize (n: usize, times: usize) {
    let mut _x: usize = 0;
    for _ in 1..=times {
        _x = n;
    }
}
             
// TODO: other types
  
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_few_cast_u8_to_usize(b: &mut Bencher) {
        b.iter(|| cast_u8_to_usize(8 , 90));
    }

    #[bench]
    fn bench_few_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(8 , 90));
    }

    #[bench]
    fn bench_some_cast_u8_to_usize(b: &mut Bencher) {
        b.iter(|| cast_u8_to_usize(8 , 9000));
    }

    #[bench]
    fn bench_some_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(8 , 9000));
    }

    #[bench]
    fn bench_many_cast_u8_to_usize(b: &mut Bencher) {
        b.iter(|| cast_u8_to_usize(8 , 9000000));
    }

    #[bench]
    fn bench_many_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(8 , 9000000));
    }
}
