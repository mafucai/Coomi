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
      - uses: android-actions/setup-android@v3
      - name: Install Android platform
        run: sdkmanager 'platforms;android-35' 'build-tools;35.0.0'
      - name: Build release APK
        run: ./gradlew --no-daemon --max-workers=2 :app:assembleRelease
      # ... 签名 + 验证 + 上传 + Release（见 §3）
```

---

## 2. 检查清单（每次构建前过一遍）

- [ ] `runs-on` 用固定 `ubuntu-24.04`
- [ ] Java 17（temurin）
- [ ] Android platform 35 + build-tools 35.0.0 显式安装
- [ ] `./gradlew --no-daemon --max-workers=2`（限制并发防 OOM）
- [ ] 签名 secrets 已配：`KEYSTORE_BASE64` / `KEYSTORE_PASSWORD` / `KEYSTORE_ALIAS`
- [ ] 构建产物路径正确：`app/build/outputs/apk/release/*.apk`
- [ ] `apksigner verify` 已加
- [ ] 有 `if-no-files-found: error`（防静默失败）
- [ ] 版本号与代码同步（⚠️ 见失败台账 #1）

---

## 3. 签名步骤（必须验证）

```yaml
- name: Sign release APK
  env:
    KEYSTORE_B64: ${{ secrets.KEYSTORE_BASE64 }}
    KEYSTORE_PASS: ${{ secrets.KEYSTORE_PASSWORD }}
    KEYSTORE_ALIAS_NAME: ${{ secrets.KEYSTORE_ALIAS }}
  run: |
    set -euo pipefail
    SDK_BUILD_TOOLS="${ANDROID_HOME}/build-tools/35.0.0"
    echo -n "${KEYSTORE_B64}" | base64 -d > release-fixed.jks
    test -s release-fixed.jks                      # 空包立即失败
    UNSIGNED="app/build/outputs/apk/release/app-release-unsigned.apk"
    test -f "${UNSIGNED}"
    "${SDK_BUILD_TOOLS}/apksigner" sign \
      --ks release-fixed.jks --ks-pass "pass:${KEYSTORE_PASS}" \
      --ks-key-alias "${KEYSTORE_ALIAS_NAME}" \
      --key-pass "pass:${KEYSTORE_PASS}" --out SIGNED.apk "${UNSIGNED}"
    "${SDK_BUILD_TOOLS}/apksigner" verify SIGNED.apk      # 必验
    rm -f release-fixed.jks                               # 清密钥
```

---

## 4. 失败台账（真实记录，持续更新）

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

---

## 5. 硬规矩（铁律）

1. **编译只在云端** —— 本地不装 Android SDK/NDK
2. **本地验证全绿 → 给主人看 → 确认后才 push**（不经此流程直接 push 视为违规）
3. **改前备份** —— `cp x x.bak-日期`
4. **推送即触发 Actions** —— 想清楚再 push
5. 每次改动跑项目 `scripts/preflight.py`，全绿才算完成建

---

## 6. 与其他文档的关系

| 文档 | 说明 |
|---|---|
| `tools/mobile-build/README.md` | **本机 ARM64 构建**（APP 文档 2.0，非云端） |
| `docs/APP-TEMPLATE-GITHUB-ACTIONS.md` | App 模板化流程（本项目标准路线） |
| 各仓库 `.github/workflows/apk.yml` | 具体实现 |

---

*创建并归入本仓库：2026-09-16 · 基于 3 个 App 仓库实测*
