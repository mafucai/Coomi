# Coomi 自制备份格式规范 v1.2

> **主人钦定（2026-09-11），弃用官方备份。**
> 官方备份包膨胀到 1.4GB+（主因 `.node-install` 783MB），本规范自制备份包 454MB / 21269 文件。
> 状态：生效中 · 权威文本在 `rules/full-rules.md`「全量备份指令」章节

---

## 1. 触发方式

主人说「**备份**」/「**导出**」/「**打包**」→ 所有 AI **直接执行全量备份**，不问范围、不问触发词。

---

## 2. 核心变更（v1.2 vs v1.1）

| 项 | v1.1 | v1.2（现行） |
|---|---|---|
| 密钥处理 | 双包制：正文包 + 密钥 AES 加密包 | **单明文包，不分包** |
| providers.json（含各站 key） | 单独加密 | **直接打进主包** |
| `.zhipu_key` | 单独加密 | **直接打进主包** |
| SSH 私钥 | 单独加密 | **直接打进主包** |
| 外传 | 直接给 | 主人要求外传时**才**现场追加密码加密 |

> ⚠️ v1.1 的「密钥单独 AES 加密包」制度**已作废**。

---

## 3. 打包范围（10 项）

1. **渐进式记忆全部** —— dialogues + libraries + 摘要
2. **全部文档** —— docs/ + rules/ + 各项目 md
3. **`.coomi/config`** —— 含 providers
4. **`.coomi/sessions`**（40 会话全量）+ dialogues
5. **完整 MCP** —— `mcp_servers.json` + projects
6. **SSH 私钥** —— `/root/.ssh` + `/home/coomi/.ssh`
7. **Skills** —— `.coomi/skills` + skill-routing + libraries
8. **hooks + 核心脚本**
9. **其余项目源码** —— PureProbe / api-relay-tester / finance-v2 等
10. **密钥文件**

---

## 4. 排除清单

| 排除项 | 原因 |
|---|---|
| `.node-install` | 运行环境，可重建（783MB 主因） |
| `runtime-v2/versions` rootfs | 运行环境 |
| node / apk / flyctl 二进制 | 运行环境 |
| `cache/` `node_modules/` `__pycache__/` `.git/` | 缓存 |
| 旧备份包自身 | 避免递归膨胀 |
| 日志杂物 | 无价值 |

---

## 5. 打包流程（必须遵守）

```python
# 1. 用 Python zipfile —— 【禁止】系统 zip 命令
#    原因：系统 zip 遇中文路径崩溃
import zipfile

# 2. 路径用 /workspace 相对路径

# 3. 读回校验 + SHA256 抽验

# 4. request_file_export 交付

# 5. 命名：coomi-workspace-full-<时间>.zip
#    落位：backup-full/
```

### 命名格式
```
coomi-workspace-full-20260911-1048.zip
       └─ 前缀 ─┘ └─ 日期 ─┘ └时间┘
```

---

## 6. 首个实例（已验证）

| 项 | 值 |
|---|---|
| 文件 | `coomi-workspace-full-20260911-1048.zip` |
| 大小 | 454MB |
| 文件数 | 21269 |
| 完整性核对 | 应备 21287，缺 19，**全为刻意排除项** |
| 导入验证 | ✅ 已验证 |

> **首个实际使用场景** = 渐进式记忆 V3 包（229KB，导入已验证）。

---

## 7. 相关文档

| 文档 | 说明 |
|---|---|
| `rules/full-rules.md`「全量备份指令」章节 | **权威文本** |
| 渐进式记忆 `core-library/source/规则全文.md` | 已同步 |
| 本文档 §6b | ⚠️ 双包制**已作废** |

---

*创建：2026-09-11 · 归入本仓库：2026-09-16*
