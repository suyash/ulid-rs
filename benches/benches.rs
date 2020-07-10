use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Utc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ulid_rs::Ulid;

fn new(c: &mut Criterion) {
    c.bench_function("new", |b| {
        b.iter(|| Ulid::new(black_box(20), black_box(|| 4)))
    });
}

fn new_systemtime_now(c: &mut Criterion) {
    c.bench_function("new_systemtime_now", |b| {
        b.iter(|| {
            Ulid::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                black_box(|| 4),
            )
        })
    });
}

fn new_utc_now(c: &mut Criterion) {
    c.bench_function("new_utc_now", |b| {
        b.iter(|| Ulid::new(Utc::now().timestamp() as u64, black_box(|| 4)))
    });
}

fn new_rand_random(c: &mut Criterion) {
    c.bench_function("new_rand_random", |b| {
        b.iter(|| Ulid::new(black_box(20), || rand::random()))
    });
}

fn new_systemtime_now_rand_random(c: &mut Criterion) {
    c.bench_function("new_systemtime_now_rand_random", |b| {
        b.iter(|| {
            Ulid::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                || rand::random(),
            )
        })
    });
}

fn new_utc_now_rand_random(c: &mut Criterion) {
    c.bench_function("new_utc_now_rand_random", |b| {
        b.iter(|| Ulid::new(Utc::now().timestamp() as u64, || rand::random()))
    });
}

fn marshal(c: &mut Criterion) {
    let ulid = Ulid::new(Utc::now().timestamp() as u64, || rand::random());
    c.bench_function("marshal", |b| b.iter(|| ulid.marshal()));
}

fn marshal_to_string(c: &mut Criterion) {
    let ulid = Ulid::new(Utc::now().timestamp() as u64, || rand::random());
    c.bench_function("marshal_to_string", |b| b.iter(|| ulid.to_string()));
}

fn unmarshal(c: &mut Criterion) {
    c.bench_function("unmarshal", |b| {
        b.iter(|| Ulid::unmarshal(black_box("01ARYZ6S410000000000000000")))
    });
}

fn timestamp(c: &mut Criterion) {
    let ulid = Ulid::new(Utc::now().timestamp() as u64, || rand::random());
    c.bench_function("timestamp", |b| b.iter(|| ulid.timestamp()));
}

criterion_group!(
    benches,
    new,
    new_systemtime_now,
    new_utc_now,
    new_rand_random,
    new_systemtime_now_rand_random,
    new_utc_now_rand_random,
    marshal,
    marshal_to_string,
    unmarshal,
    timestamp,
);
criterion_main!(benches);
