use anchor_lang::prelude::*;
use crate::state::{patient_data::PatientData, test_result::TestResult}; // Import patient data structures

#[derive(Accounts)]
pub struct GetTestResults<'info> {
    #[account(
        seeds = [b"patient", patient_data.upid.as_bytes().as_ref()], 
        bump
    )]
    pub patient_data: Account<'info, PatientData>, // On-chain storage for patient test records
}

pub fn get_test_results(ctx: Context<GetTestResults>) -> Result<Vec<TestResult>> {
    let patient_data = &ctx.accounts.patient_data;

    msg!("Fetching test results for UPID: {}", patient_data.upid);

    // Return all test results stored in the patient's account
    Ok(patient_data.tests.clone())
}
