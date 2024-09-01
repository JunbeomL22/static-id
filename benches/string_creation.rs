use criterion::{criterion_group, criterion_main, Criterion, black_box};
use static_id::{
    Code,
    Venue,
    StaticId,
    //LargeStaticId,
};

fn making_id(c: &mut Criterion) {
    let mut group = c.benchmark_group("making_code_from_big_slice");
    let array: & [u8; 16] = b"AAAAAAAAAAAAAAAA";
    let slice: &[u8] = array;

    group.bench_function("Code::from", |b| b.iter(|| {
        let _ = black_box(Code::from(slice));
    }));

    group.bench_function("Venue::from", |b| b.iter(|| {
        let _ = black_box(Venue::from(slice));
    }));

    group.bench_function("StaticId::from_bytes", |b| b.iter(|| {
        let _ = black_box(StaticId::from_bytes(slice, slice));
    }));

    group.finish();
}

criterion_group!(
    benches, 
    making_id,
);
criterion_main!(benches);