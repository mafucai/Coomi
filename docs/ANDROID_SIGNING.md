# Coomi Android 固定签名资产

> 状态：已启用（2026-10-03）
> 适用应用：CoomiDev
> 应用 ID：`com.coomidev.android`

## 1. 资产说明

本仓库此前每次构建都用 `openssl rand` 现生成 keystore（见 `coomidev-v148-full.yml` 旧版
`Write signing properties` 步骤），导致每个 APK 签名不同、无法覆盖安装，必须卸载重装。
本次为**首次启用固定签名**——不涉及替换任何既有签名。

- 签名容器：`coomi-release.p12`（PKCS#12）
- 密钥别名：`coomi-release`
- 算法：RSA 2048 / SHA-256，有效期 3650 天
- 证书 SHA-256：
  `D7:1B:76:59:19:8C:E2:91:D7:D9:B8:0E:90:46:FB:60:9E:7D:43:4E:62:18:E7:E4:E0:84:94:7A:40:AC:0A:13`
- 签名容器 SHA-256：
  `5b7b893e934333e72e1b8682e5c9b249237a46b70f4aa4e1186b1e8663f69fa8`

签名容器与口令**不得**提交到本仓库、日志、对话或 Release。
备用存放请使用私有仓库 `mafucai/android-signing-backup` 或主人的密码管理器。

## 2. GitHub Actions Secrets

仓库 `mafucai/Coomi` 需配置以下 Repository Secrets：

- `KEYSTORE_BASE64` — `coomi-release.p12` 的单行 Base64
- `KEYSTORE_PASSWORD` — keystore 口令
- `KEYSTORE_ALIAS` — `coomi-release`
- `KEY_PASSWORD` — key 口令（PKCS#12 通常与 store 口令相同）

Secrets 只能由 GitHub Actions 使用；GitHub API 无法读回明文，只能确认名称与更新时间。

## 3. 构建校验（工作流内置，硬失败）

`coomidev-v148-full.yml` 的验证步骤会：

1. `apksigner verify --verbose` 必须成功；
2. `apksigner verify --print-certs` 的证书 SHA-256 必须等于本文第 1 节的指纹，否则 `exit 1`。

签名指纹不符说明用错了 keystore，会让已安装用户无法覆盖升级，因此按硬失败处理
（注意：`futures-terminal` 等仓库仅打印指纹供人肉核对，不构成硬校验）。

## 4. 变更纪律

1. **禁止重新生成签名替换本签名**——会导致已安装 APK 无法覆盖升级。
2. 禁止将 `.p12`/`.jks`/密码/Base64/Token 提交到公开仓库、日志、对话或 Release。
3. 禁止在 Actions 日志中 `echo`/`cat` Secrets。
4. 不得因为读不到 Secret 明文而创建新签名；缺 Secret 时工作流必须失败，不得回退随机签名。
5. 临时解码出的 keystore 位于 Actions 临时工作区，构建结束随 runner 销毁。

## 5. 灾难恢复

1. 从私有备份仓库或密码管理器取回 `coomi-release.p12` 与口令。
2. 按第 1 节 SHA-256 校验文件完整性。
3. 重新编码为单行 Base64，恢复第 2 节的 4 个 Secrets。
4. 触发 `workflow_dispatch`，按第 3 节核对证书指纹。

若口令与 GitHub Secrets 同时丢失，即使签名容器仍在也无法使用；
因此口令必须**离线**保存在独立密码管理器中。
