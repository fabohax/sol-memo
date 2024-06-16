use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::Message;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::read_keypair_file;
use std::str::FromStr;

fn main() {
    // Initialize the RPC client
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // Load the sender keypair
    let keypair_path = "keypair.json";
    let from_keypair = read_keypair_file(keypair_path).expect("Failed to read keypair file");

    // Shortened memo text
    let memo_text = "Agreement: Investor: Craig G. Lewis (FtwgRUsQNesSGRT8LxLbRFis1bGzZXC4Biv7nEQeb3Tj), Recipient: Fabo Hax (GzxRtLGAMLXZunL12qrbCA4yM7mcEC6vJfP4b3EgiT6S). Investment: $1,000 USD in SOL/USDC on Raydium. ROI: 1% daily: $300/month (Recipient’s expenses). 2% daily: $600/month (split $300 each). 3% daily: $900/month (split $300 each, $300 reinvested). Return: $1,000 when account reaches $10,000 or upon termination, balance split equally. Reporting: Weekly. Duration/Termination: Starts June 17, 2024, 7-day notice for termination. Risk: Investor aware of DeFi risks, no guaranteed returns. Both parties signed.";

    // Create memo instruction
    let memo_instruction = Instruction {
        program_id: solana_sdk::pubkey::Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").unwrap(),
        accounts: vec![],
        data: memo_text.as_bytes().to_vec(),
    };

    // Create a transfer instruction (can be a self-transfer to add memo)
    let transfer_instruction = system_instruction::transfer(
        &from_keypair.pubkey(),
        &from_keypair.pubkey(),  // Self-transfer
        1,  // Minimum lamports
    );

    // Create a transaction
    let message = Message::new(&[transfer_instruction, memo_instruction], Some(&from_keypair.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);

    // Sign the transaction
    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");
    transaction.sign(&[&from_keypair], recent_blockhash);

    // Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

    println!("Transaction successful with signature: {}", signature);
}
