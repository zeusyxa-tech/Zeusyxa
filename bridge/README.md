# 🌉 QUANTA Bridge — Hyperlane Implementation

> **STATUS: 🟥 EXPERIMENTAL / QUARANTINED** — Unaudited, undeployed, KNOWN BUG (globalMinted underflow in `bridgeOut`). **DO NOT USE with real funds.**

## ⚠️ Required before production use
1. Fix accounting bug: `globalMinted -= amount` underflows when `globalMinted == 0`
2. Full test coverage (unit + invariant + fuzz)
3. External audit (bridge code + Hyperlane integration)

---

> **Decision**: We use **Hyperlane** (audited, battle-tested) instead of building custom signature verification. This eliminates the #1 source of bridge hacks ($3B+ lost 2021-2025).

## Why Hyperlane (not LayerZero/Wormhole/custom)

| Factor | Hyperlane | LayerZero | Wormhole | Custom |
|--------|-----------|-----------|----------|--------|
| Permissionless | ✅ | Partial | ❌ | ✅ |
| Audit count | 6+ | 10+ | 5+ | 0-1 |
| Past exploits | None | None (oracle issues) | $326M | Many |
| Open source MIT | ✅ | ❌ | ✅ | Your code |
| Modular security (ISMs) | ✅ | ❌ | ❌ | DIY |
| Cost | Low | Low | Low | Your time |
| Time to integrate | 1 week | 1 week | 2 weeks | 6+ months |

**Choice**: Hyperlane offers best **trust minimization + customization + audit history**.

## Architecture

```
┌─────────────────────────┐        ┌─────────────────────────┐
│      Base mainnet        │        │   Arbitrum mainnet      │
│                          │        │                         │
│  ┌────────────────────┐  │        │  ┌────────────────────┐ │
│  │ QuantaToken (orig) │  │        │  │ QuantaToken (wrap) │ │
│  └────────┬───────────┘  │        │  └─────────┬──────────┘ │
│           │ bridgeBurn   │        │            │ bridgeMint │
│           ▼              │        │            ▲            │
│  ┌────────────────────┐  │        │  ┌─────────┴──────────┐ │
│  │ QuantaBridgeH...   │  │        │  │ QuantaBridgeH...   │ │
│  └────────┬───────────┘  │        │  └─────────▲──────────┘ │
│           │              │        │            │            │
│           ▼              │        │            │            │
│  ┌────────────────────┐  │        │  ┌─────────┴──────────┐ │
│  │ Hyperlane Mailbox  │──┼────────┼─▶│ Hyperlane Mailbox  │ │
│  └────────────────────┘  │        │  └────────────────────┘ │
└──────────────────────────┘        └─────────────────────────┘
                 │                                  ▲
                 │   Hyperlane validator set        │
                 └──────────────────────────────────┘
                  (7 default validators, can add custom ISM)
```

## Defense-in-depth security stack

1. **Hyperlane default ISM**: 7 validators must agree
2. **Custom ISM** (optional v2): require Dilithium signature in addition
3. **Rate limit on contract**: max 100K QTA/hour, 1M/day
4. **Anomaly detection**: auto-pause if mint > 5× rolling avg
5. **Phased TVL caps**: start at 100K, scale up
6. **Solvency tracking**: mint never exceeds locked
7. **Replay protection**: message IDs tracked
8. **48h timelock** on admin changes
9. **Pausable** with emergency multisig
10. **Forta monitoring** in real-time
11. **Bug bounty** ($100K+ critical)

Each layer catches different attack class. Need to break all 11 = practically impossible.

## Setup

```bash
# 1. Install Hyperlane CLI
npm install -g @hyperlane-xyz/cli

# 2. Hyperlane mailbox addresses (per chain)
# Base mainnet:     0xeA87ae93Fa0019a82A727bfd3eBd1cFCa8f64f1D
# Base Sepolia:     0x6966b0E55883d49BFB24539356a2f8A673E02039
# Arbitrum One:     0x979Ca5202784112f4738403dBec5D0F3B9daabB9
# Optimism:         0xd4C1905BB1D26BC93DAC913e13CaCC278CdCC80D
# Ethereum:         0xc005dc82818d67AF737725bD4bf75435d065D239

# 3. Deploy on each chain
export MAILBOX=0xeA87ae93Fa0019a82A727bfd3eBd1cFCa8f64f1D  # Base
export QUANTA_TOKEN=0xYOUR_DEPLOYED_TOKEN
export OWNER=0xYOUR_SAFE_MULTISIG

forge create QuantaBridgeHyperlane \
  --constructor-args $MAILBOX $QUANTA_TOKEN $QUANTA_TOKEN $OWNER \
  --rpc-url $BASE_RPC \
  --private-key $DEPLOYER_KEY \
  --verify

# 4. After deploying on ALL chains, set trusted bridges
# On Base:
cast send $BRIDGE_BASE "setTrustedBridge(uint32,bytes32)" \
  42161 $(cast --to-bytes32 $BRIDGE_ARBITRUM)

# On Arbitrum:
cast send $BRIDGE_ARB "setTrustedBridge(uint32,bytes32)" \
  8453 $(cast --to-bytes32 $BRIDGE_BASE)

# 5. Authorize bridge contracts as bridge() on QuantaToken
# Via Safe multisig: token.proposeBridge(bridge_address)
# Wait 48h, then executeBridgeChange()
```

## Hyperlane domain IDs (for reference)

| Chain | Domain | Mailbox |
|-------|--------|---------|
| Ethereum | 1 | 0xc005...D239 |
| Base | 8453 | 0xeA87...f1D |
| Base Sepolia | 84532 | 0x6966...2039 |
| Arbitrum | 42161 | 0x979C...abB9 |
| Optimism | 10 | 0xd4C1...c80D |
| Polygon | 137 | 0x5d93...ca29 |

## User flow

```typescript
// User wants to bridge 100 QTA from Base → Arbitrum

// 1. Approve QTA to bridge
await token.approve(bridgeBase, parseEther("100"));

// 2. Quote fee (small ETH amount for Hyperlane validator gas)
const fee = await bridgeBase.quoteBridgeOut(
  42161,                                          // Arbitrum domain
  ethers.zeroPadValue(recipientAddress, 32),     // recipient as bytes32
  parseEther("100")
);

// 3. Bridge
await bridgeBase.bridgeOut(
  42161,
  ethers.zeroPadValue(recipientAddress, 32),
  parseEther("100"),
  { value: fee }
);

// 4. Wait ~3-5 minutes for Hyperlane validators to confirm
// (track via Hyperlane Explorer: https://explorer.hyperlane.xyz)

// 5. Recipient receives 100 QTA on Arbitrum
```

## Testing

```bash
# Local test with Hyperlane sandbox
cd bridge
forge test --match-contract BridgeTest -vv

# Testnet test (Base Sepolia → Arbitrum Sepolia)
# Deploy bridges on both
# Then bridge 1 QTA, verify mint within 5 min on dest
```

## Phased rollout plan

| Week | Cap | Action |
|------|-----|--------|
| 1 | 100K QTA | Beta with whitelisted users only |
| 2-4 | 500K QTA | Public, monitor closely |
| 5-12 | 5M QTA | Scale if no incidents |
| 12+ | 100M QTA | Mature operation |

Reset cap to lower if ANY incident occurs.

## Migration to QUANTA L1 (post-v2)

When QUANTA L1 launches:
1. Deploy this bridge on L1 (Hyperlane supports custom chains)
2. Migrate users from EVM-only bridge to L1↔EVM bridge
3. Old EVM bridges remain operational
4. Token holders can move freely between L1 ↔ Base ↔ Arbitrum ↔ etc.

## Cost comparison

| Action | Cost (Base) | Cost (custom bridge) |
|--------|-------------|----------------------|
| Deploy this contract | $5 | $10-50 |
| Audit | $30K (small, since uses audited Hyperlane) | $200K+ (audit signature verification) |
| Per-transaction gas | ~$0.05 | $0.05+ |
| Maintenance | Hyperlane handles validators | You run validators |
| Insurance cost | Lower (less custom code) | Higher |
| **5-year TCO** | **~$50K** | **~$1M+** |

## What we DON'T do (intentionally)

- ❌ Build custom signature verification (use Hyperlane's audited)
- ❌ Run our own validators (use Hyperlane's permissionless set)
- ❌ Custom proof formats (use Hyperlane's standard)
- ❌ Allow bridging arbitrary ERC-20s (only QTA, prevents exploit surface)
- ❌ Upgrade bridge contract (immutable, deploy new version if needed)

**Boring is good in bridges.** Innovation here = catastrophe.
