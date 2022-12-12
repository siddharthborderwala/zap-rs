use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zap_core::Zap;

fn bench_set(c: &mut Criterion) {
    let mut zap = Zap::new();
    c.bench_function("zap name - John", |b| {
        b.iter(|| zap.set(black_box("name"), black_box("John")))
    });
}

fn bench_get(c: &mut Criterion) {
    let mut zap = Zap::new();
    zap.set("name", "John");
    c.bench_function("zap get name", |b| b.iter(|| zap.get(black_box("name"))));
}

fn bench_has(c: &mut Criterion) {
    let mut zap = Zap::new();
    zap.set("name", "John");
    c.bench_function("zap has name", |b| b.iter(|| zap.has(black_box("name"))));
}

fn bench_delete(c: &mut Criterion) {
    let mut zap = Zap::new();
    zap.set("name", "John");
    c.bench_function("zap delete name", |b| {
        b.iter(|| zap.delete(black_box("name")))
    });
}

criterion_group!(benches, bench_set, bench_get, bench_has, bench_delete);
criterion_main!(benches);
