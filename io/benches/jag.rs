use criterion::*;
use criterion::measurement::WallTime;

use io::JagFile;

fn bench_jag_hash(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("jag");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    group.bench_function("hash", move |b| {
        b.iter_batched(
            || (),
            |()| {
                JagFile::hash("abcdefghijklmnopqrstuvwxys0123456789");
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_jag_hash);

criterion_main!(benches);
