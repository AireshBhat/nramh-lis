pub mod analyzer;
pub mod patient;
pub mod result;
pub mod sample;
pub mod test_order;
pub mod upload;

pub use analyzer::{Analyzer, AnalyzerStatus, ConnectionType, Protocol};
pub use patient::Patient;
pub use result::{ResultStatus, TestResult};
pub use sample::{Sample, SampleStatus};
pub use test_order::TestOrder;
pub use upload::{ResultUploadStatus, UploadStatus};
