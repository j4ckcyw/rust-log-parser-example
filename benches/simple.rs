use bytes::Bytes;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use log::debug;
use rust_logs_regex::parser::Parser;
use std::fs::File;
use std::io::Read;

fn read_testinput(file: &str) -> Bytes {
    let file = File::open(format!("{}{}", "./testinput/", file)).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    Bytes::from(buf)
}

fn parser_benchmark(c: &mut Criterion) {
    let _ = env_logger::builder().is_test(true).try_init();
    let mut group = c.benchmark_group("parser_benchmark");
    let parser =
        Parser::new(r"\[(?P<timestamp>\S+)\s+(?P<level>\S+)\s+(?P<class>\S+)]\s+(?P<content>.*)");
    for file in ["small.log", "medium.log", "large.log"].iter() {
        let bytes = read_testinput(file);
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(&file), &bytes, |b, bytes| {
            b.iter(|| {
                let _events = parser.parse(bytes.clone());
            });
        });
    }
    group.finish();
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);
