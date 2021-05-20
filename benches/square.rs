use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rug::Integer;

fn bench_increment(c: &mut Criterion) {
    let mut group = c.benchmark_group("square");
    let inputs = [
        3u128,
        2u128.pow(63) + 7,
    ];

    for input in inputs.iter().map(|i| *i) {
        group.bench_with_input(
            BenchmarkId::new("square u128", format!("{}", input)),
            &input,
            |b, input| {
                let mut value = *input;
                b.iter(|| {
                    value = value.pow(2);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("square rug::Integer", format!("{}", input)),
            &input,
            |b, input| {
                let mut value = Integer::from(input.clone());
                b.iter(|| {
                    value.square_mut();
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_increment);
criterion_main!(benches);
