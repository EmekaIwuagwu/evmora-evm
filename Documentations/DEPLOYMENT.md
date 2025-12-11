# Deployment Guide

## Understanding Evmora's Architecture

Evmora is currently a **library** (like `libgeth` or `reth-evm`), not a standalone application. This means:

```
┌─────────────────────────────────────────────┐
│  What Evmora IS:                             │
│  - Execution engine library                  │
│  - Smart contract compiler                   │
│  - Runtime orchestrator                      │
│  - Embeddable in your applications          │
└─────────────────────────────────────────────┘

┌─────────────────────────────────────────────┐
│  What Evmora is NOT (yet):                   │
│  - Standalone blockchain node                │
│  - JSON-RPC server                           │
│  - P2P networking layer                      │
│  - Consensus mechanism                       │
└─────────────────────────────────────────────┘
```

---

## Do I Need to Deploy to DigitalOcean/AWS?

### SHORT ANSWER: **NO**

For development, testing, and using Evmora as a library, everything runs on your local machine.

### LONG ANSWER: It depends on your use case

| Use Case | Deployment Needed? | Details |
|----------|-------------------|---------|
| **Learning EVM internals** | ❌ No | Run locally with `cargo test` and examples |
| **Building smart contracts** | ❌ No | Compile locally with `evmora-compiler` |
| **Integrating into your app** | ❌ No | Add as Rust dependency |
| **Running a public node** | ✅ Yes | Need to wrap in RPC server + deploy |
| **Building an L2 chain** | ✅ Yes | Need consensus + networking + deploy |

---

## Local Development Workflow

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone <your-repo>
cd evmora-evm
```

### Running Tests (No Deployment Required)
```bash
# Full test suite
cargo test --workspace

# Expected result: 3/3 tests passing in < 1 second
```

### Compiling Contracts (No Deployment Required)
```bash
# Build compiler
cargo build --release --bin evmora-compiler

# Compile a contract
./target/release/evmora-compiler compile \
    ./tests/fixtures/sol/Counter.sol \
    --lang sol \
    --out ./artifacts
```

### Running Examples (No Deployment Required)
```bash
# Multi-language compilation demo
cargo run -p evmora-compiler --example multilang_compile

# Contract execution demo
cargo run -p evmora-runtime --example basic_contract
```

---

## When You WOULD Deploy to Cloud

### Scenario 1: Building a Public RPC Node

If you want to run a node that others can connect to:

```rust
// You would create a JSON-RPC wrapper:

use jsonrpsee::server::Server;
use evmora_runtime::EvmClient;

#[tokio::main]
async fn main() {
    let server = Server::builder()
        .build("0.0.0.0:8545")
        .await
        .unwrap();

    let client = EvmClient::new("config.toml").unwrap();
    
    // Register RPC methods
    // server.register_method("eth_call", move |params| {
    //     client.execute(...)
    // });
    
    server.start().await;
}
```

**Then deploy to DigitalOcean:**

1. Create a Droplet (4GB RAM, 2 vCPUs)
2. Install Rust on the server
3. Clone and build Evmora
4. Run your RPC server

**Cost:** ~$20-40/month

---

### Scenario 2: Building an L2 Rollup

If you're building a Layer 2 chain:

```
Your L2 Stack:
┌─────────────────────────────┐
│  Consensus Layer            │ ← Deploy to cloud
│  (Tendermint/HotStuff)      │
├─────────────────────────────┤
│  Sequencer                  │ ← Deploy to cloud
│  (Transaction ordering)     │
├─────────────────────────────┤
│  Evmora Runtime             │ ← Embedded library
│  (Contract execution)       │
├─────────────────────────────┤
│  Data Availability          │ ← Deploy to cloud
│  (Celestia/EigenDA)         │
└─────────────────────────────┘
```

**Deployment Strategy:**
- Use Kubernetes for orchestration
- Deploy 3-5 validator nodes
- Use managed PostgreSQL for storage
- CDN for RPC endpoints

**Cost:** $500-2000/month (depending on scale)

---

## Recommended Architecture for Production

### Option A: Standalone Node (Simple)

```
Internet → Load Balancer → RPC Server (with Evmora embedded)
                             ↓
                          Database
```

**DigitalOcean Setup:**
- 1x Droplet (8GB RAM): RPC Server + Evmora
- 1x Managed PostgreSQL: Storage
- 1x Load Balancer (optional)

**Commands:**
```bash
# On DigitalOcean Droplet
ssh root@your-droplet-ip

# Install Rust
curl --proto='=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone <your-repo>
cd evmora-evm
cargo build --release

# Run your RPC server (you write this)
./target/release/your-rpc-server
```

---

### Option B: Scalable Infrastructure (Advanced)

```
           ┌─ Load Balancer ─┐
           │                  │
    ┌──────▼──────┐    ┌──────▼──────┐
    │  RPC Node 1  │    │  RPC Node 2  │
    └──────┬──────┘    └──────┬──────┘
           │                  │
    ┌──────▼──────────────────▼──────┐
    │     Shared Storage (RocksDB)    │
    └─────────────────────────────────┘
```

**Use Docker:**
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/evmora-node /usr/local/bin/
CMD ["evmora-node"]
```

**Kubernetes Deployment:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: evmora-node
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: evmora
        image: your-registry/evmora:latest
        ports:
        - containerPort: 8545
```

---

## Cost Estimates

### Local Development: **$0/month**
- Everything runs on your machine
- Only need electricity for your computer

### Small RPC Node (DigitalOcean): **$20-40/month**
- 1x Droplet (4GB): $20/month
- 1x Managed DB: $15/month
- Bandwidth: ~$5/month

### Medium Production (AWS/GCP): **$500-1000/month**
- 3x EC2 instances (t3.large): $150/month
- RDS PostgreSQL: $100/month
- Load Balancer: $20/month
- Bandwidth: $200/month
- Monitoring: $30/month

### Large Scale (Multi-region): **$5000+/month**
- 10+ nodes across regions
- Managed Kubernetes (EKS/GKE)
- Multi-region database replication
- CDN for global RPC access

---

## Current Recommendation

**For Now: Don't Deploy Anywhere**

Evmora is in **alpha** stage as an execution library. You should:

1. ✅ Run tests locally
2. ✅ Experiment with the compiler
3. ✅ Build integrations in your local environment
4. ❌ Don't deploy to production yet

**When to Deploy:**

Wait for these milestones (planned for v0.2.0):
- [ ] JSON-RPC wrapper available
- [ ] Persistent storage backend
- [ ] Full EVM compliance test suite passing
- [ ] Security audit completed

---

## Quick Decision Tree

```
Are you building a public blockchain node?
├─ YES → You'll need cloud deployment (v0.2.0+)
└─ NO  
   └─ Are you testing/learning?
      ├─ YES → Run locally (current version works great!)
      └─ NO
         └─ Are you integrating Evmora into your Rust app?
            ├─ YES → Use as Cargo dependency (no deployment)
            └─ NO → Consult with the team
```

---

## FAQ

**Q: Can I deploy Evmora to Heroku/Vercel/Netlify?**  
A: Not recommended. These are for web apps. Evmora is a low-level blockchain engine. Use DigitalOcean/AWS/GCP instead.

**Q: Do I need Docker?**  
A: Not required for local development. Useful for production deployment.

**Q: How many servers do I need?**  
A: For testing: 0 (local machine). For production RPC: 1-3. For L2: 5-10.

**Q: Can I use Windows Server?**  
A: Technically yes, but Linux (Ubuntu 22.04 LTS) is recommended for production.

**Q: What's the minimum hardware?**  
A: **Local testing:** Any modern laptop. **Production node:** 4GB RAM, 2 vCPUs, 50GB SSD.

---

## Next Steps

1. **Today:** Run `cargo test --workspace` locally
2. **This Week:** Experiment with examples, try compiling contracts
3. **This Month:** Build a small integration using Evmora as a library
4. **Next Quarter:** Consider deployment when v0.2.0 releases with RPC support

---

**Questions?** Open an issue or check the [README.md](./README.md) and [TESTING.md](./TESTING.md)
