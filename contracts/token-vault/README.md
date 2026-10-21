# Token Vault Contract

Soroban smart contract implementing a standard token vault with administrative control, token minting/burning, and SEP-41 compliant transfers and allowances.

## Features
- **Administrative Control**: Secure initialization and pause capabilities.
- **Transfers**: SEP-41 compliant token transfer and transfer_from support, handling zero-amount transfers gracefully without state mutation.
- **Allowances**: Time-bound allowance management with expiration checking.
- **Minting/Burning**: Relayer/admin-authorized supply expansion and token burning.

## Testing
`transfer`, `transfer_from`, and `burn` fail with the contract error
`InsufficientBalance` (code `3`) when the source balance cannot cover the amount.
Failed calls leave balances, allowances, and total supply unchanged. The existing
error codes and successful call signatures are preserved.

Run unit tests within this directory:
```bash
cargo test
```
