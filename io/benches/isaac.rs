use criterion::measurement::WallTime;
use criterion::*;

use io::Isaac;

fn bench_isaac(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("isaac");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let isaac: Isaac = Isaac::new(vec![0, 0, 0, 0]);

    group.bench_function("next", move |b| {
        b.iter_batched(
            || isaac.clone(),
            |mut isaac| {
                for _ in 0..1000000 {
                    isaac.next();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn bench_isaac_seeded(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("isaac");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let isaac: Isaac = Isaac::new(vec![1, 2, 3, 4]);

    group.bench_function("next_seeded", move |b| {
        b.iter_batched(
            || isaac.clone(),
            |mut isaac| {
                for _ in 0..1000000 {
                    isaac.next();
                }
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_isaac, bench_isaac_seeded);

criterion_main!(benches);
