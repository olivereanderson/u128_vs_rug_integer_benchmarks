use std::u128;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rug::Integer;

fn b_divides_a_u128(a: &u128, b: &u128) -> bool {
    (a % b) == 0
}

fn b_divides_a_rug_integer(a: &Integer, b: &Integer) -> bool {
    a.is_divisible(b)
}

fn bench_increment(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_divisible");
    let inputs = [
        (3u128, 7u128),
        (2u128.pow(31) - 2, 2u128.pow(25) - 9),
        ((2u128.pow(63) + 7) * (3u128.pow(11) - 8), 3u128.pow(11) - 8),
        (2u128.pow(78) - 6, 2u128.pow(74) - 5),
        (2u128.pow(127) + 9, 2u128.pow(89) - 53),
    ];

    for input in inputs.iter().map(|i| *i) {
        group.bench_with_input(
            BenchmarkId::new(
                "b_divides_a_u128",
                format!("does {} divide {}?", input.1, input.0),
            ),
            &input,
            |b, input| b.iter(|| b_divides_a_u128(&input.0, &input.1)),
        );

        group.bench_with_input(
            BenchmarkId::new(
                "b_divides_a_rug_integer",
                format!("does {} divide {}?", input.1, input.0),
            ),
            &(Integer::from(input.0), Integer::from(input.1)),
            |b, input| b.iter(|| b_divides_a_rug_integer(&input.0, &input.1)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_increment);
criterion_main!(benches);
