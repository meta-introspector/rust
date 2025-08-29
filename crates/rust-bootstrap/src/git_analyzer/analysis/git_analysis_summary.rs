#[derive(Debug)]
pub struct GitAnalysisSummary {
    pub commits_count: usize,
    pub blobs_count: usize,
    pub trees_count: usize,
    pub tags_count: usize,
    pub refs_count: usize,
    pub commits_parquet_path: String,
    pub blobs_parquet_path: String,
    pub trees_parquet_path: String,
    pub tags_parquet_path: String,
    pub refs_parquet_path: String,
}
