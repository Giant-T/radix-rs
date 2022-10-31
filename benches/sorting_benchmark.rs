use radix_rs::{sort};
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};

fn sort_arr_benchmark(c: &mut Criterion) {
    let mut arr = black_box(
        vec![0, 2, 1, 0, 1, 3, 4, 5, 6, 12, 2, 20]
    );

    c.bench_function(
        "sorting algorithm",
        |b| b.iter(|| {sort::radix(& mut arr)})
    );
}

criterion_group!(benches, sort_arr_benchmark);
criterion_main!(benches);