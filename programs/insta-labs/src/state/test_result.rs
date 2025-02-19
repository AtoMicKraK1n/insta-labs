use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TestResult {
    pub test_id: String,       // Unique Test ID assigned for the lab test
    pub test_type: String,     // Type of test (e.g., Blood, Urine, Imaging)
    pub result: String,        // The actual test result value
    pub notes: Option<String>, // Additional notes from the test (optional)
    pub timestamp: i64,        // Timestamp when the test was recorded
}
