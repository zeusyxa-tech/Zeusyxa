# 📚 ZEUSYXA — Master Index

> Complete project map. Everything in one place.

## 🚀 Start Here

|| File | Purpose ||
|------|---------|
| [README.md](./README.md) | Project overview, what is ZEUSYXA |
| [LAUNCH_GUIDE_7_DAYS.md](./LAUNCH_GUIDE_7_DAYS.md) | Day-by-day launch plan ($0 budget) |
| [CHECKLIST_7_DAYS.md](./CHECKLIST_7_DAYS.md) | Print + tick checklist for launch |

## 📐 Design Docs

| File | Purpose |
|------|---------|
| [docs/WHITEPAPER.md](./docs/WHITEPAPER.md) | Full architecture + quantum-safety design |
| [docs/TOKENOMICS.md](./docs/TOKENOMICS.md) | 1B supply, halving, burn mechanics |
| [docs/ROADMAP.md](./docs/ROADMAP.md) | Phase 0 → Phase 5 timeline |
| [docs/MARKETING.md](./docs/MARKETING.md) | Viral hooks, GTM strategy |

## 🛡️ Security (READ ALL OF THESE!)

| File | Purpose |
|------|---------|
| **[SECURITY_AUDIT.md](./SECURITY_AUDIT.md)** | Internal audit: 6 Critical, 8 High, 7 Medium, 9 Low |
| **[SECURITY.md](./SECURITY.md)** | Public security policy + bug bounty |
| [docs-security/README.md](./docs-security/README.md) | Master security index |
| [docs-security/BRIDGE_SECURITY_REVIEW.md](./docs-security/BRIDGE_SECURITY_REVIEW.md) | 15 bridge attacks + mitigations |
| [docs-security/INCIDENT_RESPONSE.md](./docs-security/INCIDENT_RESPONSE.md) | Runbook for exploits (5 min → 24h) |
| [docs-security/AUDIT_OPTIONS_COMPARISON.md](./docs-security/AUDIT_OPTIONS_COMPARISON.md) | Compare audit firms |

## 💻 Smart Contracts

### Production (v1.1, all security fixes applied)
| File | Lines | Purpose |
|------|-------|---------|
| [contracts/src-v1.1/QuantaToken.sol](./contracts/src-v1.1/QuantaToken.sol) | 176 | ERC-20 + burn + AI tax + bridge |
| [contracts/src-v1.1/AIAgentRegistry.sol](./contracts/src-v1.1/AIAgentRegistry.sol) | 224 | Agent identity + spending policy |
| [contracts/src-v1.1/AIPaymentChannel.sol](./contracts/src-v1.1/AIPaymentChannel.sol) | 226 | x402 micropayments (EIP-712) |
| [contracts/src-v1.1/AIModelMarketplace.sol](./contracts/src-v1.1/AIModelMarketplace.sol) | 231 | AI inference marketplace |

### Experimental / Quarantined
| File | Status |
|------|--------|
| [bridge/experimental/QuantaBridgeHyperlane.sol](./bridge/experimental/QuantaBridgeHyperlane.sol) | 🟥 Unaudited, known bug — DO NOT USE |

### Testing
| Path | Purpose |
|------|---------|
| [contracts/test-v1.1/SecurityFixes.t.sol](./contracts/test-v1.1/SecurityFixes.t.sol) | Regression tests for each finding |
| [contracts/test-invariant/](./contracts/test-invariant/) | Foundry + Echidna invariant tests |
| [contracts/test-formal/HalmosSpecs.t.sol](./contracts/test-formal/HalmosSpecs.t.sol) | Formal verification with Halmos |

### Scripts
| File | Purpose |
|------|---------|
| [contracts/script/Deploy.s.sol](./contracts/script/Deploy.s.sol) | Deploy all 4 contracts |
| [multisig-setup/transfer-ownership-script.s.sol](./multisig-setup/transfer-ownership-script.s.sol) | Transfer to multisig |
| [contracts/run-all-security.sh](./contracts/run-all-security.sh) | Run all security tools |

## 🤖 Forta Detection Bot

| File | Purpose |
|------|---------|
| [forta-bot/README.md](./forta-bot/README.md) | Setup guide |
| [forta-bot/src/agent.js](./forta-bot/src/agent.js) | Main bot agent |
| [forta-bot/src/detectors.js](./forta-bot/src/detectors.js) | 9 detection functions |
| [forta-bot/src/config.js](./forta-bot/src/config.js) | Thresholds + contract addresses |
| [forta-bot/src/state.js](./forta-bot/src/state.js) | Rolling window state |

## 🔐 Multisig Setup

| File | Purpose |
|------|---------|
| [multisig-setup/MULTISIG_SETUP_GUIDE.md](./multisig-setup/MULTISIG_SETUP_GUIDE.md) | Architecture + day-to-day ops |
| [multisig-setup/HARDWARE_WALLET_CEREMONY.md](./multisig-setup/HARDWARE_WALLET_CEREMONY.md) | 11-step key ceremony |
| [multisig-setup/verify-multisig.sh](./multisig-setup/verify-multisig.sh) | Verify ownership transferred |

## 🎮 War Games (Quarterly Drills)

| File | Difficulty | Scenario |
|------|-----------|----------|
| [wargames/README.md](./wargames/README.md) | — | Overview |
| [wargames/WG-01-active-exploit.md](./wargames/WG-01-active-exploit.md) | 🔥🔥🔥 | Funds being drained |
| [wargames/WG-02-signer-compromise.md](./wargames/WG-02-signer-compromise.md) | 🔥🔥 | Multisig signer hacked |
| [wargames/WG-03-frontend-hack.md](./wargames/WG-03-frontend-hack.md) | 🔥🔥🔥 | Website serving malicious JS |
| [wargames/WG-04-zero-day-disclosure.md](./wargames/WG-04-zero-day-disclosure.md) | 🔥 | Whitehat bug report |
| [wargames/WG-05-oracle-failure.md](./wargames/WG-05-oracle-failure.md) | 🔥🔥 | Reputation oracle broken |
| [wargames/WG-06-bridge-anomaly.md](./wargames/WG-06-bridge-anomaly.md) | 🔥🔥 | Suspicious bridge mint |

## 📜 Audit Applications (Ready to Submit)

| File | Platform |
|------|----------|
| [audit-applications/CODE4RENA_SUBMISSION.md](./audit-applications/CODE4RENA_SUBMISSION.md) | Code4rena |
| [audit-applications/SHERLOCK_APPLICATION.md](./audit-applications/SHERLOCK_APPLICATION.md) | Sherlock |
| [audit-applications/IMMUNEFI_BOUNTY.md](./audit-applications/IMMUNEFI_BOUNTY.md) | Immunefi (bug bounty) |
| [audit-applications/GRANTS_APPLICATIONS.md](./audit-applications/GRANTS_APPLICATIONS.md) | Grants (Base, EF, Optimism, etc.) |

## 🌉 Bridge

| File | Purpose |
|------|---------|
| [bridge/README.md](./bridge/README.md) | Why Hyperlane + setup |
| [bridge/experimental/QuantaBridgeHyperlane.sol](./bridge/experimental/QuantaBridgeHyperlane.sol) | 🟥 Bridge contract — experimental, quarantined |

## 📱 Wallet UI (Demo)

| File | Purpose |
|------|---------|
| [wallet-ui/index.html](./wallet-ui/index.html) | Self-contained wallet with tx simulation |
| [wallet-ui/README.md](./wallet-ui/README.md) | Production architecture guide |

## 🧮 Simulator

| File | Purpose |
|------|---------|
| [simulator/tokenomics_sim.py](./simulator/tokenomics_sim.py) | 30-year economic sim |

## 🎓 Security Training

| File | Audience | Duration |
|------|----------|----------|
| [security-training/README.md](./security-training/README.md) | All | — |
| [security-training/01-basics.md](./security-training/01-basics.md) | Everyone | 30 min |
| [security-training/02-scams.md](./security-training/02-scams.md) | Everyone | 45 min |
| [security-training/03-hardware-wallets.md](./security-training/03-hardware-wallets.md) | Power users | 60 min |
| [security-training/04-contract-reading.md](./security-training/04-contract-reading.md) | Developers | 90 min |
| [security-training/05-multisig.md](./security-training/05-multisig.md) | Treasurers | 60 min |
| [security-training/07-ai-agents.md](./security-training/07-ai-agents.md) | AI builders | 60 min |

## 🐍 Python Prototype

| File | Purpose |
|------|---------|
| [prototype/quantum_wallet.py](./prototype/quantum_wallet.py) | Merkle Signature Scheme |
| [prototype/pouw_consensus.py](./prototype/pouw_consensus.py) | Proof of Useful Work |
| [prototype/ai_agent.py](./prototype/ai_agent.py) | AI agent wallet |
| [prototype/blockchain.py](./prototype/blockchain.py) | Core blockchain |
| [prototype/demo.py](./prototype/demo.py) | End-to-end demo |

## 📦 TypeScript SDK

| File | Purpose |
|------|---------|
| [sdk/README.md](./sdk/README.md) | SDK usage |
| [sdk/src/client.ts](./sdk/src/client.ts) | Main QuantaClient |
| [sdk/src/agent.ts](./sdk/src/agent.ts) | AI agent registration |
| [sdk/src/channel.ts](./sdk/src/channel.ts) | Payment channels |
| [sdk/src/marketplace.ts](./sdk/src/marketplace.ts) | Model marketplace |
| [sdk/examples/autonomous-agent.ts](./sdk/examples/autonomous-agent.ts) | Viral demo |
| [sdk/examples/langchain-integration.ts](./sdk/examples/langchain-integration.ts) | LangChain Tool |

## 🧮 Simulator

| File | Purpose |
|------|---------|
| [simulator/tokenomics_sim.py](./simulator/tokenomics_sim.py) | 30-year economic sim |

## ⚙️ CI / DevOps

| File | Purpose |
|------|---------|
| [.github/workflows/security.yml](./.github/workflows/security.yml) | CI: Slither + Mythril + tests |
| [contracts/foundry.toml](./contracts/foundry.toml) | Foundry config |
| [contracts/slither.config.json](./contracts/slither.config.json) | Slither config |
| [contracts/.env.example](./contracts/.env.example) | Environment template |

---

## 📊 Statistics

- **Files**: 93
- **Total size**: ~908 KB
- **Lines of code**: ~12,000+
- **Languages**: Solidity, TypeScript, Python, JavaScript, HTML, Markdown, YAML

## 🗺️ Reading order (recommended)

### Day 1: Understand
1. README.md (15 min)
2. docs/WHITEPAPER.md (45 min)
3. docs/TOKENOMICS.md (30 min)

### Day 2: Security
4. SECURITY_AUDIT.md (60 min)
5. docs-security/README.md (15 min)
6. docs-security/INCIDENT_RESPONSE.md (60 min)

### Day 3: Plan
7. LAUNCH_GUIDE_7_DAYS.md (60 min)
8. multisig-setup/MULTISIG_SETUP_GUIDE.md (60 min)
9. audit-applications/GRANTS_APPLICATIONS.md (30 min)

### Day 4: Code
10. contracts/src-v1.1/QuantaToken.sol (60 min)
11. sdk/examples/autonomous-agent.ts (30 min)
12. forta-bot/src/agent.js (30 min)

### Day 5: Operations
13. wargames/WG-01-active-exploit.md (60 min)
14. multisig-setup/HARDWARE_WALLET_CEREMONY.md (45 min)
15. security-training/* (~6 hours total — spread out)

**Total**: ~10-12 hours of focused reading.

After reading: you're ahead of 99% of crypto founders on security.

---

## 🎯 Recommended next actions

| When | Action |
|------|--------|
| Today | Read INDEX.md (you're doing it) |
| This week | Read all SECURITY_*.md files |
| Next week | Start 7-day launch plan |
| Month 2 | Apply for grants + bug bounty |
| Month 3 | First external audit |
| Month 4-6 | Mainnet preparation |
| Month 6+ | Phased mainnet launch with TVL caps |

---

**You now have the most comprehensive open-source DeFi security toolkit available.**
**Most projects pay $200K+ for what you have here for free.**
**Use it well. Launch safely. Help the next founder.** 🚀
