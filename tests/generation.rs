use veigo_id::{InMemoryBackend, VeigoConfig, VeigoIdGenerator};

// Import the common module
mod common;

#[test]
fn test_end_to_end_generation() {
    let backend = InMemoryBackend::new();
    let config = VeigoConfig::default();
    let node_id = 100;

    // 1. Initialize
    let generator =
        VeigoIdGenerator::new(config, backend, node_id).expect("Should create generator");

    // 2. Generate
    let context_id = 500;
    let id = generator.generate(context_id).expect("Should generate ID");

    // 3. Decode and Verify
    let parts = generator.decode(id);

    assert_eq!(parts.context, context_id);
    assert_eq!(parts.node_id, node_id);
    assert_eq!(parts.counter, 0); // First ID in this second

    println!("Generated ID: {:?}", id);
}

#[test]
fn test_sequence_increments() {
    let backend = InMemoryBackend::new();
    // Use helper: 10 bits for context, 10 for node, 10 for counter
    let config = common::create_test_config(10, 10, 10);

    let generator = VeigoIdGenerator::new(config, backend, 1).unwrap();
    let ctx = 50;

    // Generate 3 IDs
    let id1 = generator.generate(ctx).unwrap();
    let id2 = generator.generate(ctx).unwrap();
    let id3 = generator.generate(ctx).unwrap();

    let p1 = generator.decode(id1);
    let p2 = generator.decode(id2);
    let p3 = generator.decode(id3);

    assert_eq!(p1.counter, 0);
    assert_eq!(p2.counter, 1);
    assert_eq!(p3.counter, 2);
}
