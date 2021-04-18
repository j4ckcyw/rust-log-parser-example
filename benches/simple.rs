use bytes::Bytes;
use log::debug;
use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use std::fs::File;
use std::io::Read;
use rust_logs_regex::parser::Parser;

fn read_testinput(file: &str) -> Bytes {
    let file = File::open(format!("{}{}", "./testinput/", file)).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    debug!("{}", buf);
    Bytes::from(buf)
}

fn parser_benchmark(c: &mut Criterion) {
    let _ = env_logger::builder().is_test(true).try_init();
    let mut group = c.benchmark_group("parser_benchmark");
    for file in ["small.log", "medium.log", "large.log"].iter() {
        debug!("Parsing {}", &file);
        let bytes = read_testinput(file);
        let parser = Parser::new("\\[(?P<timestamp>([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\\.[0-9]+)?(([Zz])|([\\+|\\-]([01][0-9]|2[0-3]):[0-5][0-9]))) (?P<level>\\w+)\\s+(?P<class>[::\\w]+)\\] (?P<content>.*)");
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(&file), &bytes, |b, bytes| {
            b.iter(|| {
                let events = parser.parse(bytes.clone());
                debug!("{:#?}", events);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, parser_benchmark);
criterion_main!(benches);