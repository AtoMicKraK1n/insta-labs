import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { InstaLabs } from "../target/types/insta_labs";
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("insta-labs", async () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.local(); // Uses local validator or Devnet
  anchor.setProvider(provider);
  
  const program = anchor.workspace.InstaLabs as Program<InstaLabs>;

  const UPID = "TEST-12345"; //Unique-Patient ID

  const [patientPDA] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("patient"), 
      Buffer.from(UPID) // ✅ Ensure UTF-8 encoding
    ],
    program.programId
  );

  const admin = Keypair.generate();

  before(async () => {

  await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(admin.publicKey, 10 * LAMPORTS_PER_SOL)
  );

});
  
it("Should initialize a new patient record", async () => {
    // 3️⃣ Derive PDA for the patient account
    
  try {
    console.log(`📌 Patient PDA: ${patientPDA.toBase58()}`);
    // Add your test here.
    const tx = await program.methods.initializePatient(UPID).accountsStrict(
      {
        admin: admin.publicKey,
        patientData: patientPDA,
        systemProgram: SystemProgram.programId,
      }
    
    )
    .signers([admin])
    .rpc();
    console.log("✅ Transaction Signature", tx);

  }
    catch (error) {
      console.error ("❌ Error:", error);
    }
      
  });


  it("✅ Should store test results for the patient", async () => {

    // Define test data (multiple blood parameters)
    const testID = "BLOOD-001";
    const timestamp = new anchor.BN(Date.now());
    const hemoglobin = 13.5;
    const rbcCount = 4.8;
    const wbcCount = 6700;
    const plateletCount = 250000;
    const mcv = 90.0;
    const mch = 30.5;
    const mchc = 34.0;

    // 7️⃣ Send transaction to store test results
    const tx = await program.methods
      .storeTestResults(
        testID,
        timestamp,
        hemoglobin,
        rbcCount,
        wbcCount,
        plateletCount,
        mcv,
        mch,
        mchc
      )
      .accountsStrict({
        admin: admin.publicKey,
        patientData: patientPDA,
        systemProgram: SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    console.log("✅ Test Results Stored - Transaction Signature:", tx);
  });
});
