/// A SHA-256 hash of a raw, encrypted data blob.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, PartialOrd, Ord,
)]
pub struct BlobId(String);

/// Data blob representing raw data without metadata. Imutable.
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash, PartialOrd, Ord,
)]
pub struct Blob {
    /// The immutable hash of the blob.
    id: BlobId,

    /// Size of the blob in bytes.
    size: u64,
}

impl Blob {
    pub fn new(id: BlobId, size: u64) -> Self {
        Self { id, size }
    }

    /// Gets the ID of the blob.
    pub fn get_id(&self) -> &BlobId {
        &self.id
    }

    /// Gets the size of the blob in bytes.
    pub fn get_size(&self) -> u64 {
        self.size
    }
}
