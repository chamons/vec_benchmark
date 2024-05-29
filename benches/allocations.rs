use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vec_benchmark::{
    function_that_returns_enum, function_that_returns_only_single_enum,
    function_that_returns_single_vec, function_that_returns_vec,
};

fn allocation_speed_benchmark(c: &mut Criterion) {
    c.bench_function("returning", |b| {
        b.iter(|| {
            // let v = function_that_returns_vec();
            // let v = function_that_returns_single_vec();
            // let v = function_that_returns_enum();
            let v = function_that_returns_only_single_enum();
            black_box(v);
        })
    });
}

criterion_group!(benches, allocation_speed_benchmark);
criterion_main!(benches);
