use anchor_lang::prelude::*;
use crate::state::PatientData;

#[derive(Accounts)]
pub struct InitializePatient<'info> {
    #[account(mut)]
    pub upid: Signer<'info>,
    #[account(
        init, 
        payer = admin,
        space = 2048,
        seeds = [b"patient", upid.key().to_bytes().as_ref()],
        bump,
    )]
    pub patient_data: Account<'info, PatientData>, // On-chain storage for patient
    #[account(mut)]
    pub admin: Signer<'info>, // Only the admin (program deployer) can create patient records
    pub system_program: Program<'info, System>, // Solana system program
}

pub fn initialize_patient(ctx: Context<InitializePatient>, upid: String) -> Result<()> {
    let patient_data = &mut ctx.accounts.patient_data;
    patient_data.upid = upid;
    patient_data.admin = ctx.accounts.admin.key();
    patient_data.tests = Vec::new(); // Initialize an empty test record list

    msg!("Patient record created with UPID: {}", patient_data.upid);
    Ok(())
}
