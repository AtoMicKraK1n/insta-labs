use anchor_lang::prelude::*;
use crate::state::{patient_data::PatientData, test_result::TestResult};
use crate::error::Errors; // Importing state structures

#[derive(Accounts)]
pub struct StoreTestResults<'info> {
    #[account(
        mut, 
        seeds = [b"patient", patient_data.upid.as_bytes().as_ref()], 
        bump
    )]
    pub patient_data: Account<'info, PatientData>, // Patient's on-chain record
    #[account(mut)]
    pub admin: Signer<'info>, // Only the admin can store test results
}

pub fn store_test_results(
    ctx: Context<StoreTestResults>, 
    test_id: String, 
    test_type: String, 
    result: String, 
    notes: Option<String>
) -> Result<()> {
    let patient_data = &mut ctx.accounts.patient_data;
    
    // Ensure only the admin can add test results
    require!(ctx.accounts.admin.key() == patient_data.admin, Errors::UnauthorizedAccess);

    // Append the new test result to the patient's record
    patient_data.tests.push(TestResult {
        test_id,
        test_type,
        result,
        notes,
        timestamp: Clock::get()?.unix_timestamp, // Store current timestamp
    });

    msg!("Test result added for UPID: {}", patient_data.upid);
    Ok(())
}


