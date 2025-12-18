use cli::dump;
use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn dump_hex(c: &mut Criterion) {
    let path = PathBuf::from("../target/debug/app.exe");
    c.bench_function("dump_app", |b| {
        b.iter(|| dump(black_box(path.clone()), black_box(None)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(100)
        .warm_up_time(std::time::Duration::from_secs(10))
        .measurement_time(std::time::Duration::from_secs(60))
        .nresamples(100_000);
    targets = dump_hex
}
criterion_main!(benches);
