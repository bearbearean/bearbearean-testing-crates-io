//! # bearbearean-testing-crates-io

/// Example struct
pub struct Example {
    /// A field of the struct
    pub field: i64,
    /// A private field of the struct
    private_field: i64,
}

impl Example {
    /// Create a new [`Example`]
    pub fn new(field: i64) -> Self {
        Self {
            field,
            private_field: field * 2,
        }
    }

    /// Double the [`Example::field`]
    pub fn double(self) -> Self {
        Self {
            field: self.field * 2,
            private_field: self.private_field * 2,
        }
    }

    /// Triple the [`Example::field`]
    pub fn triple(self) -> Self {
        Self {
            field: self.field * 3,
            private_field: self.private_field * 3,
        }
    }
}

/// Test `Example` functionality
#[test]
fn test_example() {
    let example = Example::new(1);
    assert_eq!(example.field, 1);
    assert_eq!(example.private_field, 2);

    let doubled = example.double();
    assert_eq!(doubled.field, 2);
    assert_eq!(doubled.private_field, 4);

    let tripled = doubled.triple();
    assert_eq!(tripled.field, 6);
    assert_eq!(tripled.private_field, 12);
}
