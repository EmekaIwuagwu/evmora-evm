# Comprehensive VM Test Script
Write-Host "Testing All VM Implementations..." -ForegroundColor Cyan

# Test Quorlin VM
Write-Host "`n=== Testing Quorlin VM ===" -ForegroundColor Yellow
cargo test --package evmora-quorlin-vm --lib 2>&1 | Tee-Object -Variable quorlin_result

# Test Solana VM
Write-Host "`n=== Testing Solana VM ===" -ForegroundColor Yellow
cargo test --package evmora-solana-vm --lib 2>&1 | Tee-Object -Variable solana_result

# Test Polkadot VM
Write-Host "`n=== Testing Polkadot VM ===" -ForegroundColor Yellow
cargo test --package evmora-polkadot-vm --lib 2>&1 | Tee-Object -Variable polkadot_result

# Test Aptos VM
Write-Host "`n=== Testing Aptos VM ===" -ForegroundColor Yellow
cargo test --package evmora-aptos-vm --lib 2>&1 | Tee-Object -Variable aptos_result

# Test EVM Core (Solidity)
Write-Host "`n=== Testing EVM Core (Solidity) ===" -ForegroundColor Yellow
cargo test --package evmora-core --lib 2>&1 | Tee-Object -Variable evm_result

# Summary
Write-Host "`n=== Test Summary ===" -ForegroundColor Cyan
Write-Host "Quorlin VM: " -NoNewline
if ($quorlin_result -match "test result: ok") { Write-Host "PASSED" -ForegroundColor Green } else { Write-Host "FAILED" -ForegroundColor Red }

Write-Host "Solana VM: " -NoNewline
if ($solana_result -match "test result: ok") { Write-Host "PASSED" -ForegroundColor Green } else { Write-Host "FAILED" -ForegroundColor Red }

Write-Host "Polkadot VM: " -NoNewline
if ($polkadot_result -match "test result: ok") { Write-Host "PASSED" -ForegroundColor Green } else { Write-Host "FAILED" -ForegroundColor Red }

Write-Host "Aptos VM: " -NoNewline
if ($aptos_result -match "test result: ok") { Write-Host "PASSED" -ForegroundColor Green } else { Write-Host "FAILED" -ForegroundColor Red }

Write-Host "EVM Core (Solidity): " -NoNewline
if ($evm_result -match "test result: ok") { Write-Host "PASSED" -ForegroundColor Green } else { Write-Host "FAILED" -ForegroundColor Red }

Write-Host "`nAll VM tests completed!" -ForegroundColor Cyan
