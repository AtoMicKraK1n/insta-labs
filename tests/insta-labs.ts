import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { InstaLabs } from "../target/types/insta_labs";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("insta-labs", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.local(); // Uses local validator or Devnet
  anchor.setProvider(provider);

  const program = anchor.workspace.InstaLabs as Program<InstaLabs>;

  const admin = provider.wallet;
  const testUPID = "TEST-12345";

  it("Should initialize a new patient record", async () => {
    // 3️⃣ Derive PDA for the patient account
    const [patientPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from("patient"), 
        Buffer.from(testUPID, "utf8") // ✅ Ensure UTF-8 encoding
      ],
      program.programId
    );;

    console.log(`📌 Patient PDA: ${patientPDA.toBase58()}`);
    // Add your test here.
    const tx = await program.methods.initializePatient(testUPID).accounts({
      patient_data: patientPDA,
      admin: admin.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();
    console.log("✅ Transaction Signature:s", tx);
  });
});
