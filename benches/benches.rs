use criterion::{black_box, criterion_group, criterion_main, Criterion};
use music_math::{
    binaryops::clip, binaryops::fold, binaryops::wrap, scaling::linexp, scaling::linlin,
};

fn criterion_benchmark(c: &mut Criterion) {
    // clip
    c.bench_function("clip", |b| {
        b.iter(|| clip(black_box(20), black_box(0), black_box(10)))
    });
    // wrap
    c.bench_function("wrap", |b| {
        b.iter(|| wrap(black_box(20.0), black_box(0.0), black_box(10.0)))
    });
    // fold
    c.bench_function("fold", |b| {
        b.iter(|| fold(black_box(20.0), black_box(0.0), black_box(10.0)))
    });

    // ampdb
    c.bench_function("ampdb", |b| {
        b.iter(|| music_math::scaling::ampdb(black_box(20.0)))
    });

    // dbamp
    c.bench_function("dbamp", |b| {
        b.iter(|| music_math::scaling::dbamp(black_box(20.0)))
    });

    // linexp
    c.bench_function("linexp", |b| {
        b.iter(|| {
            linexp(
                black_box(0.5),
                black_box(0.0),
                black_box(1.0),
                black_box(1.0),
                black_box(10.0),
                black_box(2.0),
            )
        })
    });
    // linlin
    c.bench_function("linlin", |b| {
        b.iter(|| {
            linlin(
                black_box(0.5),
                black_box(0.0),
                black_box(1.0),
                black_box(0.0),
                black_box(10.0),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
