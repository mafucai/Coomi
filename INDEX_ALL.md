# 🗂️ 全局文档索引 INDEX_ALL.md

> **用途**：任何 AI 接手本项目，**先读本文档**，即可定位全部文件，不必再到处搜索。
> **生成时间**：2026-09-16
> **覆盖范围**：本机全部已知资产（GitHub 仓库 + 本地 zip + inbox 文件）
> **维护规则**：新增/移动文件后，更新对应章节即可。

---

## 0. 最快上手（30 秒版）

| 我要做什么 | 看哪里 |
|---|---|
| 了解 Coomi 项目 | `Coomi/README.md` |
| **构建 Android APK** | `Coomi/tools/mobile-build/README.md`（**APP 文档 2.0**） |
| 看 RelayScope 规格 | `api-relay-tester/docs/RELAYSCOPE-APP-SPEC.md` |
| 看金融项目 | `_extracted/可视化系统文档/finance-v2__HANDOFF.md` |
| 看渐进式记忆 V3 | `_extracted/渐进式记忆-v3-20260911/novel-kg-compressor/docs/v3/README.md` |
| 找某个文档在哪 | 本文档 §2 文档地图 |

---

## 1. 资产总览（三个来源）

```
源1 GitHub (账号 mafucai)          → /tmp/ghtest/          4 仓库 / 41 份文档
源2 本地 zip (inbox)               → /workspace/inbox/_extracted/  5 个 zip
源3 inbox 根目录                    → /workspace/inbox/     小说/记忆/技能库
```

### 源1：GitHub 仓库（真源，SSH 可 clone）

| 仓库 | 类型 | 文档数 | clone 地址 |
|---|---|---|---|
| **Coomi** | 主项目（Android+Rust+Web） | 18 | `git@github.com:mafucai/Coomi.git` |
| **api-relay-tester** | RelayScope 安卓 App | 15 | `git@github.com:mafucai/api-relay-tester.git` |
| **PureProbe** | 节点体检 App | 8 | `git@github.com:mafucai/PureProbe.git` |
| **futures-terminal** | 期货终端 App | **0** ⚠️ | `git@github.com:mafucai/futures-terminal.git` |

> ⚠️ 注意：`futures-terminal` 在 GitHub 上**零文档**，其文档只在 `可视化系统文档.zip` 里。

### 源2：本地 zip（inbox，共 5 个）

| zip | 大小 | 内容 | 解压位置 |
|---|---|---|---|
| `final-gap-20260911.zip` | 1.9M | 金融系统+记忆系统+文档考古层（792 文件） | `_extracted/final-gap-20260911/` |
| `渐进式记忆-v3-20260911.zip` | 229K | **渐进式记忆 V3 完整版** | `_extracted/渐进式记忆-v3-20260911/` |
| `可视化系统文档.zip` | 12.2K | **金融交接文档 + 文档地图** | `_extracted/可视化系统文档/` |
| `final3-tasks.zip` | 948B | 任务事件/元数据 | `_extracted/final3-tasks/` |
| `ssh-keys-20260911.zip` | 1.9K | SSH 私钥（⚠️ 敏感） | ⚠️ 见 §5 安全 |

---

## 2. 文档地图（按主题）

### 2.1 Coomi 主项目（GitHub: Coomi）

| 文档 | 路径 | 大小 | 说明 |
|---|---|---|---|
| ⭐ **APP 文档 2.0** | `tools/mobile-build/README.md` | 2.1K | **手机端构建权威入口**（见 §3） |
| 项目总览 | `README.md` | 6.6K | Coomi 是什么 |
| 提示词清单 | `docs/prompts-inventory.md` | 14.1K | 全部提示词盘点 |
| 反馈指南 | `docs/coomi-feedback-guide.md` | 5.9K | 用户反馈流程 |
| Rust 内核 | `apps/coomi-rs/README.md` | 5.4K | coomi-rs 架构 |
| 自定义迭代 | `apps/coomi-rs/catalogs/coomi-custom-iteration.md` | 12.0K | 迭代机制 |
| 运行环境 | `apps/coomi-rs/catalogs/runtime-environments.md` | 2.1K | Runtime V2 |
| 技能创建 | `apps/coomi-rs/catalogs/skill-creator.md` | 236B | 技能创建指引 |
| 运行时 v2 | `docs/runtime-v2.md` | 1.6K | Runtime V2 说明 |
| 社区 | `docs/community.md` | 2.0K | 社区信息 |
| 踩坑记录 | `docs/mistakes/termux-bootstrap-permission-denied.md` | 2.8K | Termux 权限坑 |
| 版本说明 ×6 | `RELEASE_NOTES_v1.3.0~v1.4.5.md` | ~1K each | 版本历史 |

**配套脚本**（`tools/mobile-build/`）：
```
build-coomidev.sh              主构建脚本
install-coomidev-buildkit.sh   Build Kit 安装器（SHA-256 校验）
coomidev-doctor.sh             环境体检
coomidev-env.sh                环境变量
buildkit-manifest.schema.json  manifest schema
```

**App 代码结构**：
```
apps/coomi-app/    Android App（terminal-emulator / terminal-view / termux-shared）
apps/coomi-rs/     Rust 内核（engine/security/services/telemetry/tools/ui）
apps/web/          前端（Vue：bridge/components/protocol/router/stores/views）
```

### 2.2 RelayScope 安卓 App（GitHub: api-relay-tester）

| 文档 | 路径 | 大小 |
|---|---|---|
| 工程规范 | `docs/ENGINEERING.md` | 16.3K |
| ⭐ 统一规格 v2.0 | `docs/RELAYSCOPE-APP-SPEC.md` | 13.1K |
| 当前版本 | `docs/CURRENT-VERSION.md` | 6.2K |
| 项目 README | `README.md` | 3.1K |
| 项目铁律 | `PROJECT_RULES.md` | 3.1K |
| 验收标准 | `ACCEPTANCE.md` | 2.2K |
| 风险清单 | `RISK_CHECKLIST.md` | 2.0K |
| 任务模板 | `LOW_MODEL_TASK_TEMPLATE.md` | 2.6K |
| ⭐ **渐进式记忆 V3 存档** | `docs/archive/v3-progressive-memory/README.md` | 11.4K |
| 渐进式记忆 v2 存档 | `docs/archive/v2-progressive-memory/`（5 份） | — |

### 2.3 PureProbe 节点体检 App（GitHub: PureProbe）

| 文档 | 路径 | 大小 |
|---|---|---|
| 交付报告 | `docs/DELIVERY-REPORT.md` | 7.7K |
| 项目铁律 | `PROJECT_RULES.md` | 6.7K |
| 架构 | `docs/ARCHITECTURE.md` | 5.3K |
| 风险清单 | `RISK_CHECKLIST.md` | 2.2K |
| 验收标准 | `ACCEPTANCE.md` | 1.8K |
| 记忆存档 | `docs/memory-pureprobe-archived.md` | 1.8K |
| 任务模板 | `LOW_MODEL_TASK_TEMPLATE.md` | 1.2K |
| 第三方许可 | `THIRD-PARTY-LICENSES.md` | 1.1K |

### 2.4 金融项目（期货终端） ⚠️ 文档不在 GitHub

| 文档 | 路径 | 大小 |
|---|---|---|
| ⭐ **完整交接文档** | `_extracted/可视化系统文档/finance-v2__HANDOFF.md` | 18.4K |
| 文档地图 | `_extracted/可视化系统文档/DOCS_INDEX.md` | 4.2K |
| 导出说明 | `_extracted/可视化系统文档/导出说明.txt` | 2.5K |
| 金融系统代码备份 | `_extracted/final-gap-20260911/backups-v2/finance-v2-v1.0-full-20260810/` | — |
| 旧文档 | `backups-v2/old-docs/FINANCE_PLAN.md` 等 3 份 | — |

### 2.5 渐进式记忆 V3（本地 zip）

| 文档 | 路径 | 大小 |
|---|---|---|
| ⭐ **权威文档** | `_extracted/渐进式记忆-v3-20260911/novel-kg-compressor/docs/v3/README.md` | 11.4K |
| 脚本 ×11 | `.../novel-kg-compressor/scripts/` | — |
| 构建器 | `.../novel-kg-compressor/minimal_build.js` | 30.2K |
| 测试对话 ×3 | `.../渐进式记忆/dialogues/` | — |

### 2.6 小说记忆（inbox 根）

| 文件 | 大小 | 说明 |
|---|---|---|
| `index.txt` | 19.3K | 《神秘复苏》385 实体索引 |
| `block_001~101.txt` | 各 1-1.7K | 101 块记忆 |
| `神秘复苏_utf8.txt` | 16.4M | 原文（1608 章） |
| `神秘复苏_记忆摘要.txt` | 1.3K | 前30章摘要 |
| `神秘复苏.NOVEL_MEMORY.txt` | 2.7K | 30/345章缩略（旧格式） |
| `捞尸人(1-500章).txt` | 13.7M | 原文（500章） |
| `捞尸人_utf8.txt` | 4.1M | ⚠️ 双重编码损坏，无价值 |

### 2.7 技能库（inbox 根）

| 文件 | 大小 | 说明 |
|---|---|---|
| `ALL_SKILLS.md` | 168K | **68 个技能完整原文** |
| `skills300.md` | 18.6K | 300+ 技能清单（仅目录） |
| `build_skills_lib.sh` | 3.6K | 技能库生成脚本 |

### 2.8 元数据 / 日志

| 文件 | 大小 | 说明 |
|---|---|---|
| `.origins.jsonl` | 7.1K | 文件来源记录（20+ 条） |
| `task_checkpoints.json` | 1.2K | 任务检查点（7 条） |
| `conversation_log.txt` | 82K | ⚠️ 1000 轮，**仅前 14 轮真实，15 轮起模板循环** |

---

## 2.9 代码结构（GitHub 4 仓库，869 文件）

> 逐个文件列会淹没索引，此处给**目录级地图**。找具体文件用 `find /tmp/ghtest -name "*关键词*"`。

### Coomi（712 文件）— 主项目

| 子目录 | 内容 | 主要语言 |
|---|---|---|
| `apps/coomi-app/` | Android 主 App | Java 244 / XML 223 |
| `apps/coomi-app/app/src/` | App 源码 | Java |
| `apps/coomi-app/terminal-emulator/` | 终端模拟器 | Java/C |
| `apps/coomi-app/terminal-view/` | 终端视图 | Java |
| `apps/coomi-app/termux-shared/` | Termux 共享库 | Java |
| `apps/coomi-rs/` | Rust 内核 | Rust 39 |
| `apps/coomi-rs/engine/` `security/` `services/` `telemetry/` `tools/` `ui/` | 各模块 | Rust |
| `apps/web/` | 前端 | Vue 42 / TS 29 |
| `apps/web/src/` | bridge/components/protocol/router/stores/views | Vue |
| `deploy/` `scripts/` `tools/` | 部署与工具 | Python/Shell |
| `extensions/coomi-life/` | 扩展 | — |
| `gradle/` | Gradle wrapper | — |
| `.github/workflows/` | `sync.yml` `runtime-v2-assets.yml` | YAML |

**根文件**：`build.gradle` `settings.gradle` `gradle.properties` `gradlew` `reasonix.toml`

### api-relay-tester（61 文件）— RelayScope App

| 子目录 | 内容 |
|---|---|
| `app/` | Android App 源码（Java/XML） |
| `docs/` | 文档（见 §2.2） |
| `tools/` | 工具脚本 |
| `gradle/` | wrapper |
| `.github/workflows/apk.yml` | APK 构建 |

### PureProbe（53 文件）— 节点体检 App

| 子目录 | 内容 |
|---|---|
| `app/` | Android App 源码 |
| `docs/` | 文档（见 §2.3） |
| `preview/` | 预览资源 |
| `scripts/` | `gen_icon.py` `preflight.py` |
| `.github/workflows/apk.yml` | APK 构建 |

### futures-terminal（43 文件）— 金融 App ⚠️ 无文档

| 子目录 | 内容 |
|---|---|
| `app/` | Android App 源码 |
| `preview/` | 预览资源 |
| `scripts/` | 脚本 |
| `gradle/` | wrapper |
| `.github/workflows/apk.yml` | APK 构建 |

### 代码搜索速查

```sh
# 找任意文件
find /tmp/ghtest -name "*关键词*" -not -path "*/node_modules/*"

# 找代码内容
grep -rn "关键词" /tmp/ghtest --include="*.java" --include="*.rs" --include="*.vue"

# 按语言找
find /tmp/ghtest -name "*.rs" -not -path "*/node_modules/*"     # Rust 39
find /tmp/ghtest -name "*.java" -not -path "*/node_modules/*"   # Java 244
```

---

## 3. ⭐ APP 文档 2.0（手机端构建）— 重点

**位置**：`Coomi/tools/mobile-build/README.md`

**定位**：Runtime V2 内构建隔离 CoomiDev APK 的**唯一受支持 Linux ARM64 入口**。

**运行时布局**：
```
/opt/coomi-dev/
├── bin/          辅助脚本
├── current/      已校验 Build Kit
├── toolchains/   不可变版本
├── cache/        Gradle/Cargo 缓存
├── state/        安装器状态
├── logs/         构建日志
└── keys/         签名密钥（禁提交/打印）
```

**安装 Build Kit**（强制 SHA-256）：
```sh
coomidev-install-buildkit /path/to/buildkit.tar.gz EXPECTED_SHA256
```

**四阶段验证**（按序）：
```sh
coomidev-build doctor          # 体检
coomidev-build android-smoke   # Android 冒烟
coomidev-build rust-smoke      # Rust 冒烟
coomidev-build full            # 完整构建
```

**硬规矩**：
- 原生产物必须 Linux AArch64/glibc
- 官方 Android SDK/NDK 是 x86_64，**不得当 ARM64 用**
- Termux Bionic 可执行文件**永不许进 guest PATH**
- **拿不到 Build Kit 时 → GitHub Actions 是受支持 fallback**

**各 App 的 CI workflow**：
```
Coomi/.github/workflows/             sync.yml + runtime-v2-assets.yml
PureProbe/.github/workflows/apk.yml       ✅
api-relay-tester/.github/workflows/apk.yml ✅
futures-terminal/.github/workflows/apk.yml ✅
```

---

## 4. 待补齐的文档（GitHub 上缺失）

`DOCS_INDEX.md` 提到但 GitHub 上**找不到**的：

| 文档 | 状态 |
|---|---|
| `docs/APP-TEMPLATE-GITHUB-ACTIONS.md` | ❌ 缺失（App 模板化流程） |
| `docs/ANDROID-APP-GITHUB-ACTIONS-BUILD.md` | ❌ 缺失（构建坑清单） |
| `docs/COOMI-BACKUP-FORMAT.md` | ❌ 缺失（自制备份格式规范） |
| `rules/full-rules.md` | ❌ 缺失（铁律全文） |
| `rules/custom-prompt-v3-slim.md` | ❌ 缺失（精简规则） |
| `futures-terminal` README + docs | ❌ 缺失（金融 App 零文档） |

---

## 5. ⚠️ 安全提醒

| 项 | 说明 | 处置 |
|---|---|---|
| `ssh-keys-20260911.zip` | SSH 私钥（`id_ed25519`），可访问 GitHub `mafucai` | **任务完成后删除** |
| `Coomi-Android-arm64-v1.4.5*.apk` ×2 | 各 191M，占 89% 空间 | 可删 |
| `捞尸人_utf8.txt` | 双重编码损坏，乱码 | 可删 |
| `conversation_log.txt` | 98% 是伪造模板循环 | 可删 |

---

## 6. 🤖 给 AI 的操作规范（必读）

### 6.1 接手任务的第一步

1. **读本文档**（`INDEX_ALL.md`）→ 知道东西在哪
2. 按 §0「最快上手」跳到目标文档
3. 需要代码 → 用 §2.9 的搜索速查
4. **不要**在没有目标时全盘 find

### 6.2 环境事实（避免重复踩坑）

| 事实 | 说明 |
|---|---|
| GitHub 账号 | `mafucai`，**4 个公开仓库** |
| SSH 认证 | ✅ **可用**（`Hi mafucai!`），密钥在 `ssh-keys-20260911.zip` |
| SSH 端口 | 走 **443**（config 里 `HostName ssh.github.com`） |
| API 限流 | GitHub API 匿名调用**易限流**，优先用 SSH clone / 网页抓取 |
| 编译位置 | **不在本地编译**，走 GitHub Actions（各仓库 `apk.yml` / `ci.yml`） |
| 沙箱限制 | 看不到手机 `/storage`，只有 `/workspace`(=inbox) |

### 6.3 常用命令

```sh
# clone 仓库（SSH 443）
git clone --depth 1 git@github.com:mafucai/Coomi.git

# 解压任意 zip
unzip -o -q xxx.zip -d _extracted/xxx

# 查索引
grep -n "关键词" /workspace/inbox/INDEX_ALL.md
```

### 6.4 工作流铁律（主人钦定）

1. **编译在仓库跑**，本地不编译
2. 被问"要多久"→ 直接给判断依据 + 结论，不堆探测命令
3. 出方案前先问「**是不是根本不用写**」，优先复用
4. 先结论后细节；列假设；只改该改的

---

## 7. 索引维护指引

新增文件时，按以下规则更新本文档：

1. **GitHub 仓库新增文档** → 更新 §2.1~2.3 对应表
2. **新增 zip / 解压** → 更新 §1 源2 表 + 对应主题章节
3. **新增仓库** → 更新 §1 源1 表 + §2.9 代码结构
4. **补齐 §4 缺失文档** → 从 §4 移到对应主题章节
5. **删除文件** → 同步删除索引条目，避免指向不存在路径

---

*最后更新：2026-09-16 · 覆盖 3 源 / 4 仓库 / 5 zip / 869 代码文件 / 41 文档*
