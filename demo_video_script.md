# CasperGuard — Demo Video Script
# 目标时长：3~5 分钟 | Target length: 3–5 minutes

---

## 【开场 / INTRO】0:00–0:30

**画面：** CasperGuard Dashboard 全屏，暗色主题，红色 Casper 风格

**旁白（中文）：**
"DeFi 正在整合真实世界资产——但谁来保证合规？
CasperGuard 是一个运行在 Casper 网络上的自主 AI 合规代理。
它用 x402 协议按需获取数据，用 AI 评估风险，
然后把合规证明永久写入 Casper 区块链。
让我们看看它是如何工作的。"

**English narration:**
"DeFi is integrating real-world assets — but who verifies compliance?
CasperGuard is an autonomous AI agent on Casper Network.
It fetches data on-demand via x402, runs AI risk assessment,
and records compliance attestations permanently on-chain.
Let me show you how it works."

---

## 【Dashboard 介绍 / DASHBOARD TOUR】0:30–1:30

**画面：** 鼠标慢慢滑过 Dashboard 各区块

**旁白：**
"这是 CasperGuard 的实时仪表盘。
顶部显示：监控了3个 RWA 资产，链上已有3条合规证明，
发现1个 CRITICAL 风险警报，预言机信誉评分100%。

左侧的合规证明表格显示每个资产的：
- 风险等级徽章（LOW / MEDIUM / HIGH / CRITICAL）
- AI 评分进度条
- 具体风险标记
- 可点击的链上 Deploy Hash，直接跳转到 Casper Testnet Explorer

右侧显示预言机的链上信誉评分，以及最近的 Agent 活动。"

---

## 【运行 Agent / RUN THE AGENT】1:30–3:00

**画面：** 切换到终端，运行 Python agent

```bash
$ python agent/casperguard_agent.py
```

**旁白：**
"现在我们实际运行 Agent。

首先，Agent 通过 x402 协议向数据提供商发请求。
服务器返回 402 Payment Required——
Agent 自动签名一笔 0.001 CSPR 的微支付，
然后重新请求，收到数据。全程自主，无需人工干预。

接着 AI 引擎开始评估：
- RWA-001（迪拜房产基金）——LOW 风险，直接通过
- RWA-002（新加坡黄金仓库）——LOW 风险，通过
- RWA-003（BVI 离岸实体）——注意！
  高风险司法管辖区、发行人未验证、AML 金额超标——CRITICAL！

三条评估结果立即发布到 Casper 测试网。"

**画面：** 终端显示 deploy hash，然后切换到浏览器

---

## 【链上验证 / ON-CHAIN VERIFICATION】3:00–4:00

**画面：** 打开 https://testnet.cspr.live，粘贴 deploy hash

**旁白：**
"我们把刚才的 Deploy Hash 粘贴到 Casper Testnet Explorer。

可以看到：
- 合约调用：ComplianceOracle.record_attestation
- 资产：RWA-003
- 风险等级：CRITICAL
- 证明哈希：[显示 hash]
- 时间戳：[显示时间]

这条记录已永久上链，任何人都可以验证。
DeFi 协议在接受 RWA-003 之前，只需查询这个合约——
就能知道这个资产已被 AI 标记为高风险。"

---

## 【智能合约 / SMART CONTRACT】4:00–4:30

**画面：** 快速展示 compliance_oracle.rs 代码

**旁白：**
"底层智能合约用 Odra 框架编写，Rust 语言，
部署在 Casper Testnet 上。

核心功能：
- record_attestation：AI Agent 写入合规证明
- get_oracle_reputation：查询预言机信誉分
- authorize_oracle：DAO 可以授权/吊销新的 AI Agent

合约完全开源，支持升级，无需重新部署。"

---

## 【总结 / SUMMARY】4:30–5:00

**画面：** 回到 Dashboard，点击 'Run Agent' 按钮，动画效果

**旁白：**
"CasperGuard 将 Agentic AI + DeFi + RWA + Casper 完美结合：
- x402 实现了 Agent 的经济自主性
- AI 引擎提供可扩展的合规判断
- Casper 区块链提供不可篡改的信任基础

这就是 CasperGuard——Casper 上的 RWA 合规预言机。
感谢观看！"

---

## 【录制建议 / RECORDING TIPS】

**工具推荐：**
- OBS Studio（免费录屏）
- 或 Loom（在线录制，直接获得链接）

**录制顺序：**
1. 先打开 frontend/index.html（全屏浏览器）
2. 打开终端，准备好命令但不要运行
3. 开始录制
4. 按脚本顺序演示

**上传：**
- 上传到 YouTube（公开或不公开均可）
- 复制链接填入 DoraHacks 演示视频栏
