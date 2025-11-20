pub mod common {
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/common.v2.rs"));
    }
}

pub mod errordetails {
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/errordetails.v2.rs"));
    }
}

pub mod dfdaemon {
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/dfdaemon.v2.rs"));
    }
}

pub mod manager {
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/manager.v2.rs"));
    }
}

pub mod scheduler {
    pub mod v2 {
        include!(concat!(env!("OUT_DIR"), "/scheduler.v2.rs"));
    }
}

// FILE_DESCRIPTOR_SET is the serialized FileDescriptorSet of the proto files.
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/descriptor.bin"));
