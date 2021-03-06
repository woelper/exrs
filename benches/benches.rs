#[macro_use]
extern crate bencher;

extern crate exr;
use exr::prelude::*;

use bencher::Bencher;
use std::fs;


fn read_single_image_uncompressed(bench: &mut Bencher) {
    bench.iter(||{
        let path = "D:/Pictures/openexr/crowskull/crow_uncompressed.exr";

        let image = FullImage::read_from_file(path, ReadOptions::default()).unwrap();
        bencher::black_box(image);
    })
}

fn read_single_image_uncompressed_from_buffer(bench: &mut Bencher) {
    let file = fs::read("D:/Pictures/openexr/crowskull/crow_uncompressed.exr").unwrap();

    bench.iter(||{
        let image = FullImage::read_from_buffered(file.as_slice(), ReadOptions::fast_loading()).unwrap();
        bencher::black_box(image);
    })
}

fn read_single_image_zips(bench: &mut Bencher) {
    bench.iter(||{
        let path = "D:/Pictures/openexr/crowskull/crow_zips.exr";
        let image = FullImage::read_from_file(path, ReadOptions::default()).unwrap();
        bencher::black_box(image);
    })
}

fn read_single_image_rle(bench: &mut Bencher) {
    bench.iter(||{
        let path = "D:/Pictures/openexr/crowskull/crow_rle.exr";
        let image = FullImage::read_from_file(path, ReadOptions::default()).unwrap();
        bencher::black_box(image);
    })
}

fn read_single_image_non_parallel_zips(bench: &mut Bencher) {
    bench.iter(||{
        let path = "D:/Pictures/openexr/crowskull/crow_zips.exr";
        let options = ReadOptions {
            parallel_decompression: false,
            .. ReadOptions::default()
        };

        let image = FullImage::read_from_file(path, options).unwrap();
        bencher::black_box(image);
    })
}

/*fn write_single_image_parallel_zip(bench: &mut Bencher) {
    let path = "D:/Pictures/openexr/crowskull/crow_zips.exr";
    let options = ReadOptions {
        parallel_decompression: false,
        .. ReadOptions::default()
    };

    let image = FullImage::read_from_file(path, options).unwrap();

    bencher.iter(||{
        let mut result = Vec::new();
        FullImage::write_to_buffered(&image, Cursor::new(&mut result), WriteOptions::debug()).unwrap();
        bencher::black_box(result);
    })
}*/

benchmark_group!(benches,
    read_single_image_uncompressed_from_buffer,
    // write_single_image_parallel_zip,
    read_single_image_uncompressed,
    read_single_image_zips,
    read_single_image_rle,
    read_single_image_non_parallel_zips
);

benchmark_main!(benches);