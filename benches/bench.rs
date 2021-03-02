use std::time::Duration;

use criterion::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("flat-sampling");
    group.sampling_mode(SamplingMode::Flat);

    group.bench_function("sieve u8", |b| b.iter(|| erasto::sieve_u8_d()));
    group.bench_function("sieve u8d", |b| b.iter(|| erasto::sieve_u8_d()));
    group.bench_function("sieve u8 init", |b| b.iter(|| erasto::sieve_u8_init()));
    group.bench_function("sieve bools", |b| b.iter(|| erasto::sieve_bools()));
    group.bench_function("sieve bools dist", |b| b.iter(|| erasto::sieve_bools_dist()));

    group.finish();
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!{
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(1000);
    targets = criterion_benchmark
}
criterion_main!(benches);