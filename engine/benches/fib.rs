use criterion::*;
use criterion::measurement::WallTime;

use cache::{CacheProvider, ScriptProvider, ScriptState};
use engine::engine::Engine;

fn bench_fib(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("script");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    let engine: Engine = Engine::new(CacheProvider::new("../data/pack", "19".to_string(), true));

    let script_provider: ScriptProvider = ScriptProvider::io("../data/pack", "19".to_string());

    group.bench_function("fib", move |b| {
        b.iter_batched(
            || script_provider.clone(),
            |provider| {
                provider.on_by_name(
                    "[proc,fib]",
                    |script| {
                        let mut state: ScriptState =
                            ScriptState::new_with_args(script, vec![45], Vec::new());
                        let _ = state.execute(&engine, true);
                    },
                    || {},
                );
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}
criterion_group!(benches, bench_fib);

criterion_main!(benches);
