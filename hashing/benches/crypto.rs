use criterion::*;
use rand::RngCore;

fn bench(c: &mut Criterion) {
    let mut rand_generator = black_box(rand::rngs::OsRng {});

    // 100B
    let mut data = black_box(vec![0u8; 100]);
    rand_generator.fill_bytes(&mut data);

    let mut group = c.benchmark_group("100B");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(&data);
        });
    });
    group.finish();


    // 1kB
    let mut data = black_box(vec![0u8; 1000]);
    rand_generator.fill_bytes(&mut data);

    let mut group = c.benchmark_group("1kB");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(&data);
        });
    });
    group.finish();


    // 100kB
    let mut data = black_box(vec![0u8; 100_000]);
    rand_generator.fill_bytes(&mut data);

    let mut group = c.benchmark_group("100kB");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(&data);
        });
    });
    group.finish();

    // 1MB
    let mut data = black_box(vec![0u8; 1_000_000]);
    rand_generator.fill_bytes(&mut data);

    let mut group = c.benchmark_group("1MB");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(&data);
        });
    });
    group.finish();


    // 10MB
    let mut data = black_box(vec![0u8; 10_000_000]);
    rand_generator.fill_bytes(&mut data);

    let mut group = c.benchmark_group("10MB");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(&data);
        });
    });
    group.finish();


    // 100MB
    let mut data = black_box(vec![0u8; 100_000_000]);
    rand_generator.fill_bytes(&mut data);

    let mut group = c.benchmark_group("100MB");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(&data);
        });
    });
    group.finish();


    // 1GB
    let mut data = black_box(vec![0u8; 1_000_000_000]);
    rand_generator.fill_bytes(&mut data);

    let mut group = c.benchmark_group("1GB");
    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("blake3", |b| {
        b.iter(|| {
            blake3::hash(&data);
        });
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
