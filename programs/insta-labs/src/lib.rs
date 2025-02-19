use anchor_lang::prelude::*;

declare_id!("9tNAK3cjM1HKEd2ijZZpwF4LN84Kbq1eKR3LhvRC8HD5");

pub mod contexts;
pub mod state;
pub mod error;

pub use contexts::*;
pub use state::*;
pub use error::*;

#[program]
pub mod insta_labs {
    use super::*;

    // ✅ Initialize a Patient Record (Admin Only)
    pub fn initialize_patient(ctx: Context<InitializePatient>, upid: String) -> Result<()> {
        initialize_patient::initialize_patient(ctx, upid)
    }

    // ✅ Store Lab Test Results (Admin Only)
    pub fn store_test_results(
        ctx: Context<StoreTestResults>, 
        test_id: String, 
        test_type: String, 
        result: String, 
        notes: Option<String>
    ) -> Result<()> {
        store_test_results::store_test_results(ctx, test_id, test_type, result, notes)
    }

    // ✅ Fetch Patient Test Results Using UPID
    pub fn get_test_results(ctx: Context<GetTestResults>) -> Result<Vec<TestResult>> {
        get_test_results::get_test_results(ctx)
    }
}


