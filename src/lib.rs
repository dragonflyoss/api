pub mod common;
pub mod dfdaemon;
pub mod manager;
pub mod scheduler;
pub mod security;

// FILE_DESCRIPTOR_SET is the serialized FileDescriptorSet of the proto files.
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("descriptor.bin");
