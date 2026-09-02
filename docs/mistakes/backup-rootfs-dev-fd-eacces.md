# 备份扫到 `rootfs/dev/fd` 导致 EACCES

## 事故概述

备份任务扫描完整 `~/.coomi` 树时会进入 Runtime V2 的 `rootfs/dev/fd`。该路径是进程文件描述符的虚拟链接，不是普通文件。Android SELinux 会拒绝读取，备份因此卡住失败。数据本身不会损坏。

## 错误修复

- 把用户项目目录里的 `dev` 一律排除。这会误伤真实项目。
- 手动删除 `rootfs/dev/fd`。容器重启后会重建，而且依赖 `/dev/fd` 的程序可能临时失败。

## 正确修复

1. 备份排除 `runtime-v2/versions`、`downloads`、`tmp`、`node20` 和 `cache`。
2. 只把 `rootfs` / `versions` / `runtime-v2` 下的 `dev`、`proc`、`sys`、`run`、`fd` 当虚拟挂载跳过。
3. 读取单个文件遇到 `IOException` 或 `SecurityException` 时跳过该文件，不中止整次备份。
4. 继续备份 `runtime-v2/home` 和配置/会话/Skill。
