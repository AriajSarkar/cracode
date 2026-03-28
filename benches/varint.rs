use cracode::config;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::distr::Distribution;

fn slice_varint_u8(c: &mut Criterion) {
    let mut rng = rand::rng();
    let dist = rand::distr::Uniform::new(0, u8::MAX).unwrap();
    let input: Vec<u8> = std::iter::from_fn(|| Some(dist.sample(&mut rng))).take(10_000).collect();
    let config = config::standard();
    let bytes = cracode::encode_to_vec(input, config).unwrap();

    c.bench_function("slice_varint_u8", |b| {
        b.iter(|| {
            let _: (Vec<u8>, usize) = cracode::decode_from_slice(&bytes, config).unwrap();
        })
    });
}

fn slice_varint_u16(c: &mut Criterion) {
    let mut rng = rand::rng();
    let dist = rand::distr::Uniform::new(0, u16::MAX).unwrap();
    let input: Vec<u16> = std::iter::from_fn(|| Some(dist.sample(&mut rng))).take(10_000).collect();
    let config = config::standard();
    let bytes = cracode::encode_to_vec(input, config).unwrap();

    c.bench_function("slice_varint_u16", |b| {
        b.iter(|| {
            let _: (Vec<u16>, usize) = cracode::decode_from_slice(&bytes, config).unwrap();
        })
    });
}

fn slice_varint_u32(c: &mut Criterion) {
    let mut rng = rand::rng();
    let dist = rand::distr::Uniform::new(0, u32::MAX).unwrap();
    let input: Vec<u32> = std::iter::from_fn(|| Some(dist.sample(&mut rng))).take(10_000).collect();
    let config = config::standard();
    let bytes = cracode::encode_to_vec(input, config).unwrap();

    c.bench_function("slice_varint_u32", |b| {
        b.iter(|| {
            let _: (Vec<u32>, usize) = cracode::decode_from_slice(&bytes, config).unwrap();
        })
    });
}

fn slice_varint_u64(c: &mut Criterion) {
    let mut rng = rand::rng();
    let dist = rand::distr::Uniform::new(0, u64::MAX).unwrap();
    let input: Vec<u64> = std::iter::from_fn(|| Some(dist.sample(&mut rng))).take(10_000).collect();
    let config = config::standard();
    let bytes = cracode::encode_to_vec(input, config).unwrap();

    c.bench_function("slice_varint_u64", |b| {
        b.iter(|| {
            let _: (Vec<u64>, usize) = cracode::decode_from_slice(&bytes, config).unwrap();
        })
    });
}

fn bufreader_varint_u8(c: &mut Criterion) {
    let mut rng = rand::rng();
    let dist = rand::distr::Uniform::new(0, u8::MAX).unwrap();
    let input: Vec<u8> = std::iter::from_fn(|| Some(dist.sample(&mut rng))).take(10_000).collect();
    let config = config::standard();
    let bytes = cracode::encode_to_vec(input, config).unwrap();

    c.bench_function("bufreader_varint_u8", |b| {
        b.iter(|| {
            let _: Vec<u8> =
                cracode::decode_from_reader(&mut std::io::BufReader::new(&bytes[..]), config)
                    .unwrap();
        })
    });
}

fn bufreader_varint_u16(c: &mut Criterion) {
    let mut rng = rand::rng();
    let dist = rand::distr::Uniform::new(0, u16::MAX).unwrap();
    let input: Vec<u16> = std::iter::from_fn(|| Some(dist.sample(&mut rng))).take(10_000).collect();
    let config = config::standard();
    let bytes = cracode::encode_to_vec(input, config).unwrap();

    c.bench_function("bufreader_varint_u16", |b| {
        b.iter(|| {
            let _: Vec<u16> =
                cracode::decode_from_reader(&mut std::io::BufReader::new(&bytes[..]), config)
                    .unwrap();
        })
    });
}

fn bufreader_varint_u32(c: &mut Criterion) {
    let mut rng = rand::rng();
    let dist = rand::distr::Uniform::new(0, u32::MAX).unwrap();
    let input: Vec<u32> = std::iter::from_fn(|| Some(dist.sample(&mut rng))).take(10_000).collect();
    let config = config::standard();
    let bytes = cracode::encode_to_vec(input, config).unwrap();

    c.bench_function("bufreader_varint_u32", |b| {
        b.iter(|| {
            let _: Vec<u32> =
                cracode::decode_from_reader(&mut std::io::BufReader::new(&bytes[..]), config)
                    .unwrap();
        })
    });
}

fn bufreader_varint_u64(c: &mut Criterion) {
    let mut rng = rand::rng();
    let dist = rand::distr::Uniform::new(0, u64::MAX).unwrap();
    let input: Vec<u64> = std::iter::from_fn(|| Some(dist.sample(&mut rng))).take(10_000).collect();
    let config = config::standard();
    let bytes = cracode::encode_to_vec(input, config).unwrap();

    c.bench_function("bufreader_varint_u64", |b| {
        b.iter(|| {
            let _: Vec<u64> =
                cracode::decode_from_reader(&mut std::io::BufReader::new(&bytes[..]), config)
                    .unwrap();
        })
    });
}

criterion_group!(
    benches,
    slice_varint_u8,
    slice_varint_u16,
    slice_varint_u32,
    slice_varint_u64,
    bufreader_varint_u8,
    bufreader_varint_u16,
    bufreader_varint_u32,
    bufreader_varint_u64,
);
criterion_main!(benches);
