use blake3::Hash;

pub trait BlobStore {
    type Error;
    fn upload(&mut self, blob_id: &BlobId, data: &[u8]) -> Result<(), Self::Error>;
    fn download(&self, blob_id: &BlobId) -> Result<Vec<u8>, Self::Error>;
}

/// A BLAKE3 hash of a raw, encrypted data blob.
#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct BlobId(Hash);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataBlobMetadata {
    pub original_size: u64,
    // pub compressed_size: u64,
    // pub compression_algorithm: Option<String>,
}

/// If data does not fit into a single blob, it is split into multiple blobs
/// and referenced using a BlobManifest.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BlobManifest {
    pub parts: Vec<BlobId>,
    pub total_size: u64,
    pub chunk_size: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DataBlob {
    Single {
        blob: BlobId,
        metadata: DataBlobMetadata,
    },
    Chunked {
        manifest: BlobId,
        metadata: DataBlobMetadata,
    },
}

impl DataBlob {
    pub fn from_data<S: BlobStore>(store: &mut S, data: &[u8]) -> Result<DataBlob, S::Error> {
        // For simplicity, we will not implement chunking here.
        use blake3::Hasher;

        let mut hasher = Hasher::new();
        hasher.update(data);
        let hash = hasher.finalize();

        let blob_id = BlobId(hash);

        store.upload(&blob_id, data)?;

        let metadata = DataBlobMetadata {
            original_size: data.len() as u64,
        };

        Ok(DataBlob::Single {
            blob: blob_id,
            metadata,
        })
    }

    pub fn retrieve_data<S: BlobStore>(&self, store: &S) -> Result<Vec<u8>, S::Error> {
        match self {
            DataBlob::Single { blob, .. } => store.download(blob),
            DataBlob::Chunked { manifest, .. } => {
                // Chunked retrieval not implemented in this example.
                unimplemented!("Chunked blob retrieval is not implemented.");
            }
        }
    }
}
