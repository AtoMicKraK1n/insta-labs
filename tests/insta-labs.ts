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
    const testID = "NAME-001";
    const test_type = "URINE";
    const path_lab_name = "Dr Lal Path Labs";
    const timestamp = new anchor.BN(Date.now());
    const haemoglobin = scaleUp(13.5);
    const rbcCount = scaleUp(4.8);
    const wbcCount = 6700;
    const plateletCount = 250000;
    const mcv = scaleUp(90.0);
    const mch = scaleUp(30.5);
    const mchc = scaleUp(34.0);
    const rdw = scaleUp(12.5);
    const neutrophils = scaleUp(55.0);
    const lymphocytes = scaleUp(35.0);
    const monocytes = scaleUp(7.0);
    const eosinophils = scaleUp(3.0);
    const basophils = scaleUp(0.5);

    const phLevel = scaleUp(5.5);
    const protein = scaleUp(0.2);
    const glucose = scaleUp(0.1);
    

    // 7️⃣ Send transaction to store test results
    const tx = await program.methods
      .storeTestResults(
        testID,
        test_type,
        path_lab_name,
        timestamp,
        haemoglobin,
        rbcCount,
        wbcCount,
        plateletCount,
        mcv,
        mch,
        mchc,
        rdw,
        neutrophils,
        lymphocytes,
        monocytes,
        eosinophils,
        basophils,

        phLevel,
        protein,
        glucose
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
    const rerievedRdw = scaleDown(testResults.tests[0].rdw / 10);
    const retrievedNeutrophils = scaleDown(testResults.tests[0].neutrophils / 10);
    const retrievedLymphocytes = scaleDown(testResults.tests[0].lymphocytes / 10);
    const retrievedMonocytes = scaleDown(testResults.tests[0].monocytes / 10);
    const retrievedEosinophils = scaleDown(testResults.tests[0].eosinophils / 10);
    const retrievedBasophils = scaleDown(testResults.tests[0].basophils / 10);

    const retrievedPhLevel = scaleDown(testResults.tests[0].phLevel / 10);
    const retrievedProtein = scaleDown(testResults.tests[0].protein / 10);
    const retrievedGlucose = scaleDown(testResults.tests[0].glucose / 10);

    expect(testResults.tests.length).to.equal(1);
    expect(testResults.tests[0].testId).to.equal("NAME-001");
    expect(testResults.tests[0].testType).to.equal("URINE");
    expect(testResults.tests[0].pathLabName).to.equal("Dr Lal Path Labs");
    expect(retrievedHaemoglobin).to.equal(13.5);
    expect(retrievedRbcCount).to.equal(4.8);
    expect(testResults.tests[0].wbcCount).to.equal(6700);
    expect(testResults.tests[0].plateletCount).to.equal(250000);
    expect(retrievedMcv).to.equal(90.0);
    expect(retrievedMch).to.equal(30.5);
    expect(retrievedMchc).to.equal(34.0);   
    expect(rerievedRdw).to.equal(12.5);
    expect(retrievedNeutrophils).to.equal(55.0);
    expect(retrievedLymphocytes).to.equal(35.0);
    expect(retrievedMonocytes).to.equal(7.0);
    expect(retrievedEosinophils).to.equal(3.0);
    expect(retrievedBasophils).to.equal(0.5);

    expect(retrievedPhLevel).to.equal(5.5);
    expect(retrievedProtein).to.equal(0.2);
    expect(retrievedGlucose).to.equal(0.1);
  })
});
