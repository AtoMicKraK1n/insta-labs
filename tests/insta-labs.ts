import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { InstaLabs } from "../target/types/insta_labs";
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

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

   function scaleUp(value: number): number {
    return Math.round(value * 10); // Multiply by 100 to store as integer
  }

  function scaleDown(value: number): number {
    return value / 10; // Convert back to original float
  }

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
    const haemoglobin = scaleUp(13.5);
    const rbcCount = scaleUp(4.8);
    const wbcCount = 6700;
    const plateletCount = 250000;
    const mcv = scaleUp(90.0);
    const mch = scaleUp(30.5);
    const mchc = scaleUp(34.0);

    // 7️⃣ Send transaction to store test results
    const tx = await program.methods
      .storeTestResults(
        testID,
        timestamp,
        haemoglobin,
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

  it("✅ Should retrieve test results for the patient", async () => {
    const testResults = await program.account.patientData.fetch(patientPDA);

    console.log("Retrieved Tests Results:", testResults);

    const retrievedHaemoglobin = scaleDown(testResults.tests[0].haemoglobin / 10);
    const retrievedRbcCount = scaleDown(testResults.tests[0].rbcCount / 10);
    const retrievedMcv = scaleDown(testResults.tests[0].mcv / 10);
    const retrievedMch = scaleDown(testResults.tests[0].mch / 10);
    const retrievedMchc = scaleDown(testResults.tests[0].mchc / 10);

    expect(testResults.tests.length).to.equal(1); // Ensure one test is retrieved
    expect(testResults.tests[0].testId).to.equal("BLOOD-001");
    expect(retrievedHaemoglobin).to.equal(13.5);
    expect(retrievedRbcCount).to.equal(4.8);
    expect(testResults.tests[0].wbcCount).to.equal(6700);
    expect(testResults.tests[0].plateletCount).to.equal(250000);
    expect(retrievedMcv).to.equal(90.0);
    expect(retrievedMch).to.equal(30.5);
    expect(retrievedMchc).to.equal(34.0);
    
  })
});
