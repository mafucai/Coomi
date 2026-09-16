# 精简规则 v3（常驻提示词 slim）

> **用途**：常驻提示词用。铁律摘要 + 指针。
> **全文**：`rules/full-rules.md`（冲突时以全文为准）

---

## 铁律（7 条）

1. **编译在仓库跑** —— Android → `apk.yml`，服务端 → `ci.yml`。本地不编译、不探测工具链。
2. **推送纪律** —— 本地全绿 → 给主人看 → 确认 → 才 push。
3. **改前备份** —— `cp x x.bak-日期`；>500 行分块读写。
4. **失败必沉淀** —— 按「失败→根因→应对」记入 `PROJECT_RULES.md`。
5. **大师思维** —— 先结论后细节；列假设；只改该改的；先定验收标准；先问「是不是不用写」。
6. **安全红线** —— 订阅只存本机；并发 ≤8；权限最小化；密钥只在本机。
7. **质量门禁** —— Web/JS 改动跑 `preflight.py`；图标程序化生成；版本号必须同步。

---

## 大师思维（两技能）

- `karpathy-skills` —— 先想后写 / 简洁优先 / 外科手术式修改 / 目标驱动
- `linus-torvalds-skill` —— 正确性 > API 稳定 > 安全 > 性能 > 简洁 > 风格

---

## 收轮规则（渐进式记忆 V3）

```
收轮 → append 落原文
  ├─ summary_due=true
  │   ├─ AI 在场  → 润色草稿 → save-summary 落库
  │   └─ 缺席/急 → 规则草稿直落：save-summary "第X-Y轮（规则草稿）：<draft.text>"
  └─ 看到 .missing_turns → 补 append → 删标记
```

---

## 指针（详情看这些）

| 主题 | 文档 |
|---|---|
| 规则全文 | `rules/full-rules.md` |
| 备份格式 | `docs/COOMI-BACKUP-FORMAT.md` |
| App 模板 | `docs/APP-TEMPLATE-GITHUB-ACTIONS.md` |
| 构建坑 | `docs/ANDROID-APP-GITHUB-ACTIONS-BUILD.md` |
| 本机构建 | `tools/mobile-build/README.md`（APP 文档 2.0） |
| 全局索引 | `INDEX_ALL.md` |
| 记忆 V3 | 渐进式记忆 `docs/v3/README.md` |

---

*创建并归入本仓库：2026-09-16*
