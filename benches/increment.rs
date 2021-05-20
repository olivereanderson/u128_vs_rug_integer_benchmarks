use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rug::Integer;


fn bench_increment(c: &mut Criterion) {
    let mut group = c.benchmark_group("increment");
    let inputs = [
        3u128,
        2u128.pow(31) -2, 
        2u128.pow(63) + 7,
        2u128.pow(78) - 6,
        2u128.pow(127) + 9,
    ];

    for input in inputs.iter().map(|i| *i) {
        group.bench_with_input(
            BenchmarkId::new(
                "increment u128",
                format!("{}", input),
            ),
            &input,
            |b, input| {
                let mut value = *input; 
                b.iter(|| {
                    value += 1;
                })
            },

        );

        group.bench_with_input(
            BenchmarkId::new(
                "increment rug::Integer",
                format!("{}",input),
            ),
            &Integer::from(input),
            |b, input| {
                let mut value = input.clone(); 
                b.iter(|| {
                    value +=1;
                })
            },
            
        );
    }
    group.finish();
}

criterion_group!(benches, bench_increment);
criterion_main!(benches);