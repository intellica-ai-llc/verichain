# Addendum E — Revenue evidence, pricing signals and targets (complements D)

Date: 2026-10-23 · Source quality: analyst/vendor articles are tagged [A] (analyst/vendor, weak), [P] (primary/regulatory), [J] (my judgement/assumption).

## 1. What the current ARC42s say about money
- VERICHAIN_ARC42: **no revenue model.** Money is mentioned only as validator "verification fees", "oversight premiums", UBE fee distributions and "revenue funds paid infrastructure". There are no prices, customers or unit economics.
- MAS_ARC42: money = trading P&L from three swarms (targets: Sharpe ≥3, Sortino ≥5). Capital caps are small (charters total ~1.15M sats; per-trade caps $20–$100). Even a strong monthly return on that capital cannot fund a company, and Addendum B shows the end-of-candle edge has negative expected value as written. [J, from the docs + Addendum B]
- Conclusion: VeriChain as specified has a technology plan but no business plan. Closing that is the highest-value gap.

## 2. Where the money actually is (evidence)
| Signal | Fact | Tag |
|---|---|---|
| Market | Agent observability & governance: $1.23B (2025), $1.68B (2026), $8.62B by 2031 (38.7% CAGR); no vendor >15% share; BFSI the largest vertical (~20%) | [A] Mordor |
| Regulation (EU) | AI Act Digital Omnibus = Regulation (EU) 2026/1744, in force 27 Jul 2026. Annex III high-risk obligations deferred to **2 Dec 2027**, Annex I to 2 Aug 2028. Art. 50 transparency applied 2 Aug 2026. Fines up to €15M/3% for most high-risk breaches | [P-secondary] CSA note |
| Regulation (US/EU banking) | SR 11-7 replaced 17 Apr 2026 (SR 26-2 / OCC 2026-13) and *excludes* generative/agentic AI; DORA incident reporting within 4 h; 17a-4 time-stamped audit-trail logic. The article found **no enforcement action yet** against a bank for an autonomous agent; mappings are the author's inference | [A] Diagrid |
| Insurance | AIUC-1 certification + Lloyd's capacity ($50M ElevenLabs policy, Feb 2026); insurers adding AI exclusions (Verisk/ISO, Berkley). Underwriters want identity, portable track record, controls evidence. Article doesn't mention logs, and sells identity | [A] Tobira |
| Pricing (very weak) | per action $0.01–$0.08; audit-trail add-on $0.05–$0.15 per transaction; per-agent $10–50k/yr; tiers $5–15k / $15–40k / $40–100k+ per month; cites 2023 Forrester | [A] Monetizely |
| Crypto-native agent payments | x402: ~$24M in 30 days to Jul 2026, mostly sub-dollar bots; one tracker ≈ $28k/day genuine | [A] BlindPay |
| Competition on receipts | ~15 products; many OSS and free; basic receipts trend to ~$0 | Addendum C |

## 3. Revenue wedges, ranked (judgement)
1. **Regulated-deployer evidence packs** (banks, fintechs, insurers, government). Sell "can you prove what your agent did, to an examiner, years later" with a verifier, retention and export, not raw receipts. Deadline pull: EU high-risk Dec 2027; banking supervision shifting after SR 11-7. First customers are in your own stack: **Verity** (agent-native banking) and **VeriCrypt** (PQ compliance) become design partners; Caribbean central-bank/government work via the national programme. [J]
2. **Underwriting/assurance feed.** Evidence of completeness + authorization lineage + track record sold to AIUC-style certifiers, MGAs and auditors (e.g., as the data source behind a certificate). Revenue = per-certified-agent or revenue share. Speculative; validate with 3 conversations before building. [J]
3. **Hosted witness / anchoring / retention** (open-core). OSS spec + verifier + self-hosted log; charge for independent witnessing, RFC 3161/anchoring, long-term PQ renewal, legal-hold retention. This is the part customers cannot self-serve credibly (independence is the product). [J]
4. **Verified performance ledger for trading/agent strategies** (flip the MAS idea: sell verification, don't trade). Your own ADR-008 shows demand: a bot "80.2% win rate" that was really −$89. One study: 46,945 wallets in a week, 36.8% profitable. Sell attested strategy provenance (which code version, no retroactive edits, authorization limits) to allocators, copy-trading platforms and bot marketplaces. Needs validation. [J + Telonex]
5. **Certification of agent runtimes** (completeness level C2/C3): auditor-style fee. Slower, needs credibility and independent audit. [J]
6. De-prioritize: UBE token, exchange/tokenization, Lightning-first settlement, validator-fee economics, own trading P&L.

## 4. What "exponential" can mean here (and what it cannot)
- Revenue scales with **number of agents × actions × retention years**, not with trading capital. Evidence is a recurring obligation: retention periods (e.g., six months under AI Act logging, longer in finance) make revenue sticky. [P-secondary for six months; rest J]
- Network effects are real only if the **verifier and receipt format become the thing others check**. That argues for open standards (SCITT profile, public test vectors), an OSS verifier, and being cited by regulators/insurers. Closed formats cap growth.
- Each customer's witnessed log strengthens the shared witness set; a portable cross-vendor "agent track record" (ERC-8004-compatible) is the long-run asset. [J]
- Unit-economics sanity (arithmetic): $1M ARR = $83k/month. At $0.05 per action = 1.67M actions/month; at $0.02 = 4.2M. At a $60k/yr contract = 17 customers; at $12k/yr = 83. Basic per-action pricing will be pressured by free OSS, so price on assurance and compliance, not events.

## 5. Targets I'd propose (assumptions; to be replaced by your real objectives)
- By week 10: working log + verifier + ASL emitter + MCP gateway adapter; 3 design partners signed (letter of intent).
- By month 6: 2 paying pilots; one evidence pack accepted by a real auditor/insurer/regulator conversation.
- By month 12: 10–15 paying customers; one standards contribution (SCITT agent profile); independent crypto review done.
- These figures are my guess; your stated financial objectives are not in the repo, so they need your input.

## 6. Risks to the money case
- Demand timing: no enforcement against a bank yet; Annex III obligations are 14 months away. [A]/[P]
- Incumbents (Datadog, Splunk, Microsoft Agent 365) can bundle "good enough" logging. Differentiator must be independent verifiability and completeness.
- Single-founder capacity; ASL dependency; sales cycles in banking are long.
- Price evidence is thin; run customer discovery before fixing prices.

## Sources
- https://www.mordorintelligence.com/industry-reports/agent-observability-and-governance-market
- https://labs.cloudsecurityalliance.org/research/csa-research-note-eu-ai-act-high-risk-deadline-omnibus-20260/
- https://www.diagrid.io/infrastructure/2026-compliance-checklist-ai-agents-banking
- https://blog.tobira.ai/insuring-ai-agents-underwriting-identity/
- https://www.getmonetizely.com/articles/agentic-ai-governance-pricing-compliance-and-control-features
- https://blindpay.com/resources/more/agent-payment-protocols-compared.md
- https://telonex.io/research/top-crypto-traders-polymarket-15m
