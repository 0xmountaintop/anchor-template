import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { BN, Program, type Provider } from "@coral-xyz/anchor";
import { Xxx } from "../target/types/xxx";
import { clusterApiUrl, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import fs from 'fs';
import path from 'path';


export class XxxClient {
    private program: Program<Xxx>;
    private provider: anchor.AnchorProvider;
  
    constructor(
      connection: web3.Connection,
      wallet: anchor.Wallet,
      programId: PublicKey
    ) {
      this.provider = new anchor.AnchorProvider(
        connection,
        wallet,
        { commitment: "confirmed" }
      );
      
      // Load the program
      this.program = new Program<Xxx>(
        require("../target/idl/xxx.json"),
        programId,
        this.provider
      );
    }

    async getXxxPDA(userPubkey: PublicKey): Promise<[PublicKey, number]> {
        return PublicKey.findProgramAddress(
            [
                Buffer.from("object"),
                userPubkey.toBuffer(),
            ],
            this.program.programId
        );
    }

    async createObject(): Promise<string> {
        const [object] = await this.getXxxPDA(this.provider.wallet.publicKey);

        let info = await this.provider.connection.getAccountInfo(object);
        if (info == null) {
            const tx = await this.program.methods
                .createObject()
                .accounts({
                    payer: this.provider.wallet.publicKey,
                    object: object,
                    systemProgram: SystemProgram.programId,
            })
            .rpc();
        
            return tx;
        } else {
            return "Already initialized";
        }
    }

    async setObject(): Promise<string> {
        const [object] = await this.getXxxPDA(this.provider.wallet.publicKey);

        const tx = await this.program.methods
            .setObject(1)
            .accounts({
                object: object,
                user: this.provider.wallet.publicKey,
            })
            .rpc();

        return tx;
    }

    async getObjectValue(): Promise<number> {
        const [object] = await this.getXxxPDA(this.provider.wallet.publicKey);
        const result = await this.program.account.object.fetch(object);
        return result.value;
    }
}

async function main() {
    // const connection = new web3.Connection("http://localhost:8899");
    const connection = new web3.Connection(clusterApiUrl("devnet"));
  
    // const wallet = new anchor.Wallet(web3.Keypair.generate()); // Use your wallet in production
    const homedir = require('os').homedir();
    const default_wallet_path = path.join(homedir, '.config/solana/id.json');
    const wallet_path = process.env.WALLET_PATH || default_wallet_path;
    const kp = Keypair.fromSecretKey(Buffer.from(JSON.parse(fs.readFileSync(wallet_path, 'utf8')))); 
    const wallet = new anchor.Wallet(kp);
    
    const programId = new PublicKey("your_program_id_here");
  
    const client = new XxxClient(connection, wallet, programId);
    
    try {
        const createObjTx = await client.createObject();
        console.log("create_object tx:", createObjTx);

        const setObjTx = await client.setObject();
        console.log("set_object tx:", setObjTx);

        const value = await client.getObjectValue();
        console.log("object value:", value);
    } catch (error) {
        console.error("Error:", error);
    }
}

main();