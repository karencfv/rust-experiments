#![feature(test)]

extern crate test;

pub fn cast_u8_to_usize (n: u8, times: usize) {
    for _ in 1..=times {
        let _x = n as usize;
    }
}

pub fn cast_u64_to_usize (n: u64, times: usize) {
    for _ in 1..=times {
        let _x = n as usize;
    }
}

pub fn cast_u8_to_u64 (n: u8, times: usize) {
    for _ in 1..=times {
        let _x = n as u64;
    }
}

pub fn initialise_usize (n: usize, times: usize) {
    let mut _x: usize = 0;
    for _ in 1..=times {
        _x = n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_few_cast_u8_to_usize(b: &mut Bencher) {
        b.iter(|| cast_u8_to_usize(8 , 90));
    }

    #[bench]
    fn bench_few_u8_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(8 , 90));
    }

    #[bench]
    fn bench_some_cast_u8_to_usize(b: &mut Bencher) {
        b.iter(|| cast_u8_to_usize(8 , 9000));
    }

    #[bench]
    fn bench_some_u8_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(8 , 9000));
    }

    #[bench]
    fn bench_many_cast_u8_to_usize(b: &mut Bencher) {
        b.iter(|| cast_u8_to_usize(8 , 9000000));
    }

    #[bench]
    fn bench_many_u8_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(8 , 9000000));
    }

    #[bench]
    fn bench_few_cast_u64_to_usize(b: &mut Bencher) {  
        b.iter(|| cast_u64_to_usize(u64::MAX , 90));
    }

    #[bench]
    fn bench_few_u64_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(4294967295, 90));
    }

    #[bench]
    fn bench_some_cast_u64_to_usize(b: &mut Bencher) {
        b.iter(|| cast_u64_to_usize(u64::MAX , 9000));
    }

    #[bench]
    fn bench_some_u64_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(4294967295, 9000));
    }

    #[bench]
    fn bench_many_cast_u64_to_usize(b: &mut Bencher) {
        b.iter(|| cast_u64_to_usize(u64::MAX , 9000000));
    }

    #[bench]
    fn bench_many_u64_initialise_usize(b: &mut Bencher) {
        b.iter(|| initialise_usize(4294967295, 9000000));
    }
}
