use criterion::{criterion_group, criterion_main, Criterion};
use cloxers::chunk::Chunk;
use cloxers::opcodes::OpCode;
use cloxers::vm::VM;
use cloxers::value::Value;


fn run_arithmetic() {
    let mut chunk = Chunk::new();
    let _ = chunk.write_constant(Value::Number(1.2), 1);
    let _ = chunk.write_constant(Value::Number(3.4), 1);
    let _ = chunk.write(OpCode::Add.into(), 1);
    let _ = chunk.write_constant(Value::Number(5.6), 2);
    let _ = chunk.write(OpCode::Divide.into(), 4);
    chunk.write(OpCode::Return.into(),2);
    VM::new(&chunk).run().unwrap();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("arithmetic 20", |b| b.iter(|| run_arithmetic()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);