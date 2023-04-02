use criterion::*;
use rand::RngCore;

fn bench(c: &mut Criterion) {
    let mut rand_generator = black_box(rand::rngs::OsRng {});
    let average_size = 64 * 1024;
    let min_size = average_size / 4;
    let max_size = average_size * 4;

    let mut group = c.benchmark_group("chunking");

    for size in [64_000, 1_000_000, 100_000_000] {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("fast_cdc_v2016", size), &size, |b, i| {
            let mut data = black_box(vec![0u8; *i]);
            rand_generator.fill_bytes(&mut data);

            b.iter(|| {
                let chunks = fastcdc::v2016::FastCDC::new(&data, min_size, average_size, max_size);
                for chunk in chunks {
                    _ = chunk
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("fast_cdc_v2020", size), &size, |b, i| {
            let mut data = black_box(vec![0u8; *i]);
            rand_generator.fill_bytes(&mut data);

            b.iter(|| {
                let chunks = fastcdc::v2020::FastCDC::new(&data, min_size, average_size, max_size);
                for chunk in chunks {
                    _ = chunk
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("quick_cdc", size), &size, |b, i| {
            let mut data = black_box(vec![0u8; *i]);
            rand_generator.fill_bytes(&mut data);
            let salt = 15222894464462204665;

            b.iter(|| {
                let chunker = quickcdc::Chunker::with_params(&data, 64, 128, salt).expect("building chunker");
                for chunk in chunker {
                    _ = chunk
                }
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
