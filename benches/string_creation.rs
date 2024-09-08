use criterion::{criterion_group, criterion_main, Criterion, black_box};
use static_id::static_id::*;

fn making_id(c: &mut Criterion) {
    let mut group = c.benchmark_group("making_id_safe");
    let array: &[u8; 16] = b"AAAAAAAAAAAAAAAA";
    let slice: &[u8] = array;

    group.bench_function("StaticId::from_bytes", |b| b.iter(|| {
        let _ = StaticId::from_bytes(black_box(slice), black_box(slice));
    }));

    let array: & [u8; 16] = b"BBBBBBBBBBBBBBBB";
    let slice: &[u8] = array;
    let _ = StaticId::from_bytes(slice, slice);

    group.bench_function("StaticId::from_bytes existing object", |b| b.iter(|| {
        let _ = black_box(StaticId::from_bytes(black_box(slice), black_box(slice)));
    }));

    group.finish();
}

criterion_group!(
    benches, 
    making_id,
);
criterion_main!(benches);