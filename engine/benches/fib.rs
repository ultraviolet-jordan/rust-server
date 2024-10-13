use criterion::*;
use criterion::measurement::WallTime;

use cache::{ScriptProvider, ScriptState};
use engine::script::script_runner;

fn bench_fib(c: &mut Criterion) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("script");

    // Define the throughput in operations (you can use 1 if it's per operation)
    group.throughput(Throughput::Elements(1)); // Measure as ops/second

    std::env::set_var("COMPILER_VERSION", "19");
    let script_provider: ScriptProvider = ScriptProvider::io("../data/pack");

    group.bench_function("fib", move |b| {
        b.iter_batched(
            || script_provider.clone(),
            |mut provider| {
                provider.get_by_name(
                    "[proc,fib]",
                    |script| {
                        let mut state: ScriptState =
                            ScriptState::new_with_args(script, vec![45], Vec::new());
                        state.execute(&provider, script_runner, true);
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
