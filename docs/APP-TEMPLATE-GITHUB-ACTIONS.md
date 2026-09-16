# App 模板化流程（GitHub Actions 标准路线）

> **主人钦定（2026-09-11）** — 新建任何 Android App 一律走本流程，不另起炉灶。
> 状态：生效中

---

## 1. 核心原则

| 原则 | 说明 |
|---|---|
| **云端编译** | 本地零 Android SDK，编译只在 GitHub Actions |
| **模板复用** | 从已有 App 复制结构，不重造 |
| **单仓库单 App** | 一个 App 一个 GitHub 仓库 |
| **首推即构建** | push 到 main 自动触发 `apk.yml` |

---

## 2. 模板资产来源（`PureProbe` 是母版）

| 资产 | 路径 | 用途 |
|---|---|---|
| **APK workflow** | `PureProbe/.github/workflows/apk.yml` | 云端构建 + 签名 + Release 模板 |
| **图标生成器** | `PureProbe/scripts/gen_icon.py` | PIL 程序化生成，新项目复制改配色 |
| **preflight 脚本** | `PureProbe/scripts/preflight.py` | 治理检查模板 |
| **项目铁律** | `PureProbe/PROJECT_RULES.md` | 铁律 + 失败台账格式 |
| **验收标准** | `PureProbe/ACCEPTANCE.md` | 验收模板 |
| **风险清单** | `PureProbe/RISK_CHECKLIST.md` | 风险模板 |
| **任务模板** | `PureProbe/LOW_MODEL_TASK_TEMPLATE.md` | 低模型任务下发模板 |

---

## 3. 新建 App 步骤

### 步骤 1：建仓库
```
github.com/mafucai/<AppName>
```
单仓库、单 App、`main` 为主分支。

### 步骤 2：复制治理四件套
```
PROJECT_RULES.md            ← 改项目名 + 铁律
ACCEPTANCE.md               ← 改验收标准
RISK_CHECKLIST.md           ← 改风险项
LOW_MODEL_TASK_TEMPLATE.md  ← 直接复用
```

### 步骤 3：复制构建资产
```
.github/workflows/apk.yml   ← 改 App 名 + 产物名
scripts/gen_icon.py         ← 改配色
scripts/preflight.py        ← 直接复用
gradle/ gradlew gradlew.bat ← 直接复用
build.gradle settings.gradle gradle.properties
```

### 步骤 4：配置签名 secrets
仓库 Settings → Secrets and variables → Actions：
```
KEYSTORE_BASE64       # keystore 文件 base64
KEYSTORE_PASSWORD     # 密码
KEYSTORE_ALIAS        # 别名
```

### 步骤 5：写 docs/
```
docs/ARCHITECTURE.md        ← 架构设计
docs/DELIVERY-REPORT.md     ← 交付报告
```

### 步骤 6：首推验证
本地验证全绿 → **给主人看** → 主人确认 → `push` 触发 Actions。

---

## 4. 标准目录结构

```
<AppName>/
├── .github/workflows/apk.yml     ← 云端构建（模板复制）
├── app/                          ← Android App 源码
│   └── src/main/
│       ├── java/.../             ← Java 源码
│       ├── assets/               ← WebView UI（若有）
│       ├── res/                  ← 资源 + mipmap 图标
│       └── jniLibs/              ← 原生二进制（若有）
├── docs/                         ← 文档
│   ├── ARCHITECTURE.md
│   └── DELIVERY-REPORT.md
├── scripts/
│   ├── gen_icon.py               ← 图标生成
│   └── preflight.py              ← 治理检查
├── gradle/ gradlew gradlew.bat
├── build.gradle settings.gradle gradle.properties
├── PROJECT_RULES.md              ← 治理四件套
├── ACCEPTANCE.md
├── RISK_CHECKLIST.md
├── LOW_MODEL_TASK_TEMPLATE.md
└── README.md
```

---

## 5. 已按此流程落地的 App

| App | 仓库 | 状态 | 备注 |
|---|---|---|---|
| **PureProbe** | `mafucai/PureProbe` | ✅ 活跃 v0.3.0 | **母版** |
| **RelayScope** | `mafucai/api-relay-tester` | ✅ | 含 SPEC/ENGINEERING 规范 |
| **futures-terminal** | `mafucai/futures-terminal` | ⚠️ 无文档 | 金融项目，待补 docs |

---

## 6. 硬规矩

1. **不本地编译** —— 见 `docs/ANDROID-APP-GITHUB-ACTIONS-BUILD.md`
2. **模板优先** —— 先看母版能不能直接用，不重复造
3. **推送纪律** —— 本地全绿 → 主人确认 → 才 push
4. **失败必沉淀** —— 按「失败 → 根因 → 应对」记入 `PROJECT_RULES.md`
5. **图标程序化** —— 禁手工改 PNG，跑 `gen_icon.py` 重生成

---

*创建并归入本仓库：2026-09-16 · 主人钦定标准路线*
