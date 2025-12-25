use blake3::Hash;

pub trait BlobStore {
    type Error;
    fn upload(&mut self, blob_id: &BlobId, data: &[u8]) -> Result<(), Self::Error>;
    fn download(&self, blob_id: &BlobId) -> Result<Vec<u8>, Self::Error>;
}

/// A BLAKE3 hash of a raw, encrypted data blob.
#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct BlobId(pub Hash);

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

const CHUNK_SIZE: usize = 32 * 1024 * 1024; // 32 MiB
// const CHUNK_SIZE: usize = 2; // 2 bytes for testing

impl DataBlob {
    pub fn from_data<S: BlobStore>(store: &mut S, data: &[u8]) -> Result<DataBlob, BlobError>
        where <S as BlobStore>::Error: std::fmt::Debug
    {
        let size = data.len();

        // todo: rolling checksum chunking for large data blobs
        if size > CHUNK_SIZE {
            let (chunks, remainder) = data.as_chunks::<CHUNK_SIZE>();
            let mut chunk_ids: Vec<BlobId> = Vec::new();

            for chunk in chunks {
                let hash = blake3::hash(chunk);
                let blob_id = BlobId(hash);
                store
                    .upload(&blob_id, chunk)
                    .map_err(|e| BlobError::StoreError(format!("{:?}", e)))?;
                chunk_ids.push(blob_id);
            }

            if !remainder.is_empty() {
                let hash = blake3::hash(remainder);
                let blob_id = BlobId(hash);
                store
                    .upload(&blob_id, remainder)
                    .map_err(|e| BlobError::StoreError(format!("{:?}", e)))?;
                chunk_ids.push(blob_id);
            }

            let manifest = BlobManifest {
                parts: chunk_ids,
                chunk_size: CHUNK_SIZE as u32,
            };

            let manifest_data = serde_json::to_vec(&manifest).unwrap();
            let manifest_hash = blake3::hash(&manifest_data);
            let manifest_blob_id = BlobId(manifest_hash);
            store
                .upload(&manifest_blob_id, &manifest_data)
                .map_err(|e| BlobError::StoreError(format!("{:?}", e)))?;

            let metadata = DataBlobMetadata {
                original_size: data.len() as u64,
            };

            return Ok(DataBlob::Chunked {
                manifest: manifest_blob_id,
                metadata,
            });
        }

        let hash = blake3::hash(data);

        let blob_id = BlobId(hash);

        store.upload(&blob_id, data).map_err(|e| BlobError::StoreError(format!("{:?}", e)))?;

        let metadata = DataBlobMetadata {
            original_size: data.len() as u64,
        };

        Ok(DataBlob::Single {
            blob: blob_id,
            metadata,
        })
    }

    pub fn retrieve_data<S: BlobStore>(&self, store: &S) -> Result<Vec<u8>, BlobError>
        where <S as BlobStore>::Error: std::fmt::Debug
    {
        match self {
            DataBlob::Single { blob, .. } => {
                let data = store
                    .download(blob)
                    .map_err(|e| BlobError::StoreError(format!("{:?}", e)))?;

                let hash = blake3::hash(&data);
                if &BlobId(hash) != blob {
                    return Err(BlobError::IntegrityCheckFailed);
                }
                Ok(data)
            }
            DataBlob::Chunked { manifest, .. } => {
                let manifest_data = store
                    .download(manifest)
                    .map_err(|e| BlobError::StoreError(format!("{:?}", e)))?;

                let hash = blake3::hash(&manifest_data);
                if &BlobId(hash) != manifest {
                    return Err(BlobError::IntegrityCheckFailed);
                }

                let manifest: BlobManifest = serde_json
                    ::from_slice(&manifest_data)
                    .map_err(|_| BlobError::IntegrityCheckFailed)?;

                let mut result: Vec<u8> = Vec::new();

                for part_id in manifest.parts {
                    let chunk_data = store
                        .download(&part_id)
                        .map_err(|e| BlobError::StoreError(format!("{:?}", e)))?;

                    let hash = blake3::hash(&chunk_data);
                    if &BlobId(hash) != &part_id {
                        return Err(BlobError::IntegrityCheckFailed);
                    }

                    result.extend_from_slice(&chunk_data);
                }

                Ok(result)
            }
        }
    }
}

#[derive(Debug)]
pub enum BlobError {
    NotFound,
    IntegrityCheckFailed,
    StoreError(String),
}
