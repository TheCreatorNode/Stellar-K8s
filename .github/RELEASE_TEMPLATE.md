# 🚀 Stellar-K8s v[VERSION] Release Notes

> **Release Date:** YYYY-MM-DD
> **Helm Chart:** `stellar-operator-[VERSION].tgz`
> **Container Image:** `ghcr.io/stellar/stellar-k8s:[VERSION]`

---

## 📋 Table of Contents

- [Highlights](#-highlights)
- [Features](#-features)
- [Bug Fixes](#-bug-fixes)
- [Breaking Changes](#-breaking-changes)
- [Security](#-security)
- [Performance](#-performance)
- [Dependencies](#-dependencies)
- [Upgrade Guide](#-upgrade-guide)
- [Contributors](#-contributors)
- [Checksums](#-checksums)

---

## ✨ Highlights

> A short paragraph (2–4 sentences) summarizing the most important changes in this release.
> What is the theme of this release? What should users be most excited about?

---

## 🌟 Features

> List new features added in this release. Each entry should link to the relevant PR or issue.
> Remove this section if there are no new features.

- **Feature Name** — Brief description of what it does and why it matters. ([#PR_NUMBER](https://github.com/stellar/stellar-k8s/pull/PR_NUMBER))
- **Feature Name** — Brief description of what it does and why it matters. ([#PR_NUMBER](https://github.com/stellar/stellar-k8s/pull/PR_NUMBER))

<!--
Tips:
- Group related features together if there are many.
- Use sub-bullets for additional context or usage examples.
- Link to documentation if a feature has a dedicated doc page.
-->

---

## 🐛 Bug Fixes

> List bugs that were fixed in this release.
> Remove this section if there are no bug fixes.

- **Fix description** — What was broken and how it was resolved. ([#ISSUE_NUMBER](https://github.com/stellar/stellar-k8s/issues/ISSUE_NUMBER), [#PR_NUMBER](https://github.com/stellar/stellar-k8s/pull/PR_NUMBER))
- **Fix description** — What was broken and how it was resolved. ([#ISSUE_NUMBER](https://github.com/stellar/stellar-k8s/issues/ISSUE_NUMBER), [#PR_NUMBER](https://github.com/stellar/stellar-k8s/pull/PR_NUMBER))

---

## ⚠️ Breaking Changes

> **This section is critical.** List every change that requires user action to upgrade.
> Remove this section entirely if there are no breaking changes.

> [!CAUTION]
> The following changes require manual action before or after upgrading.

### `StellarNode` CRD Changes

- **Field renamed / removed / added:** `spec.oldField` → `spec.newField`. Migration steps:
  1. Step one
  2. Step two

### Helm Chart Changes

- **Value renamed:** `oldValue` has been renamed to `newValue`. Update your `values.yaml` accordingly.

### Operator Behavior Changes

- **Behavior change:** Describe what changed and what users need to do.

---

## 🔒 Security

> List security fixes, CVE patches, and dependency security updates.
> Remove this section if there are no security-related changes.

- **[CVE-YYYY-NNNNN]** — Brief description of the vulnerability and the fix. Affects versions `>= X.Y.Z, < [VERSION]`. ([Advisory](https://github.com/stellar/stellar-k8s/security/advisories/GHSA-XXXX))
- Updated `[crate-name]` from `X.Y.Z` to `A.B.C` to address [RUSTSEC-YYYY-NNNN](https://rustsec.org/advisories/RUSTSEC-YYYY-NNNN.html).

---

## ⚡ Performance

> Highlight measurable performance improvements. Include benchmark numbers where available.
> Remove this section if there are no notable performance changes.

| Metric                        | v[PREV_VERSION] | v[VERSION]  | Change   |
| ----------------------------- | --------------- | ----------- | -------- |
| Reconciliation latency (p99)  | Xms             | Yms         | -Z%      |
| Webhook admission latency (p99)| Xms            | Yms         | -Z%      |
| Binary size                   | XMB             | YMB         | -Z%      |
| Memory usage (idle)           | XMB             | YMB         | -Z%      |

---

## 📦 Dependencies

> List notable dependency updates, especially those that affect behavior or security.
> Remove this section if there are no notable dependency changes.

| Crate / Tool     | From    | To      | Notes                          |
| ---------------- | ------- | ------- | ------------------------------ |
| `kube`           | X.Y.Z   | A.B.C   | Adds support for ...           |
| `tokio`          | X.Y.Z   | A.B.C   |                                |
| Helm chart API   | vX      | vY      | Requires Kubernetes >= 1.XX    |

---

## 📖 Upgrade Guide

### From v[PREV_VERSION] → v[VERSION]

#### Using Helm

```bash
# Pull the latest chart
helm repo update

# Upgrade the operator (dry-run first)
helm upgrade stellar-operator stellar/stellar-operator \
  --version [VERSION] \
  --namespace stellar-system \
  --dry-run

# Apply the upgrade
helm upgrade stellar-operator stellar/stellar-operator \
  --version [VERSION] \
  --namespace stellar-system
```

#### Using kubectl

```bash
# Apply the updated CRDs first (if CRD changes are listed above)
kubectl apply -f https://github.com/stellar/stellar-k8s/releases/download/v[VERSION]/crds.yaml

# Restart the operator to pick up the new image
kubectl rollout restart deployment/stellar-operator -n stellar-system
```

> [!TIP]
> Always back up your `StellarNode` resources before upgrading:
> ```bash
> kubectl get stellarnodes --all-namespaces -o yaml > stellarnodes-backup.yaml
> ```

---

## 👥 Contributors

> Thank everyone who contributed to this release — code, docs, reviews, and bug reports all count.

A huge thank you to everyone who made this release possible! 🙏

<!-- List contributors using their GitHub handles -->

| Contributor | Contribution |
| ----------- | ------------ |
| @username   | Feature / Bug fix / Docs / Review |
| @username   | Feature / Bug fix / Docs / Review |

> New contributors: welcome to the Stellar-K8s community! 🎉
> Check out our [Contributing Guide](../README.md#contributing) to get started.

---

## 🔐 Checksums

> SHA-256 checksums for all release artifacts. Verify your download before use.

```
# Verify a downloaded artifact:
sha256sum --check --ignore-missing SHA256SUMS
```

| Artifact                              | SHA-256                                                          |
| ------------------------------------- | ---------------------------------------------------------------- |
| `stellar-operator-linux-amd64.tar.gz` | `<hash>`                                                         |
| `stellar-operator-linux-arm64.tar.gz` | `<hash>`                                                         |
| `stellar-operator-darwin-amd64.tar.gz`| `<hash>`                                                         |
| `stellar-operator-darwin-arm64.tar.gz`| `<hash>`                                                         |
| `stellar-operator-[VERSION].tgz`      | `<hash>`                                                         |

---

## 📚 Resources

- [Documentation](https://github.com/stellar/stellar-k8s#readme)
- [Changelog](https://github.com/stellar/stellar-k8s/blob/main/CHANGELOG.md)
- [Open Issues](https://github.com/stellar/stellar-k8s/issues)
- [Security Policy](https://github.com/stellar/stellar-k8s/blob/main/SECURITY.md)

---

<!--
=============================================================================
HOW TO USE THIS TEMPLATE
=============================================================================

1. COPY this file's content into the GitHub Release description when creating
   a new release at: https://github.com/stellar/stellar-k8s/releases/new

2. REPLACE all placeholders:
   - [VERSION]       → the new version, e.g. 0.2.0
   - [PREV_VERSION]  → the previous version, e.g. 0.1.0
   - YYYY-MM-DD      → today's date
   - PR_NUMBER / ISSUE_NUMBER → actual GitHub numbers
   - <hash>          → SHA-256 values from the release workflow's SHA256SUMS file

3. REMOVE any sections that don't apply to this release
   (e.g. no Breaking Changes → delete that whole section).

4. FILL IN the Highlights paragraph last — it's easier once all other
   sections are complete.

5. VERIFY checksums are populated. The release workflow generates a
   SHA256SUMS file automatically; copy values from there.

6. PREVIEW the release in GitHub's editor before publishing to confirm
   all links resolve and the formatting looks correct.

Tip: The release workflow (`.github/workflows/release.yml`) auto-generates
a changelog via git-cliff. Use that output as a starting point for the
Features and Bug Fixes sections, then add context and links manually.
=============================================================================
-->
