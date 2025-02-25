use anchor_lang::prelude::*;
use crate::state::test_result::TestResult;

#[account]
pub struct PatientData {
    pub upid: String,
    pub bump: u8,           // Unique Patient ID
    pub admin: Pubkey,            // Admin who created this patient record
    pub tests: Vec<TestResult>,   // List of stored lab test results
}

impl PatientData {
    pub fn space(max_tests: usize) -> usize {
        8  // Discriminator
        + 4 + 64  // UPID (max 64 bytes + 4 for string length)
        + 32  // Admin Pubkey
        + 4 + (TestResult::size() * max_tests) // Vec<TestResult> (4 bytes for length + each test entry)
    }
}
