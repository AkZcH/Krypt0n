use std::{
    alloc::{GlobalAlloc, Layout, System},
    fs::File,
    io::{Read, Write},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use krypton::{engine::encrypt, stream::encrypt::encrypt_stream};
use tempfile::tempdir;

struct CountingAllocator;

static CURRENT_ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static PEAK_ALLOCATED: AtomicUsize = AtomicUsize::new(0);

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            record_alloc(layout.size());
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        if !ptr.is_null() {
            CURRENT_ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
        if !new_ptr.is_null() {
            let old_size = layout.size();
            if new_size > old_size {
                record_alloc(new_size - old_size);
            } else {
                CURRENT_ALLOCATED.fetch_sub(old_size - new_size, Ordering::SeqCst);
            }
        }
        new_ptr
    }
}

fn record_alloc(size: usize) {
    let current = CURRENT_ALLOCATED.fetch_add(size, Ordering::SeqCst) + size;
    let mut peak = PEAK_ALLOCATED.load(Ordering::SeqCst);

    while current > peak {
        match PEAK_ALLOCATED.compare_exchange(peak, current, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => break,
            Err(observed) => peak = observed,
        }
    }
}

fn peak_heap_growth<F>(operation: F) -> usize
where
    F: FnOnce(),
{
    let baseline = CURRENT_ALLOCATED.load(Ordering::SeqCst);
    PEAK_ALLOCATED.store(baseline, Ordering::SeqCst);

    operation();

    PEAK_ALLOCATED
        .load(Ordering::SeqCst)
        .saturating_sub(baseline)
}

fn whole_file_encrypt(input_path: &Path, password: &[u8], aad: &[u8]) {
    let mut plaintext = Vec::new();
    File::open(input_path)
        .unwrap()
        .read_to_end(&mut plaintext)
        .unwrap();

    let encrypted = encrypt(password, &plaintext, aad).unwrap();
    black_box(encrypted.len());
}

fn chunked_stream_encrypt(input_path: &Path, password: &[u8], aad: &[u8]) {
    encrypt_stream(
        File::open(input_path).unwrap(),
        std::io::sink(),
        password,
        aad,
    )
    .unwrap();
}

fn write_bench_file(path: &Path, size: usize) {
    let mut file = File::create(path).unwrap();
    let block = vec![0xA5; 1024 * 1024];
    let mut remaining = size;

    while remaining > 0 {
        let to_write = remaining.min(block.len());
        file.write_all(&block[..to_write]).unwrap();
        remaining -= to_write;
    }
}

fn mib(bytes: usize) -> f64 {
    bytes as f64 / 1024.0 / 1024.0
}

fn bench_memory_profile(c: &mut Criterion) {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("memory-profile-input.bin");
    let file_size = 32 * 1024 * 1024;
    let password = b"memory-profile-password";
    let aad = b"krypton-memory-profile";

    write_bench_file(&input_path, file_size);

    let whole_peak = peak_heap_growth(|| whole_file_encrypt(&input_path, password, aad));
    let stream_peak = peak_heap_growth(|| chunked_stream_encrypt(&input_path, password, aad));

    eprintln!(
        "Approx peak heap growth for 32 MiB input: whole-file = {:.2} MiB, chunked-stream = {:.2} MiB",
        mib(whole_peak),
        mib(stream_peak)
    );

    let mut group = c.benchmark_group("whole file vs chunked stream");
    group.throughput(Throughput::Bytes(file_size as u64));
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(30);

    group.bench_with_input(
        BenchmarkId::new("whole file load then encrypt", "32MiB"),
        &input_path,
        |b, path| b.iter(|| whole_file_encrypt(path, password, aad)),
    );

    group.bench_with_input(
        BenchmarkId::new("chunked stream encrypt", "32MiB"),
        &input_path,
        |b, path| b.iter(|| chunked_stream_encrypt(path, password, aad)),
    );

    group.finish();
}

criterion_group!(benches, bench_memory_profile);
criterion_main!(benches);
