# Multi-Language Compiler Test

Write-Host "=== TESTING ALL LANGUAGE FRONTENDS ===" -ForegroundColor Cyan

# Test Quorlin
Write-Host "`n[1/4] Quorlin (.ql)" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.ql --target evm

# Test Solidity
Write-Host "`n[2/4] Solidity (.sol)" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.sol --target evm

# Test Vyper
Write-Host "`n[3/4] Vyper (.vy)" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.vy --target evm

# Test Move
Write-Host "`n[4/4] Move (.move)" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.move --target aptos

Write-Host "`n=== ALL LANGUAGE TESTS COMPLETE ===" -ForegroundColor Green
