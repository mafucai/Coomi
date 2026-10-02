# Android App · GitHub Actions 构建清单（含失败台账）

> 定位：自研 Android App 的**云端构建标准流程 + 踩坑记录**。
> 铁律：**编译只在 GitHub Actions 云端，本地零 Android SDK。**
> 来源：PureProbe / api-relay-tester / futures-terminal 三仓库实测（`apk.yml`）。

---

## 1. 标准 workflow 模板（已验证）

```yaml
name: <App> APK

on:
  push:
    branches: [ main ]
  workflow_dispatch:

permissions:
  contents: write          # 发布 Release 必需

jobs:
  apk:
    runs-on: ubuntu-24.04  # 固定版本，不用 ubuntu-latest
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-java@v4
        with:
          distribution: temurin
          java-version: '17'          # AGP 要求
      - uses: android-actions/setup-android@v4   # v3 会因 tools 包下线失败，见失败台账 #9
      - name: Install Android platform
        run: |
          set -euo pipefail
          SDKMANAGER="$(find "${ANDROID_SDK_ROOT:-${ANDROID_HOME:-}}" -path '*/cmdline-tools/*/bin/sdkmanager' -type f | sort | tail -n 1)"
          test -n "$SDKMANAGER"
          sdkmanager 'platforms;android-35' 'build-tools;35.0.0'
      - name: Build release APK
        run: ./gradlew --no-daemon --max-workers=2 :app:assembleRelease
      # ... 签名 + 验证 + 上传 + Release（见 §3）
```

---

## 2. 检查清单（每次构建前过一遍）

- [ ] `runs-on` 用固定 `ubuntu-24.04`
- [ ] Java 17（temurin）
- [ ] **`android-actions/setup-android@v4`**（v3 已废弃，见失败台账 #9）
- [ ] Android platform 35 + build-tools 35.0.0 显式安装
- [ ] `./gradlew --no-daemon --max-workers=2`（限制并发防 OOM）
- [ ] 签名 secrets 已配：`KEYSTORE_BASE64` / `KEYSTORE_PASSWORD` / `KEYSTORE_ALIAS` / `KEY_PASSWORD`
- [ ] 项目如已有固定签名，先读项目签名资产文档；禁止因无法读取 Secret 明文而重新生成签名
- [ ] **若 App 需内置离线运行时资产**：构建前先下载并校验 SHA256，放入 Gradle 期望的目录（见失败台账 #10）
- [ ] **产物防废包**：体积下限断言 + 打开 APK 核对关键资产存在（见失败台账 #10）
- [ ] 构建产物路径正确：`app/build/outputs/apk/release/*.apk`
- [ ] `apksigner verify` 已加
- [ ] 有 `if-no-files-found: error`（防静默失败）
- [ ] 版本号与代码同步（⚠️ 见失败台账 #1）
- [ ] workflow 若需手动触发，文件必须在**默认分支**上（见失败台账 #12）

---

## 3. 签名步骤（必须验证）

```yaml
- name: Sign release APK
  env:
    KEYSTORE_B64: ${{ secrets.KEYSTORE_BASE64 }}
    STORE_PASS: ${{ secrets.KEYSTORE_PASSWORD }}    # ← 独立 secret：keystore 密码
    KEY_PASS: ${{ secrets.KEY_PASSWORD }}          # ← 独立 secret：key password（重要！不可与 store pass 混用）
    KEYSTORE_ALIAS_NAME: ${{ secrets.KEYSTORE_ALIAS }}
  run: |
    set -euo pipefail
    trap 'rm -f release-fixed.jks SIGNED.apk.tmp' EXIT  # ← 失败也清理，不遗留密钥文件
    SDK_BUILD_TOOLS="${ANDROID_HOME}/build-tools/35.0.0"
    echo -n "${KEYSTORE_B64}" | base64 -d > release-fixed.jks
    test -s release-fixed.jks
    UNSIGNED="app/build/outputs/apk/release/app-release-unsigned.apk"
    test -f "${UNSIGNED}"
    "${SDK_BUILD_TOOLS}/apksigner" sign \
      --ks release-fixed.jks \
      --ks-pass "pass:${STORE_PASS}" \
      --ks-key-alias "${KEYSTORE_ALIAS_NAME}" \
      --key-pass "pass:${KEY_PASS}" \
      --out SIGNED.apk.tmp "${UNSIGNED}"
    mv SIGNED.apk.tmp SIGNED.apk
    "${SDK_BUILD_TOOLS}/apksigner" verify SIGNED.apk
```

> ⚠️ **两个密码必须分开**：`KEYSTORE_PASSWORD`（keystore 库口令）与 `KEY_PASSWORD`（key 口令）在 `keytool` 里本就独立。
> 若把 `--key-pass` 也填成 store 口令，遇到"库口令 ≠ key 口令"的 keystore 会**签名失败**。
> `trap ... EXIT` 保证**失败路径也清理** `release-fixed.jks`（内含私钥），不能让它在 runner 上残留。

---

## 4. 已登记的固定签名资产

| 项目 | 状态 | 权威说明 |
|---|---|---|
| `mafucai/futures-terminal` | 已启用固定签名；私钥备份位于独立 Private 仓库 | `/workspace/repos/futures-terminal/docs/ANDROID_SIGNING.md` |

> 所有 AI：这里只登记资产存在及文档入口。不得把密码、Base64、Token 或私钥正文复制到公共文档；不得重新生成签名替换已有密钥。

---

## 4.1 内置离线运行时资产（Coomi 类 App 专用）

适用：App 需把整套 Linux 运行时（PRoot host + rootfs）打进 APK，实现离线可用。

**关键约束**：仓库 `.gitignore` 排除了 `runtime-v2-dist/`，构建产物里的环境**不会自动出现**。缺了它 APK 会小 300MB+，装上没有内置环境（失败台账 #10）。

### 做法

1. 把运行时资产托管到**固定 URL**（GitHub Release 或自有 CDN），便于 CI 复用。
   - 本仓库实测可用：`mafucai/coomi-runtime-assets`（公开，从官方 APK 提取，SHA256 逐字节校验）
2. 构建前下载 → **校验 SHA256 与大小** → 放进 Gradle 期望目录（Coomi 为 `runtime-v2-dist/`，文件名必须是 `proot-host-arm64.tar.gz` / `ubuntu-noble-arm64.tar.gz`）。
3. Gradle 任务 `stageCoomiRuntimeV2` 会二次校验 size+sha256 并打进 APK；**校验不过会抛异常**，这是正确的保护，不要绕过。
4. 构建后加**双重防废包闸门**：
   - 体积断言：`apk_bytes >= 320000000`
   - 内容断言：`unzip -p` 取出 APK 内 `assets/runtime-v2/*.tgz`，SHA256 必须等于官方值
5. 参考实现：`mafucai/Coomi` 分支 `codex/coomidev-v148-full` 的 `.github/workflows/coomidev-v148-full.yml`
   - 实测结果：355,302,525 字节（官方 355,310,063，差 7,538 字节），全流程 7 分 33 秒
   - **反例**：同一项目历史上不经该步骤构建，产物仅 39MB，属废包

### 不要做

- 不要因校验失败就删掉校验或跳过 `stageCoomiRuntimeV2`
- 不要把 300MB 级资产直接提交进 git（GitHub 单文件上限 100MB）
- 不要把私有签名密钥放进 Runtime 资产仓库

---

## 5. 失败台账（真实记录，持续更新）

| # | 失败现象 | 根因 | 应对 |
|---|---|---|---|
| 1 | 版本号停在旧值，改动未生效 | `app/build.gradle` 版本号字段未同步（PureProbe v0.3.0 改动在 commit f8c0a90，但 versionCode 仍 0.2.7/13） | 每次改功能**同步改 versionName + versionCode**，构建后核对 |
| 2 | 下载内核 404 | mihomo android 资产名是 `mihomo-android-arm64-v8-<版本>.gz`（多了 `-v8`），按猜的 `arm64` 写 URL | **下载前先查 release 真实资产名**（API） |
| 3 | 编译失败：`Process.pid()` 找不到 | `Process.pid()` 是 Java 9 API，Android Process 类没有 | 改用 `proc.destroy()+waitFor(3s)+destroyForcibly()`；**本地无 javac，写 Android 代码避免 Java 9+ API** |
| 4 | 编译失败：漏 import | `SubStore` 漏 `InputStream` | 补 import |
| 5 | 真机"订阅拉取失败"但实际下载成功 | Java 读内核 API 用 `http://127.0.0.1` 被 Android **明文策略拦截**（回环也拦） | `network_security_config` 白名单 `127.0.0.1`/`localhost`/`ip-api.com`；**targetSdk > 23 默认禁明文，回环需白名单** |
| 6 | 真机"全部不通"但代理正常 | Java SOCKS 代理**本地解析 DNS**（域名被污染→假 IP→直连失败→全标 dead） | 探活改走 HTTP CONNECT（域名由内核远程解析）；**Java SOCKS ≠ 远程 DNS** |
| 7 | 桥回调报 `[object Object] is not valid JSON` | 锁跨 20s 网络等待持锁 + `evaluateJavascript` 拼 JSON 无引号 | 网络等待零持锁；Java 侧 `quote()` 转义；**桥回调必须引号包裹，锁永不跨网络** |
| 8 | "体检完成但已测 0" | 前端 `onDone` 不重拉数据 | 进度回调实时带结果 + `onDone` 重拉全量 |
| 9 | 构建在 `Setup Android SDK` 步骤 30 秒失败：`Warning: Failed to find package 'tools'` → `sdkmanager failed with exit code 1` | `android-actions/setup-android@v3` 内部会 `sdkmanager tools`，而 Google 已不再提供 `tools` 包（该 action README 承认此问题，**v4 已修**：不再默认请求 `tools`，显式请求时只警告不失败） | **把 `setup-android` 固定为 `@v4`**；`sdkmanager` 用 `find ... \| sort \| tail -n1` 取最新版，取不到就显式报错 |
| 10 | APK 只有 39MB，装上去没有内置环境（"废包"） | 构建只跑 `gradlew :app:assembleRelease`，未把 `runtime-v2-dist/` 的 proot host 与 rootfs 放进去；Gradle 任务 `stageCoomiRuntimeV2` 依赖该目录且含 size+sha256 硬校验，资产缺失时**若未被触发则静默出包** | **先下载 runtime 资产并校验 SHA256 再构建**；产物加**双重防废包闸门**：① APK ≥ 320000000 字节 ② 打开 APK 核对 `assets/runtime-v2/*.tgz` 的 SHA256 等于官方值 |
| 11 | 签名口令缺失导致 Gradle 抛异常，或每次构建签名不同装不上 | `app/build.gradle` 的 `signingConfigs.debug` 从 `local.properties` 读 `coomi.signing.storePassword` / `keyPassword`，缺失即 `throw`；CI 里无该文件 | CI 中 `openssl rand -hex 16` 生成一次性口令 + `keytool -genkeypair` 造临时 keystore + 写 `local.properties`；**代价**：每次构建签名不同，装新版前需先卸载旧预览包 |
| 12 | `workflow_dispatch` 报 `HTTP 404: Not Found`，无法手动触发 | GitHub 只索引**默认分支**上的 workflow 文件；新 workflow 若只存在于特性分支，则不在 dispatch 索引中 | 把 workflow 文件放到默认分支（main）后即可手动触发；或先靠 `on.push.branches` 触发一次 |

---

## 6. 硬规矩（铁律）

1. **编译只在云端** —— 本地不装 Android SDK/NDK
2. **本地验证全绿 → 给主人看 → 确认后才 push**（不经此流程直接 push 视为违规）
3. **改前备份** —— `cp x x.bak-日期`
4. **推送即触发 Actions** —— 想清楚再 push
5. 每次改动跑项目 `scripts/preflight.py`，全绿才算完成建

---

## 7. 与其他文档的关系

| 文档 | 说明 |
|---|---|
| `tools/mobile-build/README.md` | **本机 ARM64 构建**（APP 文档 2.0，非云端） |
| `docs/APP-TEMPLATE-GITHUB-ACTIONS.md` | App 模板化流程（本项目标准路线） |
| 各仓库 `.github/workflows/apk.yml` | 具体实现 |

---

*创建并归入本仓库：2026-09-16 · 基于 3 个 App 仓库实测*
