use veigo_id::{self, InMemoryBackend, VeigoId};

#[test]
fn test_generate_and_decode() {
    let backend = InMemoryBackend::new();
    veigo_id::configure(None, backend).unwrap();

    let context = 42u128;
    let id1 = VeigoId::new(context).unwrap();
    let parts1 = id1.decode().unwrap();

    assert_eq!(parts1.context, context);
    assert_eq!(parts1.counter, 0);

    let id2 = VeigoId::new(context).unwrap();
    let parts2 = id2.decode().unwrap();

    assert_eq!(parts2.context, context);
    assert_eq!(parts2.counter, 1);
    assert_eq!(
        parts2.timestamp, parts1.timestamp,
        "IDs should share the same timestamp if generated in the same second"
    );
}
