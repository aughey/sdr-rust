use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sdr_rust::{average, average_with_trig, average_optimized};

fn generate_test_data(size: usize) -> Vec<(f64, f64)> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| (rng.gen_range(0.0..360.0), 1.0))
        .collect()
}

fn bench_averages(c: &mut Criterion) {
    let test_data = generate_test_data(100);
    
    let mut group = c.benchmark_group("Average Functions (100 values)");
    
    group.bench_function("original", |b| {
        b.iter(|| average(black_box(&test_data)))
    });
    
    group.bench_function("trig", |b| {
        b.iter(|| average_with_trig(black_box(&test_data)))
    });
    
    group.bench_function("optimized", |b| {
        b.iter(|| average_optimized(black_box(&test_data)))
    });
    
    group.finish();
}

criterion_group!(benches, bench_averages);
criterion_main!(benches); 