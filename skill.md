---
name: cc-switch-cli-release-workflow
description: Workflow for contributing to cc-switch-cli (Rust) including debugging/fixing interactive CLI issues, writing Conventional Commits, bumping versions, tagging, pushing, and verifying GitHub Releases (tags v* trigger .github/workflows/release.yml). Use when you need a repeatable 修复→commit→发布 checklist for this repo.
---

# CC Switch CLI 修复 → Commit → 发布 工作流

## 仓库结构（改哪里）

- 代码：`src-tauri/`（Rust crate）
  - 交互式 UI：`src-tauri/src/cli/interactive/`
  - i18n：`src-tauri/src/cli/i18n.rs`
- 发布：打 `v*` tag 会触发 `.github/workflows/release.yml`，并把各平台二进制上传到 GitHub Releases。

## 1) 先定位问题（不要上来就改）

1. Reproduce the issue with exact steps.
2. Locate the code with `rg` (prefer narrow, specific searches).
3. Identify root cause and smallest safe change.

例：UI “拖影” → 输出没有清屏、反复叠加 → 抽一个 `clear_screen()` 并在各菜单渲染前调用。

## 2) 实现修复（保持改动聚焦）

- 跨页面的行为优先放 `src-tauri/src/cli/interactive/utils.rs`（别到处 copy/paste）。
- 用户可见文案要走 i18n（用/补 `texts::*`）。

这次用到的两个模式：
- **去拖影：** 菜单渲染前清屏（例如 `console::Term::clear_screen()`）。
- **返回上一步：** `inquire` 按 `Esc` 会返回 `OperationCanceled`，把它当成 `None`，然后 `break`/`return Ok(())`。

## 3) 本地验证（发版前最少过这些）

From repo root:

```bash
cd src-tauri
rustup component add rustfmt   # if needed
cargo fmt
cargo test --lib
cargo build
```

备注：`src-tauri/tests/` 的部分集成测试可能不稳定；快速迭代时至少保证 `cargo test --lib` 通过。

## 4) Commit 写法（Conventional Commits）

把“功能改动”和“发版元数据”分开提交：

- Fix/feature commit (scoped): `fix(interactive): clear terminal between screens`
- Feature commit: `feat(interactive): support Esc to go back`
- Release commit (only version/changelog/readme): `chore(release): vX.Y.Z`

## 5) 升版本 checklist（确保下载链接对）

1. Bump `src-tauri/Cargo.toml` version.
2. Update `CHANGELOG.md` with a dated entry.
3. Update README download examples: `README.md`, `README_ZH.md` (`cc-switch-cli-vX.Y.Z-...`).
4. Run `cd src-tauri && cargo build` to refresh `src-tauri/Cargo.lock` if needed.

## 6) 打 tag、推送、确认 Release

```bash
git tag vX.Y.Z
git push origin main vX.Y.Z

gh run list -R SaladDay/cc-switch-cli --workflow release.yml --limit 3
gh run watch <run-id> -R SaladDay/cc-switch-cli --exit-status
gh release view vX.Y.Z -R SaladDay/cc-switch-cli
```

## 7) 收尾（防止“脏文件”跟着发版）

- Keep `git status` clean before tagging (don’t accidentally ship local screenshots/logs).
