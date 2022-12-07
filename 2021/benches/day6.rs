use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use rudvent::day6::sol::{do_it, load_data, DumbFish};


fn criterion_benchmark(c: &mut Criterion) {
    let input = load_data("input.txt");
    let mut fishes = DumbFish::new(input);
    c.bench_function("day-6", move |b| {
        b.iter_batched_ref(|| fishes.clone(), |mut fish: &mut DumbFish<u64>| do_it(fish, 256), BatchSize::SmallInput )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);