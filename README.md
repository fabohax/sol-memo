# Sol-Memo Transaction with Investment Agreement

This project demonstrates how to create a Solana transaction that includes a memo with a contract between two parties about an investment agreement. The memo is stored on the Solana blockchain, providing a permanent and verifiable record of the agreement.

## Overview

The project uses Rust and the Solana SDK to:
1. Connect to the Solana mainnet.
2. Load a keypair from a local file.
3. Create a memo instruction with the details of an investment agreement.
4. Create a self-transfer transaction to include the memo.
5. Sign and send the transaction to the Solana network.

## Memo Details

The memo contains a concise version of the investment agreement between Craig G. Lewis and Fabo Hax. The agreement includes:
- Parties involved.
- Date of the agreement.
- Investment amount.
- Expected ROI (Return on Investment).
- Payout and reinvestment strategy.
- Return of initial investment conditions.
- Reporting requirements.
- Duration and termination conditions.
- Risk acknowledgment.
- Signatures.

## Files

- `main.rs`: The main Rust file containing the code to create and send the transaction.
- `keypair.json`: The keypair file used to sign the transaction (not included for security reasons).

## Usage

### Prerequisites

- Rust installed on your system.
- A Solana keypair file (`keypair.json`) in the same directory as `main.rs`.

### Instructions

1. Clone this repository:
    ```sh
    git clone <repository_url>
    cd <repository_name>
    ```

2. Ensure you have a `keypair.json` file in the same directory as `main.rs`.

3. Run the project:
    ```sh
    cargo run
    ```

