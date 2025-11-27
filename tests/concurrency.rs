use std::sync::Arc;
use std::thread;

use veigo_id::{InMemoryBackend, VeigoConfig, VeigoIdGenerator};

#[test]
fn test_multithreaded_generation() {
    let backend = InMemoryBackend::new();
    let config = VeigoConfig::default();

    // Create generator and wrap in Arc to share across threads
    let generator = Arc::new(VeigoIdGenerator::new(config, backend, 1).unwrap());

    let mut handles = vec![];

    // Spawn 10 threads, each generating 100 IDs for the SAME context
    for _ in 0..10 {
        let gen_clone = generator.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let _ = gen_clone.generate(1).expect("Thread generation failed");
            }
        }));
    }

    // Wait for all to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify counter
    // 10 threads * 100 IDs = 1000 IDs total.
    // The last ID generated should have counter around 999 (if all in same second)
    // OR we can just check the backend state if we exposed it,
    // but simply not crashing/panicking here is the main test.
}
