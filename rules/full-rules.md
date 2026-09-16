# 强制规则全文（full-rules）

> **权威文本**。所有项目、所有 AI 一律遵守，冲突时以本文为准。
> 精简版见 `rules/custom-prompt-v3-slim.md`（常驻提示词用）。
> 说明：本文件由 2026-09-16 从各项目 `PROJECT_RULES.md` 与主人明确交代中**归纳合成**，后续以主人新指令为准。

---

## 一、工作流铁律（最高优先级）

1. **编译在仓库跑，本地不编译**
   - Android App → `.github/workflows/apk.yml`
   - 服务端 → `.github/workflows/ci.yml`
   - 本地不装 Android SDK/NDK、不探测工具链

2. **推送纪律**
   - 本地验证全绿 → **给主人看** → 主人确认 → 才 `push`
   - 不经此三步直接 push 视为违规

3. **改前备份**
   - 任何文件修改前 `cp x x.bak-日期`
   - >500 行文件分块读写

4. **失败必沉淀**
   - 按「失败 → 根因 → 应对」三段式，记入项目 `PROJECT_RULES.md` 末尾表格

---

## 二、大师思维（主人钦定 2026-09-16，所有事情默认遵守）

### 两个必加载技能
| 技能 | 内容 |
|---|---|
| `karpathy-skills` | 先想后写 / 简洁优先 / 外科手术式修改 / 目标驱动 |
| `linus-torvalds-skill` | 评审优先级：正确性 > API 稳定 > 安全 > 实测性能 > 简洁 > 风格 > 可 bisect |

### 应用方式
- **先结论后细节**；能一句话说清就不写三句
- **列假设**，不确定就一次问清（不要连问多轮）
- **只改该改的**，不重构无关代码
- 每次交付**先定验收标准**，再动手
- 出方案前先问「**是不是根本不用写**」，优先复用已有实现
- 被问「要多久」→ 直接给判断依据 + 结论，**不堆探测命令**

### 已纠正三次、不许再犯
1. ❌ 先查本地能否编译 → ✅ 编译交给 CI
2. ❌ 过度侦察代替思考 → ✅ 直接给判断
3. ❌ 堆代码代替简洁方案 → ✅ 先问是否根本不用写

---

## 三、安全红线

1. **订阅链接只存本机** —— App 私有目录，禁写日志、禁上传、禁提交
2. **流量红线** —— 默认并发 ≤8；每节点每轮探活 ≤3 个小请求；测速默认关；IP 查询按出口去重 + 24h 缓存
3. **权限最小化** —— 只用必需权限（如 INTERNET）
4. **密钥处理** —— 见 `docs/COOMI-BACKUP-FORMAT.md`（v1.2 单明文包，仅本机保存）
5. **GPL 合规** —— 打包第三方二进制必须在仓库显著位置放 LICENSE-attribution + 源码链接
6. **SSH 私钥** —— 任务完成后从工作区删除

---

## 四、质量门禁

1. 每次 Web/JS 改动跑 `python3 scripts/preflight.py`，全绿才算完成
2. 编译前过 Skills 检查门禁（webapp-testing / dom-static-check / debugger 按改动命中）
3. 图标**程序化生成** —— 禁手工改 PNG，跑 `scripts/gen_icon.py` 重生成后给主人确认
4. 版本号**必须同步** —— 改功能同步改 `versionName` + `versionCode`

---

## 五、全量备份指令

**触发**：主人说「备份」/「导出」/「打包」→ 直接执行全量备份，不问范围。

**格式**：v1.2 单明文包（含密钥，不分包）。

**打包范围 10 项**：渐进式记忆 / 全部文档 / `.coomi/config` / `.coomi/sessions` / 完整 MCP / SSH 私钥 / Skills / hooks+核心脚本 / 其余项目源码 / 密钥文件。

**排除**：`.node-install`、`runtime-v2/versions` rootfs、node/apk/flyctl 二进制、`cache/`、`node_modules/`、`__pycache__/`、`.git/`、旧备份包、日志杂物。

**流程**：Python `zipfile`（**禁系统 zip，中文路径崩**）→ `/workspace` 相对路径 → 读回校验 + SHA256 抽验 → `request_file_export` 交付 → 命名 `coomi-workspace-full-<时间>.zip` 落 `backup-full/`。

> 详见 `docs/COOMI-BACKUP-FORMAT.md`

---

## 六、对话记忆规范（渐进式记忆 V3）

| 时机 | 动作 |
|---|---|
| **开轮** | 调 `dialogue-lifecycle.js context` → 只把返回的 `inject` 当记忆 |
| **收轮** | 调 `append` 落原文；`summary_due=true` 时润色或草稿直落 |
| **补漏记** | 看到 `.missing_turns` → 补 append → 删标记 |

**禁止**：AI 不得自行猜测对话继承关系（new/inherit/continue），用户未明确时必须先问。

> 详见渐进式记忆 `docs/v3/README.md`

---

## 七、文档纪律

1. **重要文档收进项目目录内自包含**（工作区曾被多次重置清空）
2. **全局索引** `INDEX_ALL.md` —— 新增/移动文件后同步更新
3. **单一权威口径** —— 同主题多版本时明确标注哪个是权威（如 v3 覆盖 v1/v2）

---

*版本：v1.0 · 创建并归入本仓库：2026-09-16 · 权威文本*
