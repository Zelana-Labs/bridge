const {
  Connection,
  Keypair,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
  TransactionInstruction,
  LAMPORTS_PER_SOL,
} = require("@solana/web3.js");
const fs = require("fs");

const PROGRAM_ID = new PublicKey("95sWqtU9fdm19cvQYu94iKijRuYAv3wLqod1pcsSfYth");

(async () => {
  try {
    const connection = new Connection("http://127.0.0.1:8899", "confirmed");
    const walletPath = "/home/shanks/.config/solana/id.json";
    const secretKey = JSON.parse(fs.readFileSync(walletPath, "utf-8"));
    const payer = Keypair.fromSecretKey(new Uint8Array(secretKey));

    const sequencerSeed = Uint8Array.from([
        1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8,
        1,2,3,4,5,6,7,8,1,2,3,4,5,6,7,8
    ]);
    const sequencer = Keypair.fromSeed(sequencerSeed);
    
    console.log(" Connected to local validator.");
    console.log(" Payer:", payer.publicKey.toBase58());
    console.log(" Sequencer (Deterministic):", sequencer.publicKey.toBase58());

    const [configPda] = PublicKey.findProgramAddressSync([Buffer.from("config")], PROGRAM_ID);
    const [vaultPda] = PublicKey.findProgramAddressSync([Buffer.from("vault"), configPda.toBuffer()], PROGRAM_ID);

    // =================================================================
    // 2. CHECK & INITIALIZE
    // =================================================================
    await initializeBridgeIfNeeded(connection, payer, sequencer, configPda, vaultPda);
    await logVaultBalance(connection, vaultPda);

    // =================================================================
    // 3. MAKE 10 DEPOSITS
    // =================================================================
    const depositsToMake = 10;
    for (let i = 0; i < depositsToMake; i++) {
        console.log(`\n--- DEPOSIT ${i + 1} of ${depositsToMake} ---`);
        const depositor = Keypair.generate();
        await fundAccount(connection, depositor.publicKey, 2 * LAMPORTS_PER_SOL);
        const depositAmount = 0.1 * LAMPORTS_PER_SOL;
        await makeDeposit(connection, depositor, configPda, vaultPda, depositAmount);
        await logVaultBalance(connection, vaultPda);
    }

    // =================================================================
    // 4. MAKE 5 WITHDRAWALS
    // =================================================================
    const withdrawalsToMake = 5;
    for (let i = 0; i < withdrawalsToMake; i++) {
        console.log(`\n--- WITHDRAWAL ${i + 1} of ${withdrawalsToMake} ---`);
        const withdrawAmount = 0.05 * LAMPORTS_PER_SOL;
        await makeWithdrawal(connection, payer, sequencer, configPda, vaultPda, withdrawAmount);
        await logVaultBalance(connection, vaultPda);
    }

  } catch (err) {
    console.error("\n Script failed with error:", err);
    process.exit(1);
  }
})();


async function sendTx(connection, instruction, signers) {
    const transaction = new Transaction().add(instruction);
    const signature = await sendAndConfirmTransaction(connection, transaction, signers);
    console.log("  Transaction Signature:", signature);
    return signature;
}

async function fundAccount(connection, pubkey, amount) {
    console.log(` Funding account ${pubkey.toBase58()}...`);
    const airdropSignature = await connection.requestAirdrop(pubkey, amount);
    const latestBlockHash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropSignature,
    });
    console.log("    Funded.");
}

async function initializeBridgeIfNeeded(connection, payer, sequencer, configPda, vaultPda) {
    console.log("\nChecking if bridge is initialized...");
    const configAccountInfo = await connection.getAccountInfo(configPda);

    if (configAccountInfo === null) {
        console.log("Bridge not initialized. Sending 'initialize' transaction...");
        await fundAccount(connection, payer.publicKey, 5 * LAMPORTS_PER_SOL);
        await fundAccount(connection, sequencer.publicKey, 2 * LAMPORTS_PER_SOL);

        const initDataBuffer = Buffer.alloc(1 + 32 + 32);
        initDataBuffer.writeUInt8(0, 0);
        sequencer.publicKey.toBuffer().copy(initDataBuffer, 1);
        Buffer.alloc(32, 1).copy(initDataBuffer, 33);

        const initInstruction = new TransactionInstruction({
            keys: [
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                { pubkey: configPda, isSigner: false, isWritable: true },
                { pubkey: vaultPda, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            programId: PROGRAM_ID,
            data: initDataBuffer,
        });

        await sendTx(connection, initInstruction, [payer]);
        console.log(" Initialize successful!");
    } else {
        console.log(" Bridge already initialized.");
    }
}

async function makeDeposit(connection, depositor, configPda, vaultPda, amount) {
    const nonce = BigInt(Math.floor(Date.now() + Math.random() * 1000));
    const nonceBuffer = Buffer.alloc(8);
    nonceBuffer.writeBigUInt64LE(nonce, 0);

    const [depositReceiptPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("receipt"), configPda.toBuffer(), depositor.publicKey.toBuffer(), nonceBuffer],
        PROGRAM_ID
    );
    
    const depositDataBuffer = Buffer.alloc(1 + 8 + 8);
    depositDataBuffer.writeUInt8(1, 0);
    depositDataBuffer.writeBigUInt64LE(BigInt(amount), 1);
    depositDataBuffer.writeBigUInt64LE(nonce, 9);
    
    const depositInstruction = new TransactionInstruction({
        keys: [
            { pubkey: depositor.publicKey, isSigner: true, isWritable: true },
            { pubkey: configPda, isSigner: false, isWritable: false },
            { pubkey: vaultPda, isSigner: false, isWritable: true },
            { pubkey: depositReceiptPda, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId: PROGRAM_ID,
        data: depositDataBuffer,
    });
    await sendTx(connection, depositInstruction, [depositor]);
    console.log(` Deposited ${amount / LAMPORTS_PER_SOL} SOL.`);
}

async function makeWithdrawal(connection, payer, sequencer, configPda, vaultPda, amount) {
    const recipient = Keypair.generate();
    const nullifier = Keypair.generate().publicKey.toBuffer();

    const [usedNullifierPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("nullifier"), configPda.toBuffer(), nullifier],
        PROGRAM_ID
    );

    const withdrawDataBuffer = Buffer.alloc(1 + 32 + 8 + 32);
    withdrawDataBuffer.writeUInt8(2, 0);
    recipient.publicKey.toBuffer().copy(withdrawDataBuffer, 1);
    withdrawDataBuffer.writeBigUInt64LE(BigInt(amount), 33);
    nullifier.copy(withdrawDataBuffer, 41);

    const withdrawInstruction = new TransactionInstruction({
        keys: [
            { pubkey: sequencer.publicKey, isSigner: true, isWritable: true },
            { pubkey: configPda, isSigner: false, isWritable: false },
            { pubkey: vaultPda, isSigner: false, isWritable: true },
            { pubkey: recipient.publicKey, isSigner: false, isWritable: true },
            { pubkey: usedNullifierPda, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId: PROGRAM_ID,
        data: withdrawDataBuffer,
    });
    
    await sendTx(connection, withdrawInstruction, [payer, sequencer]);
    console.log(` Withdrew ${amount / LAMPORTS_PER_SOL} SOL to ${recipient.publicKey.toBase58()}.`);
}

/**
 * NEW HELPER FUNCTION
 * Fetches and logs the current balance of the vault PDA.
 */
async function logVaultBalance(connection, vaultPda) {
    const balance = await connection.getBalance(vaultPda);
    console.log(` Vault Balance: ${balance / LAMPORTS_PER_SOL} SOL`);
}

