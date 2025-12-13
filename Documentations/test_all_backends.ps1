# Testing all backends with semantic analysis

Write-Host "=== BACKEND SEMANTIC ANALYSIS TESTS ===" -ForegroundColor Cyan

# Test 1: EVM (default) - Should work
Write-Host "`n[TEST 1] EVM Backend - Simple Token" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.ql --target evm

# Test 2: Vulnerable contract on EVM - Should show reentrancy warning
Write-Host "`n[TEST 2] EVM Backend - Vulnerable Contract" -ForegroundColor Yellow  
.\target\debug\evmora-compiler.exe compile test_vulnerable.ql --target evm

# Test 3: Solana backend
Write-Host "`n[TEST 3] Solana Backend" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.ql --target solana

# Test 4: Polkadot backend  
Write-Host "`n[TEST 4] Polkadot Backend" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.ql --target polkadot

# Test 5: Aptos backend
Write-Host "`n[TEST 5] Aptos Backend" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.ql --target aptos

# Test 6: Quorlin backend
Write-Host "`n[TEST 6] Quorlin Native Backend" -ForegroundColor Yellow
.\target\debug\evmora-compiler.exe compile test_token.ql --target quorlin

Write-Host "`n=== ALL TESTS COMPLETE ===" -ForegroundColor Green
