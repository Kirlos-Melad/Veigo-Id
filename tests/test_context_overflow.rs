use veigo_id::{self, InMemoryBackend, VeigoConfig, VeigoId, VeigoIdError};

#[test]
fn test_context_overflow() {
    let backend = InMemoryBackend::new();
    let mut config = VeigoConfig::default();
    config.context_bits = 2; // max context = 3
    veigo_id::configure(Some(config), backend).unwrap();

    let context = 10u128; // too large for 2 bits
    let err = VeigoId::new(context).unwrap_err();

    match err {
        VeigoIdError::FieldOverflow { field, value, max } => {
            assert_eq!(field, "context");
            assert_eq!(value, 10);
            assert_eq!(max, 3);
        }
        other => panic!("expected FieldOverflow, got {:?}", other),
    }
}
