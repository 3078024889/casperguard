# CasperGuard — DoraHacks 提交文案

## ═══════════════════════════════════════
## 【BUIDL 名称】
## ═══════════════════════════════════════

CasperGuard — AI-Powered RWA Compliance Oracle Agent


## ═══════════════════════════════════════
## 【想象 / PROBLEM (想象框)】
## ═══════════════════════════════════════

### 中文版：
DeFi 协议正在快速整合真实世界资产（RWA），但缺乏可信赖的自动化合规审查机制。
人工 KYC/AML 审核耗时耗力，无法跟上链上资产的增长速度；而传统中心化预言机
存在单点故障风险，不适合去信任金融基础设施。

CasperGuard 通过 AI Agent 自主监控 RWA 资产的合规状态，利用 x402 协议按需
付费获取数据，并将经过 AI 评估的合规证明永久写入 Casper 链上——构建一个
无需信任、可持续运行的 RWA 合规预言机。

### English version:
DeFi protocols are rapidly integrating Real-World Assets (RWA), but lack a 
trustless, automated compliance layer. Manual KYC/AML reviews can't scale 
with on-chain asset growth, and centralized oracles introduce single points 
of failure incompatible with DeFi's trustless ethos.

CasperGuard deploys an autonomous AI agent that continuously monitors RWA 
compliance, pays for data on-demand via the x402 protocol, and permanently 
records AI-generated compliance attestations on the Casper Network — creating 
a scalable, trust-minimized RWA compliance oracle.


## ═══════════════════════════════════════
## 【类别 / CATEGORY】
## ═══════════════════════════════════════

✅ 加密/Web3
子类别：DeFi · RWA · AI Agent · Oracle


## ═══════════════════════════════════════
## 【这个 BUIDL 是人工智能代理吗？】
## ═══════════════════════════════════════

✅ 是（开启切换）


## ═══════════════════════════════════════
## 【项目详细描述 / BUIDL DETAILS】
## ═══════════════════════════════════════

### 中文详细描述：

**CasperGuard** 是一个部署在 Casper 测试网上的自主 AI 合规代理，专为真实世界
资产（RWA）的 DeFi 整合提供可验证的合规层。

**核心工作流程：**

1. **x402 按需数据获取**  
   Agent 使用 Casper 的 x402 HTTP 支付协议，向数据提供商按请求付费，
   以 CSPR 微支付换取 RWA 资产数据（发行人信息、司法管辖区、价值等）。
   无订阅费，无预付款，完全自主。

2. **AI 风险评估引擎**  
   获取数据后，AI 引擎对资产进行多维度合规评分：
   - FATF 黑名单/灰名单司法管辖区检测（权重 45%）
   - 发行人身份核验（权重 30%）
   - AML 金额门槛检查（>$10M 触发增强审查，权重 15%）
   - 数据新鲜度验证（权重 10%）
   
   输出四级风险等级：LOW / MEDIUM / HIGH / CRITICAL

3. **链上合规证明**  
   AI 评估结果通过 `ComplianceOracle` 智能合约（基于 Odra 框架，Rust 编写）
   写入 Casper 测试网。每条记录包含：
   - 资产 ID + 风险等级
   - 报告的 SHA-256 哈希（链外完整报告的可验证锚点）
   - 时间戳 + Agent 地址

4. **预言机信誉系统**  
   合约在链上追踪每个 AI 预言机的历史准确率，形成可查询的信誉分数。
   DeFi 协议在接受合规证明前，可先验证预言机信誉。

5. **实时 Dashboard**  
   前端仪表盘展示所有链上证明、风险分布、Agent 活动日志和预言机信誉环形图。

**使用的 Casper AI Toolkit：**
- x402 Facilitator (cspr.cloud) — 微支付结算
- Casper MCP Server — AI Agent 直接访问区块链数据
- CSPR.cloud API — 测试网 RPC 接入
- Odra Framework — 智能合约开发与部署
- casper-eip-712 — x402 支付的签名方案

---

### English Detailed Description:

**CasperGuard** is an autonomous AI compliance agent deployed on Casper Testnet, 
providing a verifiable compliance layer for Real-World Asset (RWA) DeFi integration.

**Core Workflow:**

1. **x402 Pay-Per-Request Data Fetching**  
   The agent uses Casper's x402 HTTP payment protocol to pay data providers 
   per request in CSPR micropayments — exchanging payment proofs for RWA asset 
   data (issuer, jurisdiction, valuation). No subscriptions, no pre-approval.

2. **AI Risk Assessment Engine**  
   Multi-factor compliance scoring:
   - FATF blacklist/greylist jurisdiction detection (45%)
   - Issuer identity verification (30%)  
   - AML threshold checks >$10M (15%)
   - Data freshness validation (10%)
   
   Four-tier output: LOW / MEDIUM / HIGH / CRITICAL

3. **On-Chain Attestations**  
   Results are written to the `ComplianceOracle` contract (Odra/Rust) on 
   Casper Testnet. Each record anchors the full off-chain report via SHA-256 
   hash, enabling trustless verification.

4. **Oracle Reputation System**  
   The contract tracks historical accuracy on-chain. DeFi protocols can verify 
   oracle trustworthiness before accepting attestations.

5. **Live Dashboard**  
   Real-time UI displaying attestations, risk distribution, agent logs, and 
   reputation scores.

**Casper AI Toolkit used:** x402 Facilitator · Casper MCP Server · CSPR.cloud API 
· Odra Framework · casper-eip-712


## ═══════════════════════════════════════
## 【GitHub 链接】
## ═══════════════════════════════════════

https://github.com/3078024889/casperguard
（上传所有文件后填写）


## ═══════════════════════════════════════
## 【演示视频链接】
## ═══════════════════════════════════════

[上传到 YouTube 后填写]
推荐格式：3-5 分钟演示视频，包含：
1. 项目介绍（30秒）
2. Dashboard 演示（1分钟）
3. 运行 Agent 全流程（2分钟）
4. 查看链上交易（1分钟）


## ═══════════════════════════════════════
## 【社交链接（至少1个）】
## ═══════════════════════════════════════

GitHub: https://github.com/3078024889
（可添加 Twitter/X 账号）


## ═══════════════════════════════════════
## 【Layer-1 选择】
## ═══════════════════════════════════════

Casper Network


## ═══════════════════════════════════════
## 【其他开源生态系统】
## ═══════════════════════════════════════

x402 Protocol · CSPR.cloud · Odra Framework
