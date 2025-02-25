use anchor_lang::{prelude::*, Bump};
use crate::state::PatientData;

#[derive(Accounts)]
#[instruction(upid: String)]
pub struct InitializePatient<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, 
    #[account(
        init, 
        payer = admin,
        space = 2048, 
        seeds = [b"patient", upid.as_bytes().as_ref()],
        bump
    )]
    pub patient_data: Account<'info, PatientData>,  
    pub system_program: Program<'info, System>,
}


pub fn initialize_patient(ctx: Context<InitializePatient>, upid: String) -> Result<()> {
    let patient_data = &mut ctx.accounts.patient_data;
    patient_data.bump = ctx.bumps.patient_data;
    patient_data.upid = upid;
    patient_data.admin = ctx.accounts.admin.key();
    patient_data.tests = Vec::new(); // Initialize an empty test record list

    msg!("Patient record created with UPID: {}", patient_data.upid);
    Ok(())
}
