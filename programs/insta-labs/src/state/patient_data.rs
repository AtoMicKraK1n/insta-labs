use anchor_lang::prelude::*;
use crate::state::test_result::TestResult;

#[account]
pub struct PatientData {
    pub upid: String,             // Unique Patient ID
    pub admin: Pubkey,            // Admin who created this patient record
    pub tests: Vec<TestResult>,   // List of stored lab test results
}
