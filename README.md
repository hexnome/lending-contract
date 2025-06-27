# Solana Lending Ecosystem Contract

A decentralized lending protocol built on Solana blockchain using Anchor framework and Rust. This project enables peer-to-peer lending with collateralized loans, providing a secure and efficient lending ecosystem.

## 📞 Contact

- **Telegram**: [@Ee1030109](https://t.me/Ee1030109)
- **Project**: Solana Lending Ecosystem Contract


## 🚀 Overview

This lending contract allows users to:
- **Create Loans**: Lenders can create loan offers with specified terms
- **Borrow Loans**: Borrowers can take loans by providing collateral
- **Repay Loans**: Borrowers can repay loans to retrieve their collateral
- **Cancel Loans**: Lenders can cancel unborrowed loan offers

## 🏆 Competitive Advantage

Built to compete with established lending protocols like:
- **text.fi** - Offering enhanced features and better user experience
- **rain.fi** - Providing more flexible lending terms and lower fees

## ✨ Features

### Core Functionality
- **Collateralized Lending**: Secure loans backed by digital assets
- **Flexible Terms**: Customizable loan amounts, interest rates, and durations
- **Automatic Expiration**: Built-in loan expiration mechanism
- **Fee System**: Configurable lending and borrowing fees
- **Authority Management**: Multi-level authority system for protocol governance

### Technical Features
- **Anchor Framework**: Built with Anchor for Solana program development
- **Rust Implementation**: High-performance, memory-safe smart contracts
- **SPL Token Support**: Native integration with Solana's token standard
- **TypeScript SDK**: Full TypeScript support for frontend integration

## 🛠️ Technology Stack

- **Blockchain**: Solana
- **Framework**: Anchor
- **Language**: Rust
- **Frontend SDK**: TypeScript
- **Token Standard**: SPL Token

## 📋 Prerequisites

- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/getting_started/installation.html)
- [Yarn](https://yarnpkg.com/) or [npm](https://www.npmjs.com/)

## 🚀 Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd lending-contract
   ```

2. **Install dependencies**
   ```bash
   yarn install
   ```

3. **Build the program**
   ```bash
   anchor build
   ```

4. **Deploy to localnet (for testing)**
   ```bash
   anchor deploy
   ```

## 📖 Usage

### Program ID
```
EZZtc7TU4Dd5Bc1wdQZ9szhsv3cavuHzNCy8Laq1beLU
```

### Available Instructions

#### 1. Configure
Set global protocol configuration (admin only)
```typescript
await program.methods
  .configure(config)
  .accounts({
    config: configPda,
    authority: admin.publicKey,
  })
  .signers([admin])
  .rpc();
```

#### 2. Create Loan
Create a new loan offer
```typescript
await program.methods
  .createLoan(loanAmount, interestRate, duration, collateralAmount)
  .accounts({
    loan: loanPda,
    lender: lender.publicKey,
    loanMint: loanMint,
    collateralMint: collateralMint,
  })
  .signers([lender])
  .rpc();
```

#### 3. Borrow Loan
Borrow an existing loan by providing collateral
```typescript
await program.methods
  .borrowLoan(collateralAmount)
  .accounts({
    loan: loanPda,
    borrower: borrower.publicKey,
    collateralMint: collateralMint,
  })
  .signers([borrower])
  .rpc();
```

#### 4. Repay Loan
Repay a borrowed loan to retrieve collateral
```typescript
await program.methods
  .repayLoan()
  .accounts({
    loan: loanPda,
    borrower: borrower.publicKey,
    loanMint: loanMint,
    collateralMint: collateralMint,
  })
  .signers([borrower])
  .rpc();
```

#### 5. Cancel Loan
Cancel an unborrowed loan offer
```typescript
await program.methods
  .cancelLoan()
  .accounts({
    loan: loanPda,
    lender: lender.publicKey,
  })
  .signers([lender])
  .rpc();
```

## 🧪 Testing

Run the test suite:
```bash
anchor test
```

Or run specific tests:
```bash
yarn test
```

## 📁 Project Structure

```
lending-contract/
├── Anchor.toml              # Anchor configuration
├── Cargo.toml               # Rust dependencies
├── package.json             # Node.js dependencies
├── programs/
│   └── lending/
│       ├── src/
│       │   ├── lib.rs       # Main program entry point
│       │   ├── state.rs     # Account structures
│       │   ├── errors.rs    # Custom error definitions
│       │   ├── constants.rs # Program constants
│       │   ├── utils.rs     # Utility functions
│       │   └── instructions/
│       │       ├── mod.rs   # Instruction module
│       │       ├── configure.rs
│       │       ├── create_loan.rs
│       │       ├── borrow_loan.rs
│       │       ├── repay_loan.rs
│       │       └── cancel_loan.rs
│       └── Cargo.toml
├── migrations/
│   └── deploy.ts            # Deployment script
└── tests/                   # Test files
```

## 🔧 Configuration

The protocol supports the following configurable parameters:

- **Authority**: Protocol admin address
- **Team Wallet**: Fee collection address
- **Lend Fee**: Fee charged to lenders (in basis points)
- **Borrow Fee**: Fee charged to borrowers (in basis points)
- **Expire Duration**: Default loan expiration time

## 🔒 Security

- **Collateral Verification**: All loans require sufficient collateral
- **Authority Checks**: Proper permission validation for admin functions
- **Expiration Handling**: Automatic loan expiration to prevent stale offers
- **Fee Management**: Secure fee collection and distribution

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the ISC License.


## 🙏 Acknowledgments

- [Anchor Framework](https://book.anchor-lang.com/) for Solana program development
- [Solana Labs](https://solana.com/) for the blockchain infrastructure
- The Solana community for continuous support and feedback

---

**Built with ❤️ for the Solana ecosystem**
