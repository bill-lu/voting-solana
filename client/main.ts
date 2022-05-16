/**
 * Hello world
 */
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import path from "path";

import fs from "mz/fs";

// utility functions that are wrappers of @solana/web3.js used throughout the project
import {
  getPayer,
  establishConnection,
  establishEnoughSol,
  checkAccountDeployed,
  checkBinaryExists,
  getBalance,
} from "../utils/utils";

const initialize = (tracker, user, authority, votesCounter, trackerProgramId) => {
  return new TransactionInstruction({
    keys: [
      {
        pubkey: tracker,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: user,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: authority,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: votesCounter,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.from(new Uint8Array([0])),
    programId: trackerProgramId,
  });
};

const increment = (tracker, user, authority, votesCounter, counterProgramId, trackerProgramId) => {
  return new TransactionInstruction({
    keys: [
      {
        pubkey: tracker,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: user,
        isSigner: true,
        isWritable: false,
      },
      {
        pubkey: counterProgramId,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: votesCounter,
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: authority,
        isSigner: false,
        isWritable: false,
      },
    ],
    data: Buffer.from(new Uint8Array([1])),
    programId: trackerProgramId,
  });
};

// directory with binary and keypair
const PROGRAM_PATH = path.resolve(__dirname, "../target/deploy/");

// Path to program shared object file which should be deployed on chain.
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, "voting.so");
// Path to the keypair of the deployed program (This file is created when running `solana program deploy)
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, "voting-keypair.json");
const TRACKER_PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, "votingtracker-keypair.json");


let payer = new Keypair();
const votes = new Keypair();
let votesPubkey = votes.publicKey;

async function main() {
  let args = process.argv.slice(2);
  let votesAccountCretaed = false;

  if (args.length > 0) {
    // existing Votes
    console.log("Votes address provided");
    votesPubkey = new PublicKey(args[0]);
    votesAccountCretaed = true;
  }

  if (args.length > 1) {
    let secretKeyString = await fs.readFileSync(args[1], {
      encoding: "utf8",
    });
    console.log("Loaded Keypair from ", args[1]);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    payer = Keypair.fromSecretKey(secretKey);
  }

  // Establish connection to the cluster
  let connection: Connection = await establishConnection();

  // Make sure payer has enough funds for fees and if not, top-up the account
  await establishEnoughSol(connection, payer);
  // Balance after top-up
  let [startBalanceSol, startBalanceLamport] = await getBalance(
    connection,
    payer
  );

  // Check if binary exists (ie if it's been compiled)
  let votesProgramID = await checkBinaryExists(PROGRAM_KEYPAIR_PATH);
  let votesTrackerProgramID = await checkBinaryExists(TRACKER_PROGRAM_KEYPAIR_PATH);

  let tx = new Transaction();
  let signers = [payer];

  if ( ! votesAccountCretaed) {
    console.log("Generating new counter address");
    let createIx = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: votesPubkey,
      /** Amount of lamports to transfer to the created account */
      lamports: await connection.getMinimumBalanceForRentExemption(51),
      /** Amount of space in bytes to allocate to the created account */
      space: 51,
      /** Public key of the program to assign as the owner of the created account */
      programId: votesProgramID,
    });
    signers.push(votes);
    tx.add(createIx);
  }

  const trackerKey = (await PublicKey.findProgramAddress(
    [payer.publicKey.toBuffer(), votesPubkey.toBuffer()],
    votesTrackerProgramID
  ))[0];
  const authKey = (await PublicKey.findProgramAddress(
    [votesPubkey.toBuffer()],
    votesTrackerProgramID
  ))[0];

  let trackerData = await connection.getAccountInfo(trackerKey)
  if (!trackerData) {
    console.log("    -> No tracker account found. Creating new tracker account");
    const initializeIx = initialize(
      trackerKey,
      payer.publicKey,
      authKey,
      votesPubkey,
      votesTrackerProgramID
    );
    tx.add(initializeIx);
  }




  // Make sure the program is deployed
  if (await checkAccountDeployed(connection, votesProgramID)) {
    // Say hello to an account
    await sayHello(votesProgramID, connection, payer);

    // Print balances after the call
    let [endBalanceSol, endBalanceLamport] = await getBalance(
      connection,
      payer
    );

    console.log(
      `\nIt cost:\n\t${startBalanceSol - endBalanceSol} SOL\n\t${
        startBalanceLamport - endBalanceLamport
      } Lamports\nto perform the call`
    );
  } else {
    console.log(`\nProgram ${PROGRAM_SO_PATH} not deployed!\n`);
  }
}
/**
 *
 * @param programId
 * @param connection
 * @param payer
 * @description Send a transaction to the program to say hello
 */
export async function sayHello(
  programId: PublicKey,
  connection: Connection,
  payer: Keypair
): Promise<void> {
  // Creates transaction instruction object to be passed to transaction
  const transactionInstruction = new TransactionInstruction({
    keys: [], // Keys unnecessary to simply log output
    programId: programId,
    data: Buffer.alloc(0), // Program instruction data unnecessary for this program
  });

  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(transactionInstruction),
    [payer]
  );
}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
