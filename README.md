# Private Voting DAO (Powered by Arcium)

## Description
This project implements a **Confidential Governance System** for DAOs using Arcium's confidential computing network. In traditional DAOs, votes are often public, which can lead to voter intimidation or strategic voting. Using Arcium, we ensure that individual votes remain encrypted and hidden from everyone (including the nodes) until the final tally is revealed.

## How it works
1. **Encryption**: Each voter casts their choice (Yes/No), which is immediately secret-shared using Arcium's SDK.
2. **Confidential Tallying**: The votes are aggregated inside an **Encrypted Shared State**. Arcium nodes compute the sum without ever seeing individual choices.
3. **On-chain Settlement**: Only the final result is published to the Solana blockchain, ensuring complete privacy during the voting process.

## Requirements Met
* **Functional Project**: Core Rust logic for confidential tallying.
* **Privacy Benefits**: Prevents observation of votes before the final tally.
* **Open Source**: Licensed under MIT.

## Technical Implementation
This project utilizes:
* **Rust**: For high-performance and secure logic.
* **Arcium SDK**: To handle `Secret` types and multi-party computation (MPC).
* **Solana**: For transparent proposal management.

## Innovation
Unlike standard voting apps, this implementation focuses on "Universal Privacy," meaning no central authority can ever access the raw voting data.