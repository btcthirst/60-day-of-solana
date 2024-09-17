# 60-day-of-solana
update solana knowledge

## used command list
```bash
# step 0
anchor init day_1 --no-git
anchor build
solana config set --url localhost
anchor keys sync
anchor test --skip-local-validator
# step 1.1
anchor test --skip-local-validator
```
### second bash comand
```bash
# step 0
solana-test-validator
# step 1.0
# ctrl+C
solana-test-validator-reset
```