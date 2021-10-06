//! Macros.

/// Generates the signer seeds for a [crate::Migrator].
#[macro_export]
macro_rules! gen_migrator_signer_seeds {
    ($migrator:expr) => {
        &[
            b"migrator".as_ref(),
            &$migrator.program_id.to_bytes(),
            &[$migrator.bump],
        ]
    };
}
