use std::sync::Arc;
use veigo_id::{VeigoConfig, VeigoIdGenerator, backend::memory_backend::MemoryState};

#[test]
fn test_generate_and_decode() {
    let backend = Arc::new(MemoryState::new());
    let vgen = VeigoIdGenerator::new(None, backend).unwrap();

    let context = 42u128;
    let id = vgen.generate(context).unwrap();
    let parts = vgen.decode(id);

    assert_eq!(parts.context, context);
    assert_eq!(parts.counter, 0);

    let id2 = vgen.generate(context).unwrap();
    let parts2 = vgen.decode(id2);

    assert_eq!(parts2.context, context);
    assert_eq!(parts2.counter, 1);
    assert_eq!(parts2.timestamp, parts.timestamp);
}

#[test]
fn test_context_overflow() {
    let backend = Arc::new(MemoryState::new());
    let mut config = VeigoConfig::default();
    config.context_bits = 2; // max context = 3
    let vgen = VeigoIdGenerator::new(Some(config), backend).unwrap();

    let context = 10u128;
    let err = vgen.generate(context).unwrap_err().to_string();
    assert!(err.contains("field overflow"));
}
