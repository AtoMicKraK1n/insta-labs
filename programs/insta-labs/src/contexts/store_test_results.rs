use anchor_lang::prelude::*;
use crate::state::{patient_data::PatientData, test_result::TestResult};
use crate::error::Errors; 

#[derive(Accounts)]
pub struct StoreTestResults<'info> {
    #[account(
        mut, 
        seeds = [b"patient", patient_data.upid.as_bytes()],   
        bump
    )]
    pub patient_data: Account<'info, PatientData>, 
    #[account(mut)]
    pub upid: Signer<'info>,// Patient's on-chain record
    #[account(mut)]
    pub admin: Signer<'info>, // Only the admin can store test results
    pub system_program: Program<'info, System>,
}


pub fn store_test_results(
    ctx: Context<StoreTestResults>,
    test_id: String,  
    _timestamp: i64,
    haemoglobin: Option<f32>,
    rbc_count: Option<f32>,
    wbc_count: Option<f32>,
    platelet_count: Option<f32>,
    mcv: Option<f32>,
    mch: Option<f32>,
    mchc: Option<f32>,

) -> Result<()> {
    let patient_data = &mut ctx.accounts.patient_data;

    require!(ctx.accounts.admin.key() == patient_data.admin, Errors::UnauthorizedAccess);

    const MAX_TESTS: usize = 9;

    if patient_data.tests.len() >= MAX_TESTS {
        patient_data.tests.remove(0); // ✅ Remove the oldest test to make space
    }

    patient_data.tests.push(TestResult {
        test_id,
        timestamp: Clock::get()?.unix_timestamp,
        haemoglobin,
        rbc_count,
        wbc_count,
        platelet_count,
        mcv,
        mch,
        mchc,

    });

    msg!("Blood test result added for UPID: {}", patient_data.upid);
    Ok(())
}
