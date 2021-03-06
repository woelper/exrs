use std::panic::{catch_unwind, resume_unwind};
use exr::image::ReadOptions;
use std::panic;
use rayon::prelude::*;
use rand::{thread_rng, Rng, SeedableRng};
use rand::rngs::StdRng;

#[test]
pub fn incremental(){
    println!("started incremental fuzzing");
//    panic::set_hook(Box::new(|_| (/* do not println panics */)));
    let mut pool = rayon::ThreadPoolBuilder::new().build().unwrap();

    for len in 0 .. 32 {
        println!("starting fuzzy testing for byte length of {}", len);

        for variation_index in 0 .. 256_u64.pow(len as u32) {
            pool.install(|| {
                let mut bytes = vec![0_u8; len];

                for index in 0..len {
                    let base = len - index - 1;
                    let range = 256_u64.pow(base as u32);

                    bytes[index] = (variation_index / range as u64) as u8;
                }

                if catch_unwind(|| test_bytes(&bytes)).is_err() {
                    println!("found panics for bytes {:?}", bytes);
                }
            })
        }
    }
}


#[test]
pub fn stochastic(){
    println!("started stochastic fuzzing");
    let mut pool = rayon::ThreadPoolBuilder::new().build().unwrap();

    for index in 0..1024_u64 * 2048 {
        pool.install(move || {
            let mut seed = [0_u8; 32];
            for slice in seed.chunks_exact_mut(8) {
                slice.copy_from_slice(&index.to_le_bytes());
            }

            let mut random: StdRng = rand::SeedableRng::from_seed(seed);

            let len: usize = random.gen_range(0, 2048*16);
            let bytes: Vec<u8> = (0..len).map(|_| random.gen()).collect();

            if catch_unwind(|| test_bytes(&bytes)).is_err() {
                println!("found panics for bytes {:?}", bytes);
            }
        })
    }
}

// should not panic
pub fn test_bytes(bytes: &[u8]) {
    let _result = bencher::black_box(exr::image::full::FullImage::read_from_buffered(
        bytes, ReadOptions::debug()
    ));
}