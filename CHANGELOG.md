

## 0.9.0 (2023-05-08)

<csr-id-9ed17a9cb9bfcf31ccee656679ae5217365b44ea/>

### Chore (BREAKING)

 - <csr-id-9ed17a9cb9bfcf31ccee656679ae5217365b44ea/> update typify requirement from 0.0.11 to 0.0.12
   Updates the requirements on [typify](https://github.com/oxidecomputer/typify) to permit the latest version.
   <details>
   <summary>Changelog</summary>
   <p><em>Sourced from <a href="https://github.com/oxidecomputer/typify/blob/main/CHANGELOG.adoc">typify's changelog</a>.</em></p>
   <blockquote>
   <p>== 0.0.12 (released 2023-05-03)</p>
   <p><a href="https://github.com/oxidecomputer/typify/compare/v0.0.11%5C...v0.0.12%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.11\...v0.0.12[Full</a> list of commits]</p>
   <p>== 0.0.11 (released 2023-03-18)</p>
   <p>This is a big update with many, many changes to code generation, and many more
   JSON schema structures well-handled. Among the many changes:</p>
   <ul>
   <li>Generate a <code>ToString</code> impl for untagged enums with trivial variants (<a href="https://redirect.github.com/oxidecomputer/typify/issues/145">#145</a>)</li>
   <li>Allow conversion overrides by specifying a schema (<a href="https://redirect.github.com/oxidecomputer/typify/issues/155">#155</a>)</li>
   <li>Handle untyped enums that contain nulls (<a href="https://redirect.github.com/oxidecomputer/typify/issues/167">#167</a>)</li>
   <li>Handle <code>not</code> schemas for enumerated values (<a href="https://redirect.github.com/oxidecomputer/typify/issues/168">#168</a>)</li>
   <li>Improve generated code for FromStr and TryFrom impls (<a href="https://redirect.github.com/oxidecomputer/typify/issues/174">#174</a>)</li>
   <li>Handle format specifiers for enumerated strings (<a href="https://redirect.github.com/oxidecomputer/typify/issues/188">#188</a>)</li>
   </ul>
   <p>=== <em>Breaking</em>: The removal of <code>TypeSpace::to_string()</code></p>
   <p>Previously all transitive consumers required the presence of <code>rustfmt</code>. In this
   version we leave formatting to the consumer. See link:README.md#formatting[the formatting section of the README] for details on formatting.</p>
   <p>=== CLI</p>
   <p>This version adds the <code>cargo-typify</code> crate for stand-alone code generation.</p>
   <p>=== Augmented Generation</p>
   <p>Consumers can now affect how code is generated in several ways:</p>
   <ul>
   <li>adding derive macros to all generated types</li>
   <li>modifying specific types by name to rename them or add derive macros</li>
   <li>specifying a replacement type by name</li>
   <li>specifying a replacement type by schema pattern</li>
   </ul>
   <p><a href="https://github.com/oxidecomputer/typify/compare/v0.0.10%5C...v0.0.11%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.10\...v0.0.11[Full</a> list of commits]</p>
   <ul>
   <li>Allow per-type renames and derive macro applications (<a href="https://redirect.github.com/oxidecomputer/typify/issues/131">#131</a>)</li>
   <li><code>ToString</code> implementations for untagged enums with trivial newtype variants (<a href="https://redirect.github.com/oxidecomputer/typify/issues/145">#145</a>)</li>
   <li>Fixed an issue with generation of enum defaults (<a href="https://redirect.github.com/oxidecomputer/typify/issues/137">#137</a>)</li>
   <li>Allow conversion overrides by specifying a schema (<a href="https://redirect.github.com/oxidecomputer/typify/issues/155">#155</a>)</li>
   </ul>
   <p>== 0.0.10 (released 2022-09-10)</p>
   <p><a href="https://github.com/oxidecomputer/typify/compare/v0.0.9%5C...v0.0.10%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.9\...v0.0.10[Full</a> list of commits]</p>
   <ul>
   <li>Add support for string types with <code>format</code> set to <code>ip</code>, <code>ipv4</code>, or <code>ipv6</code> (<a href="https://redirect.github.com/oxidecomputer/typify/issues/76">#76</a>)</li>
   <li>Be more accommodating in the face of a missing <code>type</code> field #(79)</li>
   <li>The order of derives on types has stabilized (and therefore has changed) (<a href="https://redirect.github.com/oxidecomputer/typify/issues/81">#81</a>)</li>
   <li>Specific <code>From</code> and <code>Deserialize</code> implementations for constrainted string types (<a href="https://redirect.github.com/oxidecomputer/typify/issues/81">#81</a>)</li>
   <li>Specific <code>From</code> implementation for untagged enums with constrained string variants (<a href="https://redirect.github.com/oxidecomputer/typify/issues/81">#81</a>)</li>
   </ul>
   <!-- raw HTML omitted -->
   </blockquote>
   <p>... (truncated)</p>
   </details>
   <details>
   <summary>Commits</summary>
   <ul>
   <li><a href="https://github.com/oxidecomputer/typify/commit/c868ddcbaa07716d4cbd01312f723921bf94bce8"><code>c868ddc</code></a> release typify 0.0.12</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/a25d39f8884bd34eb8d6121d97813cb798b444c8"><code>a25d39f</code></a> update for cargo-release</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/70a5b06a0948f13264f7a27b2015c3814f90ca8a"><code>70a5b06</code></a> Allow constrained strings to be keys in generated maps (<a href="https://redirect.github.com/oxidecomputer/typify/issues/276">#276</a>)</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/b81f077d07b6a6b97c388bddf2a8b6c49a894003"><code>b81f077</code></a> Bump clap from 4.2.4 to 4.2.5 (<a href="https://redirect.github.com/oxidecomputer/typify/issues/275">#275</a>)</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/20b5ba40eaab9254831b4e18a18a9a428ab45daa"><code>20b5ba4</code></a> some general enum cleanup (<a href="https://redirect.github.com/oxidecomputer/typify/issues/270">#270</a>)</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/7d742a74fd2e1b643dab0e734e2a240d1243e600"><code>7d742a7</code></a> Bump uuid from 1.3.1 to 1.3.2 (<a href="https://redirect.github.com/oxidecomputer/typify/issues/273">#273</a>)</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/e98b83fc5c644dfc38c54b750d9d06c2e0248955"><code>e98b83f</code></a> Bump regress from 0.5.0 to 0.6.0 (<a href="https://redirect.github.com/oxidecomputer/typify/issues/274">#274</a>)</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/4152a79d8e04cb9d8625c2fa0232f6aa657dbb24"><code>4152a79</code></a> Small documentation change (<a href="https://redirect.github.com/oxidecomputer/typify/issues/271">#271</a>)</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/8d505f920c0865ead018db360d5c20e7cb668519"><code>8d505f9</code></a> Bump expectorate from 1.0.6 to 1.0.7 (<a href="https://redirect.github.com/oxidecomputer/typify/issues/268">#268</a>)</li>
   <li><a href="https://github.com/oxidecomputer/typify/commit/fb0eefa0cb7452646b564a65188631ae449224e6"><code>fb0eefa</code></a> Bump clap from 4.2.2 to 4.2.4 (<a href="https://redirect.github.com/oxidecomputer/typify/issues/267">#267</a>)</li>
   <li>Additional commits viewable in <a href="https://github.com/oxidecomputer/typify/compare/v0.0.11...v0.0.12">compare view</a></li>
   </ul>
   </details>
   <br />
   
   
   You can trigger a rebase of this PR by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 10 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#101](https://github.com/substrait-io/substrait-rs/issues/101)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#101](https://github.com/substrait-io/substrait-rs/issues/101)**
    - Update typify requirement from 0.0.11 to 0.0.12 ([`9ed17a9`](https://github.com/substrait-io/substrait-rs/commit/9ed17a9cb9bfcf31ccee656679ae5217365b44ea))
</details>

## 0.8.1 (2023-04-27)

<csr-id-e7b12ac1056fbbe4e227b7019c13cdfafcb63d09/>

### Chore

 - <csr-id-e7b12ac1056fbbe4e227b7019c13cdfafcb63d09/> update gix to 0.44
   0.43 seems to fail after 0.44 was published, see
     https://github.com/apache/arrow-datafusion/issues/6132 .

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#97](https://github.com/substrait-io/substrait-rs/issues/97)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#97](https://github.com/substrait-io/substrait-rs/issues/97)**
    - Update gix to 0.44 ([`e7b12ac`](https://github.com/substrait-io/substrait-rs/commit/e7b12ac1056fbbe4e227b7019c13cdfafcb63d09))
 * **Uncategorized**
    - Release substrait v0.8.1 ([`74cb1fe`](https://github.com/substrait-io/substrait-rs/commit/74cb1fe0640d5d57b8155ff843d5eb4c15b22d91))
</details>

## 0.8.0 (2023-04-24)

<csr-id-32f5a9dd33e298798d487830de81a93fea6139f8/>

### Chore (BREAKING)

 - <csr-id-32f5a9dd33e298798d487830de81a93fea6139f8/> bump substrait from 0.28.2 to 0.29.0
   Bumps [substrait](https://github.com/substrait-io/substrait) from `5e99f0b` to `16503aa`.
   <details>
   <summary>Commits</summary>
   <ul>
   <li><a href="https://github.com/substrait-io/substrait/commit/16503aaf412a3a4771fc0d17b5ac4883e26954aa"><code>16503aa</code></a> chore(release): 0.29.0</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/7246102f0e1f056a3b5a13eb96fec36ff28d27a5"><code>7246102</code></a> fix(text)!: mark <code>name</code> and <code>structure</code> property of <code>type</code> extension item as ...</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/b5d7ed26a17c0a0bd6d0779d312942e5216ea9fa"><code>b5d7ed2</code></a> fix: referenced simple extension in tutorial (set instead of string) (<a href="https://redirect.github.com/substrait-io/substrait/issues/494">#494</a>)</li>
   <li>See full diff in <a href="https://github.com/substrait-io/substrait/compare/5e99f0b5f89306e1ab18c355d1f82ab5aff2d21a...16503aaf412a3a4771fc0d17b5ac4883e26954aa">compare view</a></li>
   </ul>
   </details>
   <br />
   
   
   Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#96](https://github.com/substrait-io/substrait-rs/issues/96)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#96](https://github.com/substrait-io/substrait-rs/issues/96)**
    - Bump substrait from 0.28.2 to 0.29.0 ([`32f5a9d`](https://github.com/substrait-io/substrait-rs/commit/32f5a9dd33e298798d487830de81a93fea6139f8))
 * **Uncategorized**
    - Release substrait v0.8.0 ([`2feae85`](https://github.com/substrait-io/substrait-rs/commit/2feae85523dde923599dae32bfa383774f380f74))
</details>

## 0.7.5 (2023-04-17)

### Bug Fixes

 - <csr-id-e8aceb239cd7dba40f8d7dbd471bddaa5065dff9/> skip re-runs if there is no substrait git submodule

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#95](https://github.com/substrait-io/substrait-rs/issues/95)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#95](https://github.com/substrait-io/substrait-rs/issues/95)**
    - Skip re-runs if there is no substrait git submodule ([`e8aceb2`](https://github.com/substrait-io/substrait-rs/commit/e8aceb239cd7dba40f8d7dbd471bddaa5065dff9))
 * **Uncategorized**
    - Release substrait v0.7.5 ([`6f064c8`](https://github.com/substrait-io/substrait-rs/commit/6f064c83127924481ad1c57e3150a91dd7caa5b0))
</details>

## 0.7.4 (2023-04-17)

<csr-id-83a18ee71b2174cd3e1118ae789cb0e09d4a3b2e/>

### Chore

 - <csr-id-83a18ee71b2174cd3e1118ae789cb0e09d4a3b2e/> bump substrait from 0.28.1 to 0.28.2
   Bumps [substrait](https://github.com/substrait-io/substrait) from `c88686c` to `5e99f0b`.
   <details>
   <summary>Commits</summary>
   <ul>
   <li><a href="https://github.com/substrait-io/substrait/commit/5e99f0b5f89306e1ab18c355d1f82ab5aff2d21a"><code>5e99f0b</code></a> chore(release): 0.28.2</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/8c230af70bc98805d84d20c72f32d0ddb84f8644"><code>8c230af</code></a> fix: separate strptime to fix spec violation (<a href="https://redirect.github.com/substrait-io/substrait/issues/493">#493</a>)</li>
   <li>See full diff in <a href="https://github.com/substrait-io/substrait/compare/c88686c9326aeab444d596a47583fd24608928b2...5e99f0b5f89306e1ab18c355d1f82ab5aff2d21a">compare view</a></li>
   </ul>
   </details>
   <br />
   
   
   Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 8 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#93](https://github.com/substrait-io/substrait-rs/issues/93)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#93](https://github.com/substrait-io/substrait-rs/issues/93)**
    - Bump substrait from 0.28.1 to 0.28.2 ([`83a18ee`](https://github.com/substrait-io/substrait-rs/commit/83a18ee71b2174cd3e1118ae789cb0e09d4a3b2e))
 * **Uncategorized**
    - Release substrait v0.7.4 ([`e2f0cc9`](https://github.com/substrait-io/substrait-rs/commit/e2f0cc957122cfa07a88739ea13570e29ffa87c7))
</details>

## 0.7.3 (2023-04-09)

<csr-id-fb61e47daa9ad330bc2fbedbebf992313b5943b1/>

### Chore

 - <csr-id-fb61e47daa9ad330bc2fbedbebf992313b5943b1/> bump substrait from 0.28.0 to 0.28.1
   Bumps [substrait](https://github.com/substrait-io/substrait) from `26da4f1` to `c88686c`.
   <details>
   <summary>Commits</summary>
   <ul>
   <li><a href="https://github.com/substrait-io/substrait/commit/c88686c9326aeab444d596a47583fd24608928b2"><code>c88686c</code></a> chore(release): 0.28.1</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/90469453d111ba93983b00944dd79d0ddd8a3808"><code>9046945</code></a> fix: typo in the comment/docstring (<a href="https://redirect.github.com/substrait-io/substrait/issues/492">#492</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/fb3eafbc9a564fbaeb92e07dd8d616a9fe484638"><code>fb3eafb</code></a> chore(site): loosen mkdocs dependencies for site build (<a href="https://redirect.github.com/substrait-io/substrait/issues/486">#486</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/e8ced2a8df89a2fc635b3e13b6b5cd84ef9306af"><code>e8ced2a</code></a> chore(deps): bump bufbuild/buf-setup-action from 0.7.0 to 1.16.0 (<a href="https://redirect.github.com/substrait-io/substrait/issues/484">#484</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/577b4e0d87f3010469599e2087548a60372ca40e"><code>577b4e0</code></a> chore(deps): bump actions/checkout from 2 to 3 (<a href="https://redirect.github.com/substrait-io/substrait/issues/459">#459</a>)</li>
   <li>See full diff in <a href="https://github.com/substrait-io/substrait/compare/26da4f155441ce1d5faf0ae9e7445f9f45abd0f2...c88686c9326aeab444d596a47583fd24608928b2">compare view</a></li>
   </ul>
   </details>
   <br />
   
   
   Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 3 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#91](https://github.com/substrait-io/substrait-rs/issues/91)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#91](https://github.com/substrait-io/substrait-rs/issues/91)**
    - Bump substrait from 0.28.0 to 0.28.1 ([`fb61e47`](https://github.com/substrait-io/substrait-rs/commit/fb61e47daa9ad330bc2fbedbebf992313b5943b1))
 * **Uncategorized**
    - Release substrait v0.7.3 ([`9e9c1c7`](https://github.com/substrait-io/substrait-rs/commit/9e9c1c7c6ae0b84ab4ffa4c504b858c6ff0ff82d))
</details>

## 0.7.2 (2023-04-05)

<csr-id-d390c76ec390d5a346d19339eaffc69e5351452e/>

### Chore

 - <csr-id-d390c76ec390d5a346d19339eaffc69e5351452e/> add .vscode to gitignore

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#88](https://github.com/substrait-io/substrait-rs/issues/88)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#88](https://github.com/substrait-io/substrait-rs/issues/88)**
    - Add .vscode to gitignore ([`d390c76`](https://github.com/substrait-io/substrait-rs/commit/d390c76ec390d5a346d19339eaffc69e5351452e))
 * **Uncategorized**
    - Release substrait v0.7.2 ([`c83e2a7`](https://github.com/substrait-io/substrait-rs/commit/c83e2a757b298d630ff6cca38106e37524153378))
</details>

## 0.7.1 (2023-04-04)

### Bug Fixes

 - <csr-id-6445de8a35a62fd095cd5ae1c9410f51a5f7e17f/> remove filter that skipped `extension_types.yaml` deserialize test

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#78](https://github.com/substrait-io/substrait-rs/issues/78)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#78](https://github.com/substrait-io/substrait-rs/issues/78)**
    - Remove filter that skipped `extension_types.yaml` deserialize test ([`6445de8`](https://github.com/substrait-io/substrait-rs/commit/6445de8a35a62fd095cd5ae1c9410f51a5f7e17f))
 * **Uncategorized**
    - Release substrait v0.7.1 ([`98b3a7b`](https://github.com/substrait-io/substrait-rs/commit/98b3a7b2ce25c199b6a68181c618b955665e97ab))
</details>

## 0.7.0 (2023-04-02)

<csr-id-6d117739ee41eb8a680931e57ea263539196a96f/>
<csr-id-11d44c88162d76cfaf5b9ce479a864f0d3e56879/>
<csr-id-b76692943ae2a5cb7c8fa83f4d8c502b265a0980/>

### Chore

 - <csr-id-6d117739ee41eb8a680931e57ea263539196a96f/> update gix requirement from 0.42 to 0.43
   Updates the requirements on [gix](https://github.com/Byron/gitoxide) to permit the latest version.
   <details>
   <summary>Release notes</summary>
   <p><em>Sourced from <a href="https://github.com/Byron/gitoxide/releases">gix's releases</a>.</em></p>
   <blockquote>
   <h2>gix-odb v0.43.0</h2>
   <p>A maintenance release without user-facing changes.</p>
   <h3>Commit Statistics</h3>
   <ul>
   <li>1 commit contributed to the release.</li>
   <li>6 days passed between releases.</li>
   <li>0 commits were understood as <a href="https://www.conventionalcommits.org">conventional</a>.</li>
   <li>0 issues like '(#ID)' were seen in commit messages</li>
   </ul>
   <h3>Commit Details</h3>
   <!-- raw HTML omitted -->
   <!-- raw HTML omitted -->
   <ul>
   <li><strong>Uncategorized</strong>
   <ul>
   <li>Prepare changelogs prior to release (e06f5f5)</li>
   </ul>
   </li>
   </ul>
   <!-- raw HTML omitted -->
   </blockquote>
   </details>
   <details>
   <summary>Changelog</summary>
   <p><em>Sourced from <a href="https://github.com/Byron/gitoxide/blob/main/CHANGELOG.md">gix's changelog</a>.</em></p>
   <blockquote>
   <h1>Changelog</h1>
   <p>All notable changes to this project will be documented in this file.</p>
   <p>The format is based on <a href="https://keepachangelog.com/en/1.0.0/">Keep a Changelog</a>,
   and this project adheres to <a href="https://semver.org/spec/v2.0.0.html">Semantic Versioning</a>.</p>
   <h2>0.23.0 (2023-02-24)</h2>
   <p>This release adds the new <code>ein tool query</code> analytics engine, which maintains a database of a git repository as acceleration data structure to run useful queries on.
   It's nothing more than an MVP, and was inspired by a program to accelerate <a href="https://redirect.github.com/jmforsythe/Git-Heat-Map/pull/6">Git-Heat-Map</a>.</p>
   <h3>New Features</h3>
   <ul>
   <li><!-- raw HTML omitted --> <code>ein tool query</code> - a git analytics engine.
   A tool to build and efficiently maintain a database of information contained
   in a git repository, preferably the kind of information that is expensive to obtain,
   in order to facilitate queries that would be prohibitive without an accelerating
   data structure.</li>
   <li><!-- raw HTML omitted --> <code>gix tree entries</code> with rev-spec support.
   Previously it wanted a tree-id, now it can derive it itself.</li>
   </ul>
   <h3>Commit Statistics</h3>
   <!-- raw HTML omitted -->
   <ul>
   <li>4 commits contributed to the release over the course of 7 calendar days.</li>
   <li>7 days passed between releases.</li>
   <li>2 commits were understood as <a href="https://www.conventionalcommits.org">conventional</a>.</li>
   <li>0 issues like '(#ID)' were seen in commit messages</li>
   </ul>
   <h3>Commit Details</h3>
   <!-- raw HTML omitted -->
   <!-- raw HTML omitted -->
   <ul>
   <li><strong>Uncategorized</strong>
   <ul>
   <li>Merge branch 'rename-tracking' (<a href="https://github.com/Byron/gitoxide/commit/550144a5fd37d501d86f4b1c4db2948d951d1c93"><code>550144a</code></a>)</li>
   <li><code>ein tool query</code> - a git analytics engine. (<a href="https://github.com/Byron/gitoxide/commit/f8cc62376bd6e940be4c41e84ccbfce111e64e39"><code>f8cc623</code></a>)</li>
   <li><code>gix tree entries</code> with rev-spec support. (<a href="https://github.com/Byron/gitoxide/commit/49520d1f03ee1d011f6e9f2eb07084d327fe95ae"><code>49520d1</code></a>)</li>
   <li>Fix journey tests; improve panic handling when --progress is used. (<a href="https://github.com/Byron/gitoxide/commit/571121ccceaab25323bc6cd1ed2e2a1560289a57"><code>571121c</code></a>)</li>
   </ul>
   </li>
   </ul>
   <!-- raw HTML omitted -->
   <h2>0.22.1 (2023-02-17)</h2>
   <h3>Bug Fixes</h3>
   <ul>
   <li><!-- raw HTML omitted --> re-enable local-time support for all binaries
   The <code>time</code> improved the way one can opt-in to potential unsoundness</li>
   </ul>
   <!-- raw HTML omitted -->
   </blockquote>
   <p>... (truncated)</p>
   </details>
   <details>
   <summary>Commits</summary>
   <ul>
   <li><a href="https://github.com/Byron/gitoxide/commit/5dc1f9f2bcb8b3e147115fcb6f76558e8f48ffef"><code>5dc1f9f</code></a> Release gix-tempfile v5.0.2, gix-validate v0.7.4, gix-config v0.20.0, gix-pro...</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/3016a285f566bdfe7de2774fa6f2254c1b1a2c51"><code>3016a28</code></a> prepare changelogs prior to release</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/ee36e5bb985e9ad90bc382cdd051a8b5295ca18c"><code>ee36e5b</code></a> Merge branch 'fix-790'</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/603776ecf487ef087d25774d74e49465177aa370"><code>603776e</code></a> fix: binary config output parsing can now deal with quotes on windows. (<a href="https://redirect.github.com/Byron/gitoxide/issues/790">#790</a>)</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/7bd8823ab4241d6d0401f03aec8c0d34f68c347c"><code>7bd8823</code></a> fix: opening repositories without 'strict' mode also ignores IO errors. (<a href="https://redirect.github.com/Byron/gitoxide/issues/790">#790</a>)</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/e55f4ee230ed3164df5145c7a2b212464bb9db99"><code>e55f4ee</code></a> feat!: allow to ignore IO errors when reading configuration files. (<a href="https://redirect.github.com/Byron/gitoxide/issues/790">#790</a>)</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/8f2accdf738def9aa4abdf08fc299d0e9807bc3e"><code>8f2accd</code></a> Less dependencies for tests (via <code>serial_test</code> no default features)</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/a69f873ea53b073a0b86ade095596f21709b233b"><code>a69f873</code></a> Finally fix typos detected by <code>typos</code> tool.</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/2321eb971c2b89551506e2016a3495fafd15b47d"><code>2321eb9</code></a> Correct more typos with <code>typos</code> tool.</li>
   <li><a href="https://github.com/Byron/gitoxide/commit/bbb4cb03ee2bcd0c5dd0635da542a942179cf6fd"><code>bbb4cb0</code></a> add group headings to all shallow-related arguments.</li>
   <li>Additional commits viewable in <a href="https://github.com/Byron/gitoxide/compare/gix-v0.42.0...gix-v0.43.0">compare view</a></li>
   </ul>
   </details>
   <br />
   
   
   You can trigger a rebase of this PR by commenting `@dependabot rebase`.
 - <csr-id-11d44c88162d76cfaf5b9ce479a864f0d3e56879/> bump actions/deploy-pages from 1 to 2
   Bumps [actions/deploy-pages](https://github.com/actions/deploy-pages) from 1 to 2.
   <details>
   <summary>Release notes</summary>
   <p><em>Sourced from <a href="https://github.com/actions/deploy-pages/releases">actions/deploy-pages's releases</a>.</em></p>
   <blockquote>
   <h2>v2.0.0</h2>
   <h1>Changelog</h1>
   <ul>
   <li><strong>REAPPLY:</strong> Update the deployment API endpoints used by the api-client module <a href="https://github.com/TooManyBees"><code>@​TooManyBees</code></a> / <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/140">#140</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/deploy-pages/compare/v1.2.8...v2.0.0">all code changes</a> since previous release.</p>
   <h2>v1.2.8</h2>
   <p>⚠️ This release is essentially a revert of <a href="https://github.com/actions/deploy-pages/releases/v1.2.7"><code>v1.2.7</code></a> and identical to the prior release <a href="https://github.com/actions/deploy-pages/releases/v1.2.6"><code>v1.2.6</code></a>.</p>
   <h1>Changelog</h1>
   <ul>
   <li>Revert shifted Deployments API endpoint usage <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/139">#139</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/deploy-pages/compare/v1.2.7...v1.2.8">all code changes</a> since previous release.</p>
   <h2>v1.2.7</h2>
   <h1>Changelog</h1>
   <ul>
   <li>Update the deployment API endpoints used by the api-client module <a href="https://github.com/TooManyBees"><code>@​TooManyBees</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/136">#136</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/deploy-pages/compare/v1.2.6...v1.2.7">all code changes</a> since previous release.</p>
   <h2>v1.2.6</h2>
   <h1>Changelog</h1>
   <ul>
   <li>Bump eslint from 8.35.0 to 8.36.0 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/133">#133</a>)</li>
   <li>Bump <code>@​actions/http-client</code> from 2.0.1 to 2.1.0 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/134">#134</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/deploy-pages/compare/v1.2.5...v1.2.6">all code changes</a> since previous release.</p>
   <h2>v1.2.5</h2>
   <h1>Changelog</h1>
   <ul>
   <li>Use the <code>@actions/http-client</code> and <code>@actions/github</code> modules for proxy support <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/124">#124</a>)</li>
   <li>Improve name of distributables checking workflow <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/131">#131</a>)</li>
   <li>Bump eslint-config-prettier from 8.6.0 to 8.7.0 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/130">#130</a>)</li>
   <li>Bump jest from 29.4.3 to 29.5.0 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/129">#129</a>)</li>
   <li>Bump eslint from 8.34.0 to 8.35.0 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/127">#127</a>)</li>
   <li>Revise Dependabot rebuild workflow <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/126">#126</a>)</li>
   <li>Deprecate the <code>conclusion</code> parameter for the Action <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/125">#125</a>)</li>
   <li>Bump prettier from 2.8.3 to 2.8.4 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/123">#123</a>)</li>
   <li>Bump jest from 29.4.1 to 29.4.3 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/122">#122</a>)</li>
   <li>Bump eslint from 8.33.0 to 8.34.0 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/121">#121</a>)</li>
   <li>Separate use of Release Drafter <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/119">#119</a>)</li>
   <li>Bump axios from 1.3.0 to 1.3.3 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/113">#113</a>)</li>
   <li>Bump eslint-plugin-github from 4.3.4 to 4.6.1 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/116">#116</a>)</li>
   <li>Update rebuilding workflow to utilize an org-owned PAT <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/115">#115</a>)</li>
   <li>Bump nock from 13.2.0 to 13.3.0 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/104">#104</a>)</li>
   <li>Add a workflow to rebuild the distributables for Dependabot PRs <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://redirect.github.com/actions/deploy-pages/issues/110">#110</a>)</li>
   </ul>
   <!-- raw HTML omitted -->
   </blockquote>
   <p>... (truncated)</p>
   </details>
   <details>
   <summary>Commits</summary>
   <ul>
   <li><a href="https://github.com/actions/deploy-pages/commit/73e62e651178eeba977de2dc9f4c7645b3d01015"><code>73e62e6</code></a> Merge pull request <a href="https://redirect.github.com/actions/deploy-pages/issues/140">#140</a> from actions/cut-v2</li>
   <li><a href="https://github.com/actions/deploy-pages/commit/b254707f5c3c910c335f3bcfd13a08eb14d18dca"><code>b254707</code></a> Update the deployment API endpoints used by the api-client module</li>
   <li>See full diff in <a href="https://github.com/actions/deploy-pages/compare/v1...v2">compare view</a></li>
   </ul>
   </details>
   <br />
   
   
   [![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/deploy-pages&package-manager=github_actions&previous-version=1&new-version=2)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
   
   Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.

### Chore (BREAKING)

 - <csr-id-b76692943ae2a5cb7c8fa83f4d8c502b265a0980/> bump substrait from `7f272f1` to `26da4f1`
   Bumps [substrait](https://github.com/substrait-io/substrait) from `7f272f1` to `26da4f1`.
   <details>
   <summary>Commits</summary>
   <ul>
   <li><a href="https://github.com/substrait-io/substrait/commit/26da4f155441ce1d5faf0ae9e7445f9f45abd0f2"><code>26da4f1</code></a> chore(release): 0.28.0</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/4f49e967fa8451eb60443a99287ee05c8680e190"><code>4f49e96</code></a> docs: fix some typos (<a href="https://redirect.github.com/substrait-io/substrait/issues/478">#478</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/425e7f868e0f89115bc125e8dab2c04b8144ff82"><code>425e7f8</code></a> feat: adding BibTex entry to cite Substrait (<a href="https://redirect.github.com/substrait-io/substrait/issues/481">#481</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/73228b4112d79eb1011af0ebb41753ce23ca180c"><code>73228b4</code></a> feat: adding SUM0 definition for aggregate functions (<a href="https://redirect.github.com/substrait-io/substrait/issues/465">#465</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/3955871c17f9cb0a12820192d9de9595c42654de"><code>3955871</code></a> chore(release): 0.27.0</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/45b4e483ff1fca3c3e4d0f71e6e55436c6d7638a"><code>45b4e48</code></a> fix(ci): fix link to conventional commits spec (<a href="https://redirect.github.com/substrait-io/substrait/issues/482">#482</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/b4d81fba48990523012c7b2c6cc71d2c01650e59"><code>b4d81fb</code></a> feat: add regexp_match_substring_all function to yaml (<a href="https://redirect.github.com/substrait-io/substrait/issues/469">#469</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/b7df38d2099cd970d1ed1783d441d828ce84253d"><code>b7df38d</code></a> fix: remove duplication in simple extensions schema (<a href="https://redirect.github.com/substrait-io/substrait/issues/404">#404</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/bb5d0bc0390ab7bd7e2047126cf9661d7b57c3df"><code>bb5d0bc</code></a> chore(ci): skip <code>release</code> job on forked repositories (<a href="https://redirect.github.com/substrait-io/substrait/issues/475">#475</a>)</li>
   <li><a href="https://github.com/substrait-io/substrait/commit/96b686128e57eb66c55ab04e4dcbae64c9250dca"><code>96b6861</code></a> docs: fix typo, fetch operation only has one input (<a href="https://redirect.github.com/substrait-io/substrait/issues/461">#461</a>)</li>
   <li>Additional commits viewable in <a href="https://github.com/substrait-io/substrait/compare/7f272f13f22cd5f5842baea42bcf7961e6251881...26da4f155441ce1d5faf0ae9e7445f9f45abd0f2">compare view</a></li>
   </ul>
   </details>
   <br />
   
   
   Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 3 unique issues were worked on: [#75](https://github.com/substrait-io/substrait-rs/issues/75), [#80](https://github.com/substrait-io/substrait-rs/issues/80), [#86](https://github.com/substrait-io/substrait-rs/issues/86)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#75](https://github.com/substrait-io/substrait-rs/issues/75)**
    - Bump actions/deploy-pages from 1 to 2 ([`11d44c8`](https://github.com/substrait-io/substrait-rs/commit/11d44c88162d76cfaf5b9ce479a864f0d3e56879))
 * **[#80](https://github.com/substrait-io/substrait-rs/issues/80)**
    - Update gix requirement from 0.42 to 0.43 ([`6d11773`](https://github.com/substrait-io/substrait-rs/commit/6d117739ee41eb8a680931e57ea263539196a96f))
 * **[#86](https://github.com/substrait-io/substrait-rs/issues/86)**
    - Bump substrait from `7f272f1` to `26da4f1` ([`b766929`](https://github.com/substrait-io/substrait-rs/commit/b76692943ae2a5cb7c8fa83f4d8c502b265a0980))
 * **Uncategorized**
    - Release substrait v0.7.0 ([`4d94c48`](https://github.com/substrait-io/substrait-rs/commit/4d94c48417901f77f4474560fec6dd1514ee86d0))
</details>

## 0.6.1 (2023-04-02)

<csr-id-b4bf64c476038d20e5858ed1171397baec7bb5fa/>

### Chore

 - <csr-id-b4bf64c476038d20e5858ed1171397baec7bb5fa/> bump actions/configure-pages from 2 to 3
   Bumps [actions/configure-pages](https://github.com/actions/configure-pages) from 2 to 3.
   <details>
   <summary>Release notes</summary>
   <p><em>Sourced from <a href="https://github.com/actions/configure-pages/releases">actions/configure-pages's releases</a>.</em></p>
   <blockquote>
   <h2>v3.0.0</h2>
   <h1>Changelog</h1>
   <ul>
   <li>Update default behavior to NOT attempt to create/enable the Pages site <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/48">#48</a>)</li>
   <li>Bump actions/publish-action from 0.2.1 to 0.2.2 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/47">#47</a>)</li>
   <li>Bump json5 from 1.0.1 to 1.0.2 <a href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/45">#45</a>)</li>
   <li>Add Dependabot config for Actions usage updates <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/46">#46</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/configure-pages/compare/v2.1.3...v2.1.4">all code changes</a> since previous release.</p>
   <h2>v2.1.3</h2>
   <h1>Changelog</h1>
   <ul>
   <li>Warn about unsupported file extensions <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/43">#43</a>)</li>
   <li>Update to the latest <code>actions/publish-action</code> <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/42">#42</a>)</li>
   <li>Test: Better support alternative file extensions for blank config files <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/41">#41</a>)</li>
   <li>Support wrapped exports <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/38">#38</a>)</li>
   <li>Merge PR <a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/33">#33</a> <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/37">#37</a>)</li>
   <li>Bump actions <a href="https://github.com/yoannchaudet"><code>@​yoannchaudet</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/35">#35</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/configure-pages/compare/v2.1.2...v2.1.3">all code changes</a> since previous release.</p>
   <h2>v2.1.2</h2>
   <h1>Changelog</h1>
   <ul>
   <li>Update <code>@​actions/core</code> to 1.10.0 <a href="https://github.com/rentziass"><code>@​rentziass</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/31">#31</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/configure-pages/compare/v2.1.1...v2.1.2">all code changes</a> since previous release.</p>
   <h2>v2.1.1</h2>
   <h1>Changelog</h1>
   <ul>
   <li>Fix non-Code links in README <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/27">#27</a>)</li>
   <li>Clean up some unused dependencies <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/26">#26</a>)</li>
   <li>Add ESLint <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/25">#25</a>)</li>
   <li>Expand Prettier usage <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/24">#24</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/configure-pages/compare/v2.1.0...v2.1.1">all code changes</a> since previous release.</p>
   <h2>v2.1.0</h2>
   <h1>Changelog</h1>
   <ul>
   <li>Apply consistent spacing and use of single quotes <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github.com/actions/configure-pages/commit/45efe609374726fd94570f0e5a4c32f41675e823">https://github.com/actions/configure-pages/commit/45efe609374726fd94570f0e5a4c32f41675e823</a>)</li>
   <li>Use GitHub Pages site origin for setting up SSG configs <a href="https://github.com/AndrewLester"><code>@​AndrewLester</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/pull/21">#21</a>)</li>
   <li>Update major version only on full releases <a href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a> (<a href="https://github-redirect.dependabot.com/actions/configure-pages/pull/20">#20</a>)</li>
   </ul>
   <p>See details of <a href="https://github.com/actions/configure-pages/compare/v2.0.0...v2.0.1">all code changes</a> since previous release.</p>
   </blockquote>
   </details>
   <details>
   <summary>Commits</summary>
   <ul>
   <li><a href="https://github.com/actions/configure-pages/commit/5992ce8fd557229bb0e98f78a89136a149cc9758"><code>5992ce8</code></a> Merge pull request <a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/48">#48</a> from actions/do-not-enable-by-default</li>
   <li><a href="https://github.com/actions/configure-pages/commit/c8deda3832e3d38f7eb5bcaf7ebf9caab3b80449"><code>c8deda3</code></a> Update distributables</li>
   <li><a href="https://github.com/actions/configure-pages/commit/5d8963e8a5ad9247b4796dc9d7c8071edf7902df"><code>5d8963e</code></a> Update default behavior to NOT attempt to create/enable the Pages site</li>
   <li><a href="https://github.com/actions/configure-pages/commit/529ba710d519aa4546466c0eede5fb44bb7e37fc"><code>529ba71</code></a> Merge pull request <a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/47">#47</a> from actions/dependabot/github_actions/actions/publish...</li>
   <li><a href="https://github.com/actions/configure-pages/commit/29e9dd5abeec9a7e9bbd197ca34df0ca6ccfa2b1"><code>29e9dd5</code></a> Bump actions/publish-action from 0.2.1 to 0.2.2</li>
   <li><a href="https://github.com/actions/configure-pages/commit/c450a282d7e64525a8c9eb838cb283c6490b1c80"><code>c450a28</code></a> Merge pull request <a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/45">#45</a> from actions/dependabot/npm_and_yarn/json5-1.0.2</li>
   <li><a href="https://github.com/actions/configure-pages/commit/6567d4c154b26b7bd05d333f214469bf5d1315e3"><code>6567d4c</code></a> Bump json5 from 1.0.1 to 1.0.2</li>
   <li><a href="https://github.com/actions/configure-pages/commit/aba1aa6bab0f46358e2cf2ff046300601009e2a7"><code>aba1aa6</code></a> Merge pull request <a href="https://github-redirect.dependabot.com/actions/configure-pages/issues/46">#46</a> from actions/dependabot-config</li>
   <li><a href="https://github.com/actions/configure-pages/commit/6a80311a7311cfb534beb3c7e1156325dcc2e776"><code>6a80311</code></a> Comply with Prettier expectations</li>
   <li><a href="https://github.com/actions/configure-pages/commit/380c12d4cc49b34de8073de775c4aa81a2c4fcc7"><code>380c12d</code></a> Add Dependabot config for Actions usage updates</li>
   <li>See full diff in <a href="https://github.com/actions/configure-pages/compare/v2...v3">compare view</a></li>
   </ul>
   </details>
   <br />
   
   
   [![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/configure-pages&package-manager=github_actions&previous-version=2&new-version=3)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)
   
   Dependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#60](https://github.com/substrait-io/substrait-rs/issues/60)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#60](https://github.com/substrait-io/substrait-rs/issues/60)**
    - Bump actions/configure-pages from 2 to 3 ([`b4bf64c`](https://github.com/substrait-io/substrait-rs/commit/b4bf64c476038d20e5858ed1171397baec7bb5fa))
 * **Uncategorized**
    - Release substrait v0.6.1 ([`f5fa994`](https://github.com/substrait-io/substrait-rs/commit/f5fa9943e6fc1222e8d51f2ec69de8c24216326c))
</details>

## 0.6.0 (2023-03-30)

<csr-id-d2e7ac8dad29754ab190207e9a3dd39f3deb1b1f/>

### Chore (BREAKING)

 - <csr-id-d2e7ac8dad29754ab190207e9a3dd39f3deb1b1f/> update typify, prettyplease and syn
   Cherry-pick of #76, #82 and #83. These need to be combined.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#84](https://github.com/substrait-io/substrait-rs/issues/84)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#84](https://github.com/substrait-io/substrait-rs/issues/84)**
    - Update typify, prettyplease and syn ([`d2e7ac8`](https://github.com/substrait-io/substrait-rs/commit/d2e7ac8dad29754ab190207e9a3dd39f3deb1b1f))
 * **Uncategorized**
    - Release substrait v0.6.0 ([`7a12bed`](https://github.com/substrait-io/substrait-rs/commit/7a12bed30c3f8361f4f0a74f3b514f07ab2e8b90))
</details>

## 0.5.4 (2023-03-30)

### Bug Fixes

 - <csr-id-63aa2137f369dde6f1ce8dfe9f3719d569020319/> checkout repository in pull request check job to get config file
   Now that there is a config file for conventional commits we should also checkout the repo in the conventional commits check job of the pull request workflow.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#81](https://github.com/substrait-io/substrait-rs/issues/81)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#81](https://github.com/substrait-io/substrait-rs/issues/81)**
    - Checkout repository in pull request check job to get config file ([`63aa213`](https://github.com/substrait-io/substrait-rs/commit/63aa2137f369dde6f1ce8dfe9f3719d569020319))
 * **Uncategorized**
    - Release substrait v0.5.4 ([`8c1ded3`](https://github.com/substrait-io/substrait-rs/commit/8c1ded316241597ce8188fac9b817aba99bbda67))
</details>

## 0.5.3 (2023-03-28)

<csr-id-3991a0f9e25edf9e7ecce112916f322f39089702/>

### Chore

 - <csr-id-3991a0f9e25edf9e7ecce112916f322f39089702/> replace removed `typfify::TypeSpace::to_string()` with `prettyplease`
   The use of `typify` assumed that `rustfmt` was installed... which turned out not to be a great assumption. We've modified `typify` to remove the dependency on `rustfmt-wrapper` and have removed the interface that used it `ToString::to_string()`. Instead we recommend that consumers use `prettyplease` for `build.rs` uses such as the one in this crate. See https://github.com/oxidecomputer/typify/pull/221
   
   Alternatively, the `build.rs` could just emit the tokens unformatted (to remove the build-time dependency on `prettyplease` and `syn`), but that seems annoying if and when you need to look at the generated code.
   
   FWIW `syn` is an existing dependency; `prettyplease` is the only new new crate I see in `Cargo.lock`.
   
   I can share the full diff between the old and new versions of the `substrait_text.rs`, but here's a sample:
   
   ```diff
   @@ -1593,22 +1831,27 @@
                T: std::convert::TryInto<Option<super::SessionDependent>>,
                T::Error: std::fmt::Display,
            {
   -            self.session_dependent = value.try_into().map_err(|e| {
   -                format!(
   -                    "error converting supplied value for session_dependent: {}",
   -                    e
   -                )
   -            });
                self
   +                .session_dependent = value
   +                .try_into()
   +                .map_err(|e| {
   +                    format!(
   +                        "error converting supplied value for session_dependent: {}", e
   +                    )
   +                });
   +            self
            }
            pub fn variadic<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::VariadicBehavior>>,
                T::Error: std::fmt::Display,
            {
   -            self.variadic = value
   +            self
   +                .variadic = value
                    .try_into()
   -                .map_err(|e| format!("error converting supplied value for variadic: {}", e));
   +                .map_err(|e| {
   +                    format!("error converting supplied value for variadic: {}", e)
   +                });
                self
            }
            pub fn window_type<T>(mut self, value: T) -> Self
   ```

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 4 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#73](https://github.com/substrait-io/substrait-rs/issues/73)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#73](https://github.com/substrait-io/substrait-rs/issues/73)**
    - Replace removed `typfify::TypeSpace::to_string()` with `prettyplease` ([`3991a0f`](https://github.com/substrait-io/substrait-rs/commit/3991a0f9e25edf9e7ecce112916f322f39089702))
 * **Uncategorized**
    - Release substrait v0.5.3 ([`622b4bb`](https://github.com/substrait-io/substrait-rs/commit/622b4bb251e47a7d60b80c372633bd0348034bbe))
</details>

## 0.5.2 (2023-03-24)

<csr-id-ef41bcf9c346095e0f930211c688387d60e2ec8d/>

### Chore

 - <csr-id-ef41bcf9c346095e0f930211c688387d60e2ec8d/> add commitlint config file to disable max line length limits
   Adds a [commitlint configuration file](https://commitlint.js.org/#/reference-configuration) that "disables" the max (line) length rules. These limits are not required by the conventional commits specification, and they don't work well with dependabot PRs and long URLs in PR descriptions.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#77](https://github.com/substrait-io/substrait-rs/issues/77)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#77](https://github.com/substrait-io/substrait-rs/issues/77)**
    - Add commitlint config file to disable max line length limits ([`ef41bcf`](https://github.com/substrait-io/substrait-rs/commit/ef41bcf9c346095e0f930211c688387d60e2ec8d))
 * **Uncategorized**
    - Release substrait v0.5.2 ([`7964825`](https://github.com/substrait-io/substrait-rs/commit/7964825e0566b79235ba10e5d58c54d1e839d15e))
</details>

## 0.5.1 (2023-03-22)

### New Features

 - <csr-id-c3b72ae3c29744ed3cf6bb6ad7f438449b1e85ac/> add `version` module with Substrait version information
   This PR adds a `version` module that relies on some additions to the build
   script to provide information about the version of the Substrait submodule that
   was used to build the crate. It should be helpful for producers to populate the
   version field of a plan.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 6 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#46](https://github.com/substrait-io/substrait-rs/issues/46)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#46](https://github.com/substrait-io/substrait-rs/issues/46)**
    - Add `version` module with Substrait version information ([`c3b72ae`](https://github.com/substrait-io/substrait-rs/commit/c3b72ae3c29744ed3cf6bb6ad7f438449b1e85ac))
 * **Uncategorized**
    - Release substrait v0.5.1 ([`e177e14`](https://github.com/substrait-io/substrait-rs/commit/e177e14d2387e2b6025c89e67f1e3e03503a5bca))
</details>

## 0.5.0 (2023-03-15)

<csr-id-9a562fa749d8fb51806261982d357c282130eea6/>

### Chore (BREAKING)

 - <csr-id-9a562fa749d8fb51806261982d357c282130eea6/> bump `prost-wkt` dependencies to 0.4

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#59](https://github.com/substrait-io/substrait-rs/issues/59)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#59](https://github.com/substrait-io/substrait-rs/issues/59)**
    - Bump `prost-wkt` dependencies to 0.4 ([`9a562fa`](https://github.com/substrait-io/substrait-rs/commit/9a562fa749d8fb51806261982d357c282130eea6))
 * **Uncategorized**
    - Release substrait v0.5.0 ([`749f958`](https://github.com/substrait-io/substrait-rs/commit/749f958a61a1969a5325dae3530919a97d7d3789))
</details>

## 0.4.2 (2023-03-14)

<csr-id-e52ab6f38fdd96a113f646de85e1fdd4b38e7548/>
<csr-id-39e3441003de1b4c315a82a75fdbc9bb31bd1de3/>

### Chore

 - <csr-id-e52ab6f38fdd96a113f646de85e1fdd4b38e7548/> update crates index when creating releases
   As suggested by a failed run:
   ```console
   [WARN ] Consider running with --update-crates-index to assure bumping on demand
   uses the latest information
   ```
   This adds `--update-crates-index` to the smart-release invocation.
 - <csr-id-39e3441003de1b4c315a82a75fdbc9bb31bd1de3/> also check PR body for conventional commits
   Following the main Substrait repository: this makes the PR check match the Merge
   check by also checking the body of the PR.  
   
   The PR comment note is moved to the job summary.
   
   Added a note about the use of
   [`cargo-smart-release`](https://github.com/Byron/gitoxide/tree/main/cargo-smart-release)
   to the contributing guide.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 6 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 2 unique issues were worked on: [#57](https://github.com/substrait-io/substrait-rs/issues/57), [#72](https://github.com/substrait-io/substrait-rs/issues/72)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#57](https://github.com/substrait-io/substrait-rs/issues/57)**
    - Also check PR body for conventional commits ([`39e3441`](https://github.com/substrait-io/substrait-rs/commit/39e3441003de1b4c315a82a75fdbc9bb31bd1de3))
 * **[#72](https://github.com/substrait-io/substrait-rs/issues/72)**
    - Update crates index when creating releases ([`e52ab6f`](https://github.com/substrait-io/substrait-rs/commit/e52ab6f38fdd96a113f646de85e1fdd4b38e7548))
 * **Uncategorized**
    - Release substrait v0.4.2 ([`4321970`](https://github.com/substrait-io/substrait-rs/commit/4321970db81bdf5f64bb8beec04713378e94ba19))
</details>

## 0.4.1 (2023-03-07)

<csr-id-1b193ae332a649b2e0d8a07f1cde98fa90131c3a/>

### Chore

 - <csr-id-1b193ae332a649b2e0d8a07f1cde98fa90131c3a/> ignore dtolnay/rust-toolchain updates
   The way that action works is not really compatible with how dependabot suggests
   updates (e.g. https://github.com/substrait-io/substrait-rs/pull/67) for it so
   this modifies the configuration to ignore those "updates".

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 53 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#69](https://github.com/substrait-io/substrait-rs/issues/69)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#69](https://github.com/substrait-io/substrait-rs/issues/69)**
    - Ignore dtolnay/rust-toolchain updates ([`1b193ae`](https://github.com/substrait-io/substrait-rs/commit/1b193ae332a649b2e0d8a07f1cde98fa90131c3a))
 * **Uncategorized**
    - Release substrait v0.4.1 ([`a0e320f`](https://github.com/substrait-io/substrait-rs/commit/a0e320f805789a180f86a0bb1262c37b774d3df7))
</details>

## 0.4.0 (2023-01-13)

### New Features (BREAKING)

 - <csr-id-f8f50d3907a2fcbc6b5a09ff3c1b8e541c4b227e/> follow conventional commits and setup automated releases

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 28 commits contributed to the release over the course of 268 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 23 unique issues were worked on: [#1](https://github.com/substrait-io/substrait-rs/issues/1), [#10](https://github.com/substrait-io/substrait-rs/issues/10), [#11](https://github.com/substrait-io/substrait-rs/issues/11), [#14](https://github.com/substrait-io/substrait-rs/issues/14), [#2](https://github.com/substrait-io/substrait-rs/issues/2), [#23](https://github.com/substrait-io/substrait-rs/issues/23), [#26](https://github.com/substrait-io/substrait-rs/issues/26), [#27](https://github.com/substrait-io/substrait-rs/issues/27), [#29](https://github.com/substrait-io/substrait-rs/issues/29), [#30](https://github.com/substrait-io/substrait-rs/issues/30), [#31](https://github.com/substrait-io/substrait-rs/issues/31), [#32](https://github.com/substrait-io/substrait-rs/issues/32), [#33](https://github.com/substrait-io/substrait-rs/issues/33), [#35](https://github.com/substrait-io/substrait-rs/issues/35), [#37](https://github.com/substrait-io/substrait-rs/issues/37), [#39](https://github.com/substrait-io/substrait-rs/issues/39), [#4](https://github.com/substrait-io/substrait-rs/issues/4), [#40](https://github.com/substrait-io/substrait-rs/issues/40), [#41](https://github.com/substrait-io/substrait-rs/issues/41), [#42](https://github.com/substrait-io/substrait-rs/issues/42), [#48](https://github.com/substrait-io/substrait-rs/issues/48), [#5](https://github.com/substrait-io/substrait-rs/issues/5), [#6](https://github.com/substrait-io/substrait-rs/issues/6)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1](https://github.com/substrait-io/substrait-rs/issues/1)**
    - Rust bindings for substrait.io ([`2fb8390`](https://github.com/substrait-io/substrait-rs/commit/2fb83909ca5cb6770f1351441a7222f56b2f174e))
 * **[#10](https://github.com/substrait-io/substrait-rs/issues/10)**
    - Add repo-token to setup-protoc action to avoid rate limiting ([`1b03e84`](https://github.com/substrait-io/substrait-rs/commit/1b03e8497094fc4338995b05a0d5fd2fd1ae09f0))
 * **[#11](https://github.com/substrait-io/substrait-rs/issues/11)**
    - Change gitsubmodule schedule to match substrait weekly release ([`146979a`](https://github.com/substrait-io/substrait-rs/commit/146979a89006ed401879fb7f9b3862f6c4090c14))
 * **[#14](https://github.com/substrait-io/substrait-rs/issues/14)**
    - Change schedule time for Dependabot submodule updates ([`1774935`](https://github.com/substrait-io/substrait-rs/commit/1774935d3d1a9bec800b6f8f2599ad8256f6ab23))
 * **[#2](https://github.com/substrait-io/substrait-rs/issues/2)**
    - Use latest substrait & bump crate version ([`7852c8a`](https://github.com/substrait-io/substrait-rs/commit/7852c8a33882ae8ebb65dc4fba9a046c6e8d4a2d))
 * **[#23](https://github.com/substrait-io/substrait-rs/issues/23)**
    - Bump pbjson from 0.5.0 to 0.5.1 ([`510fb8f`](https://github.com/substrait-io/substrait-rs/commit/510fb8fdd1d2b3016c733c7b7d6f23ee3192d231))
 * **[#26](https://github.com/substrait-io/substrait-rs/issues/26)**
    - Replace unmaintained action-rs actions ([`cf9c0dd`](https://github.com/substrait-io/substrait-rs/commit/cf9c0dd5d169f899adcaf0540a4fb29c9b74dcc6))
 * **[#27](https://github.com/substrait-io/substrait-rs/issues/27)**
    - Generate types to deserialize simple extensions ([`2747477`](https://github.com/substrait-io/substrait-rs/commit/27474774b8baff84457db94b2c7deefdc152831f))
 * **[#29](https://github.com/substrait-io/substrait-rs/issues/29)**
    - Bump prost from 0.11.0 to 0.11.3 ([`acdf53d`](https://github.com/substrait-io/substrait-rs/commit/acdf53d852ee12c51e3ba07938d8a8f5582541fb))
 * **[#30](https://github.com/substrait-io/substrait-rs/issues/30)**
    - Bump prost-build from 0.11.1 to 0.11.3 ([`12d759b`](https://github.com/substrait-io/substrait-rs/commit/12d759b77a619c8939131d25bce703b94a76f13e))
 * **[#31](https://github.com/substrait-io/substrait-rs/issues/31)**
    - Bump substrait from `018da38` to `7f272f1` ([`a1adbc7`](https://github.com/substrait-io/substrait-rs/commit/a1adbc74a4437abc61567a48b4690340e8fa5544))
 * **[#32](https://github.com/substrait-io/substrait-rs/issues/32)**
    - Add `bors.toml` ([`80c0676`](https://github.com/substrait-io/substrait-rs/commit/80c067668d400911bdd0e357aceb767941727acf))
 * **[#33](https://github.com/substrait-io/substrait-rs/issues/33)**
    - Disable auto rebase for cargo dependabot updates ([`61d41b1`](https://github.com/substrait-io/substrait-rs/commit/61d41b1f26cc145ccfefa16cf15298cea7704576))
 * **[#35](https://github.com/substrait-io/substrait-rs/issues/35)**
    - Bump pbjson-types from 0.5.0 to 0.5.1 ([`9e7c02f`](https://github.com/substrait-io/substrait-rs/commit/9e7c02fbd434f143c229e6f340bf5a36801fe251))
 * **[#37](https://github.com/substrait-io/substrait-rs/issues/37)**
    - Bump serde_json from 1.0.85 to 1.0.89 ([`bfb8031`](https://github.com/substrait-io/substrait-rs/commit/bfb8031008eff6dc98eaf3f22f89995f67d82401))
 * **[#39](https://github.com/substrait-io/substrait-rs/issues/39)**
    - Bump serde from 1.0.144 to 1.0.150 ([`aa6c650`](https://github.com/substrait-io/substrait-rs/commit/aa6c650f104a509a464e2533aaeeae823b58920d))
 * **[#4](https://github.com/substrait-io/substrait-rs/issues/4)**
    - Corrected repo link address in README.md and Cargo.toml ([`d8c6183`](https://github.com/substrait-io/substrait-rs/commit/d8c6183449788fd72f96df854e88f9d515166c02))
 * **[#40](https://github.com/substrait-io/substrait-rs/issues/40)**
    - Bump prost-build from 0.11.3 to 0.11.4 ([`bd104c1`](https://github.com/substrait-io/substrait-rs/commit/bd104c1379648f308599192af361a31d626d7d4a))
 * **[#41](https://github.com/substrait-io/substrait-rs/issues/41)**
    - Bump version to 0.3.0 ([`b852f01`](https://github.com/substrait-io/substrait-rs/commit/b852f01d09c0d6b8c26c6ceebcf19572402b9e6b))
 * **[#42](https://github.com/substrait-io/substrait-rs/issues/42)**
    - Bump substrait from `7f272f1` to `81e34d4` ([`40e8adf`](https://github.com/substrait-io/substrait-rs/commit/40e8adf629f438b57354c23fefad61bba7843f7a))
 * **[#48](https://github.com/substrait-io/substrait-rs/issues/48)**
    - Follow conventional commits and setup automated releases ([`f8f50d3`](https://github.com/substrait-io/substrait-rs/commit/f8f50d3907a2fcbc6b5a09ff3c1b8e541c4b227e))
 * **[#5](https://github.com/substrait-io/substrait-rs/issues/5)**
    - Setup GitHub actions and Dependabot ([`7418a17`](https://github.com/substrait-io/substrait-rs/commit/7418a17078a8ebfd062a17e4cf989a64a1aa52c2))
 * **[#6](https://github.com/substrait-io/substrait-rs/issues/6)**
    - Add `pbjson` feature for Protobuf JSON serde ([`43f97ef`](https://github.com/substrait-io/substrait-rs/commit/43f97ef0e1311bdd909c58afad429b4db54fa30d))
 * **Uncategorized**
    - Release substrait v0.4.0 ([`65d5f3d`](https://github.com/substrait-io/substrait-rs/commit/65d5f3d8f48c3ae990c311ae8ed4590a76e57c3e))
    - Merge #24 ([`c726d2f`](https://github.com/substrait-io/substrait-rs/commit/c726d2fbab168487a975601d12c7b9462842b78b))
    - Bump pbjson-build from 0.5.0 to 0.5.1 ([`032eba1`](https://github.com/substrait-io/substrait-rs/commit/032eba11a6932fabe4fe1bc7f527c995a1056c9f))
    - Update LICENSE ([`06a611b`](https://github.com/substrait-io/substrait-rs/commit/06a611b1e46c39908aa55c51e4a9cb390c9a8091))
    - Initial commit ([`8edc827`](https://github.com/substrait-io/substrait-rs/commit/8edc827a4a3c5ae6de704d3d4238774d379c6c23))
</details>

