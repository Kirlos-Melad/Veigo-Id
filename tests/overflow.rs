use veigo_id::{InMemoryBackend, VeigoIdError, VeigoIdGenerator};

mod common;

#[test]
fn test_context_overflow() {
    let backend = InMemoryBackend::new();
    // 4 bits for context = Max value 15
    let config = common::create_test_config(4, 10, 10);
    let generator = VeigoIdGenerator::new(config, backend, 1).unwrap();

    // 16 needs 5 bits (10000), so this should fail
    let err = generator.generate(16);

    assert!(matches!(
        err,
        Err(VeigoIdError::FieldOverflow {
            field: "context",
            ..
        })
    ));
}

#[test]
fn test_counter_exhaustion() {
    let backend = InMemoryBackend::new();
    // 2 bits for counter = Max value 3 (0, 1, 2, 3)
    let config = common::create_test_config(10, 10, 2);
    let generator = VeigoIdGenerator::new(config, backend, 1).unwrap();
    let ctx = 1;

    // Use up all 4 slots
    assert!(generator.generate(ctx).is_ok());
    assert!(generator.generate(ctx).is_ok());
    assert!(generator.generate(ctx).is_ok());
    assert!(generator.generate(ctx).is_ok());

    // The 5th generation must fail
    let err = generator.generate(ctx);
    assert!(matches!(
        err,
        Err(VeigoIdError::FieldOverflow {
            field: "counter",
            ..
        })
    ));
}

#[test]
fn test_node_id_startup_check() {
    let backend = InMemoryBackend::new();
    // 4 bits for node = Max 15
    let config = common::create_test_config(10, 4, 10);

    // Try to start with Node ID 20
    let result = VeigoIdGenerator::new(config, backend, 20);

    assert!(matches!(
        result,
        Err(VeigoIdError::FieldOverflow {
            field: "node_id initialization",
            ..
        })
    ));
}
