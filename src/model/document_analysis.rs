
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentAnalysis {
    pub authenticity: String,
    pub extracted_data: Option<PhysicalDocumentExtractedDataAnalysis>,
    pub image_quality: String,
}
impl std::fmt::Display for DocumentAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}