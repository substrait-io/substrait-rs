## 0.26.0 (2024-03-04)

<csr-id-3c17273258101f7b228b953a4409fa09718ddbd5/>

### Chore (BREAKING)

- <csr-id-3c17273258101f7b228b953a4409fa09718ddbd5/> bump substrait from `0.43.0` to `0.44.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `5e1948e` to `2e12da1`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/2e12da1b9915eea01f12ece0be9c37afa1384f74"><code>2e12da1</code></a>
  chore(release): 0.44.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/cbec079ea03bec65cc063daa15e42807c4039707"><code>cbec079</code></a>
  feat: add extra option for on domain errors in log functions (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/536">#536</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/55db05b4cf8cbb1e2bf565e4f5f0c6def6f0e6ed"><code>55db05b</code></a>
  feat: add ignore nulls options to concat function (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/605">#605</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/5e1948e7a9e945d347b011e6518d1bb501f8522a...2e12da1b9915eea01f12ece0be9c37afa1384f74">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 1 commit contributed to the release.
- 3 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#156](https://github.com/substrait-io/substrait-rs/issues/156)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#156](https://github.com/substrait-io/substrait-rs/issues/156)** - Bump substrait from `0.43.0` to `0.44.0` ([`3c17273`](https://github.com/substrait-io/substrait-rs/commit/3c17273258101f7b228b953a4409fa09718ddbd5))
</details>

## 0.25.1 (2024-02-29)

<csr-id-cc0e947f1a21249312dfd777f5a624bea0133f41/>

### Chore

- <csr-id-cc0e947f1a21249312dfd777f5a624bea0133f41/> update typify requirement from 0.0.15 to 0.0.16
  Updates the requirements on
  [typify](https://github.com/oxidecomputer/typify) to permit the latest
  version.
  <details>
  <summary>Changelog</summary>
  <p><em>Sourced from <a
  href="https://github.com/oxidecomputer/typify/blob/main/CHANGELOG.adoc">typify's
  changelog</a>.</em></p>
  <blockquote>
  <p>== 0.0.16 (released 2024-02-28)</p>
  <ul>
  <li>Introduce a proper Error type for various conversions (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/475">#475</a>)</li>
  <li>Add docs to generated mods (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/476">#476</a>)</li>
  <li>Various enum improvements</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.15%5C...v0.0.16%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.15\...v0.0.16[Full</a>
  list of commits]</p>
  <p>== 0.0.15 (released 2023-12-15)</p>
  <ul>
  <li>Improvements to array merging and mutual exclusivity checks (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/412">#412</a>)</li>
  <li>Support for 32-bit floating-point numbers (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/440">#440</a>)</li>
  <li>Better handling for unsatisfiable merged schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/447">#447</a>)</li>
  <li>Show original JSON Schema in generated type docs (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/454">#454</a>)</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.14%5C...v0.0.15%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.14\...v0.0.15[Full</a>
  list of commits]</p>
  <p>== 0.0.14 (released 2023-09-25)</p>
  <ul>
  <li>Handle arbitrary containment cycles (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/300">#300</a>)</li>
  <li>More permissive of valid (if useless) schema constructions (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/306">#306</a>,
  <a
  href="https://redirect.github.com/oxidecomputer/typify/issues/320">#320</a>)</li>
  <li>Much better handling of <code>allOf</code> constructions by merging
  schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/405">#405</a>)</li>
  <li>Support for more <code>not</code> subschemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/410">#410</a>)</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.13%5C...v0.0.14%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.13\...v0.0.14[Full</a>
  list of commits]</p>
  <p>== 0.0.13 (released 2023-05-14)</p>
  <ul>
  <li>Fixed-length, single-type arrays to <code>[T; N]</code> (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/286">#286</a>)</li>
  <li>Support for reflexive schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/292">#292</a>)</li>
  <li>Much improved support for multi-type schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/291">#291</a>)</li>
  <li>Better error messages on failures</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.12%5C...v0.0.13%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.12\...v0.0.13[Full</a>
  list of commits]</p>
  <p>== 0.0.12 (released 2023-05-03)</p>
  <ul>
  <li>Improved enum generation (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/270">#270</a>)</li>
  <li>Improved integer type selection based on number criteria (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/255">#255</a>)</li>
  <li><code>TypeSpace::add_root_schema()</code> (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/236">#236</a>)</li>
  <li>... and many general improvements</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.11%5C...v0.0.12%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.11\...v0.0.12[Full</a>
  list of commits]</p>
  <p>== 0.0.11 (released 2023-03-18)</p>
  <p>This is a big update with many, many changes to code generation, and
  many more
  JSON schema structures well-handled. Among the many changes:</p>
  <ul>
  <li>Generate a <code>ToString</code> impl for untagged enums with
  trivial variants (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/145">#145</a>)</li>
  </ul>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/82a040e7eb8ec037cb703b5a7fc4ac1c692c7170"><code>82a040e</code></a>
  release typify 0.0.16</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/c6e4947846a2ea00aa998a0679bcc03e736e0d8f"><code>c6e4947</code></a>
  update changelog for next release</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/131fe0ef3722d3034a61a8ab2994c29f9c978903"><code>131fe0e</code></a>
  Bump clap from 4.4.18 to 4.5.1 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/509">#509</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/78dcbffb83d9faa88153b33503d64c23ec090b95"><code>78dcbff</code></a>
  Bump syn from 2.0.48 to 2.0.51 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/511">#511</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/cc35d40cdaa6b6c1e70e5338fe7954ac0df61995"><code>cc35d40</code></a>
  Bump assert_cmd from 2.0.13 to 2.0.14 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/512">#512</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/e0a11a4e085a83498427e007a55d0b5366d131ce"><code>e0a11a4</code></a>
  1.76 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/515">#515</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/113f8f9751c8ab0ce7b277fa05ea7de39bfdf9b4"><code>113f8f9</code></a>
  Bump serde from 1.0.196 to 1.0.197 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/513">#513</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/92945bc1bdab8106189f2af3947bf74ee69a2947"><code>92945bc</code></a>
  Bump serde_json from 1.0.113 to 1.0.114 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/514">#514</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/ce009d6f83b620cbd0e3acdd9b9ea071018471d8"><code>ce009d6</code></a>
  Bump thiserror from 1.0.56 to 1.0.57 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/505">#505</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/b3beca8e708f1367cd7cca74816bdca6b4a18c4a"><code>b3beca8</code></a>
  Bump chrono from 0.4.33 to 0.4.34 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/506">#506</a>)</li>
  <li>Additional commits viewable in <a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.15...v0.0.16">compare
  view</a></li>
  </ul>
  </details>
  <br />

  You can trigger a rebase of this PR by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 2 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#155](https://github.com/substrait-io/substrait-rs/issues/155)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#155](https://github.com/substrait-io/substrait-rs/issues/155)**
  - Update typify requirement from 0.0.15 to 0.0.16 ([`cc0e947`](https://github.com/substrait-io/substrait-rs/commit/cc0e947f1a21249312dfd777f5a624bea0133f41))
- **Uncategorized** - Release substrait v0.25.1 ([`17cdbeb`](https://github.com/substrait-io/substrait-rs/commit/17cdbeb513010971108903c70046c53541e4bbb8))
</details>

## 0.25.0 (2024-02-26)

<csr-id-1290ec113f03ef259c12fdcee5518470ee1f0bc0/>

### Chore (BREAKING)

- <csr-id-1290ec113f03ef259c12fdcee5518470ee1f0bc0/> bump substrait from `0.42.1` to `0.43.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `4734478` to `5e1948e`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/5e1948e7a9e945d347b011e6518d1bb501f8522a"><code>5e1948e</code></a>
  chore(release): 0.43.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/087f87c0307572cf2e9a7d1db7fdd673662699c3"><code>087f87c</code></a>
  feat: include precision parameter in timestamp types (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/594">#594</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/a3b1f32b0e6aac08bf0ee7437a5ae1c10100a859"><code>a3b1f32</code></a>
  fix: remove function definitions w/ invalid return types (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/599">#599</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/d9b9672fd3c24285afdee9344fc2f4f7fcd70afb"><code>d9b9672</code></a>
  docs: fix link to protobuf files (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/598">#598</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/3ded94d69e7ae329f2730b7634ba34900250e84a"><code>3ded94d</code></a>
  docs: fix code blocks in field_references.md (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/595">#595</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/47344783dce74645dcb636cb646cd3628df37ef0...5e1948e7a9e945d347b011e6518d1bb501f8522a">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 13 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#154](https://github.com/substrait-io/substrait-rs/issues/154)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#154](https://github.com/substrait-io/substrait-rs/issues/154)**
  - Bump substrait from `0.42.1` to `0.43.0` ([`1290ec1`](https://github.com/substrait-io/substrait-rs/commit/1290ec113f03ef259c12fdcee5518470ee1f0bc0))
- **Uncategorized** - Release substrait v0.25.0 ([`2e188ab`](https://github.com/substrait-io/substrait-rs/commit/2e188ab4365d82335a84f0c45bd3b7d0058e4b4f))
</details>

## 0.24.2 (2024-02-12)

### Bug Fixes

- <csr-id-9f08ea67024fdbc4c58b90521af14cf876292956/> upgrade git2 to resolve CVEs

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 11 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#153](https://github.com/substrait-io/substrait-rs/issues/153)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#153](https://github.com/substrait-io/substrait-rs/issues/153)**
  - Upgrade git2 to resolve CVEs ([`9f08ea6`](https://github.com/substrait-io/substrait-rs/commit/9f08ea67024fdbc4c58b90521af14cf876292956))
- **Uncategorized** - Release substrait v0.24.2 ([`da71870`](https://github.com/substrait-io/substrait-rs/commit/da71870d377a90e71cabd6e50fa7efc66eddbab2))
</details>

## 0.24.1 (2024-02-01)

<csr-id-2c392122195e3059f8afbffeb8bdd7f988ed7fe5/>

### Chore

- <csr-id-2c392122195e3059f8afbffeb8bdd7f988ed7fe5/> bump arduino/setup-protoc from 2 to 3
  Bumps [arduino/setup-protoc](https://github.com/arduino/setup-protoc)
  from 2 to 3.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/arduino/setup-protoc/releases">arduino/setup-protoc's
  releases</a>.</em></p>
  <blockquote>
  <h2>v3.0.0</h2>
  <h2>What's Changed</h2>
  <ul>
  <li>Correct <code>convetion</code> typo in README by <a
              href="https://github.com/nixpanic"><code>@​nixpanic</code></a> in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/91">arduino/setup-protoc#91</a></li>
  <li>Bump <code>@​babel/traverse</code> from 7.22.1 to 7.23.2 by <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/93">arduino/setup-protoc#93</a></li>
  <li>Upgrade to node 20 by <a
              href="https://github.com/alessio-perugini"><code>@​alessio-perugini</code></a>
  in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/95">arduino/setup-protoc#95</a></li>
  </ul>
  <h2>New Contributors</h2>
  <ul>
  <li><a href="https://github.com/nixpanic"><code>@​nixpanic</code></a>
  made their first contribution in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/91">arduino/setup-protoc#91</a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/arduino/setup-protoc/compare/v2.1.0...v3.0.0">https://github.com/arduino/setup-protoc/compare/v2.1.0...v3.0.0</a></p>
  <h2>v2.1.0</h2>
  <h2>What's Changed</h2>
  <ul>
  <li>Expose <code>path</code> and <code>version</code> in
  <code>outputs</code> by <a
              href="https://github.com/sebastienvermeille"><code>@​sebastienvermeille</code></a>
  in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/89">arduino/setup-protoc#89</a></li>
  <li>Bump semver from 7.5.1 to 7.5.2 by <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/87">arduino/setup-protoc#87</a></li>
  <li>bump semver to 7.5.3 by <a
              href="https://github.com/alessio-perugini"><code>@​alessio-perugini</code></a>
  in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/90">arduino/setup-protoc#90</a></li>
  </ul>
  <h2>New Contributors</h2>
  <ul>
  <li><a
              href="https://github.com/sebastienvermeille"><code>@​sebastienvermeille</code></a>
  made their first contribution in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/89">arduino/setup-protoc#89</a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/arduino/setup-protoc/compare/v2.0.0...v2.1.0">https://github.com/arduino/setup-protoc/compare/v2.0.0...v2.1.0</a></p>
  </blockquote>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/arduino/setup-protoc/commit/c65c819552d16ad3c9b72d9dfd5ba5237b9c906b"><code>c65c819</code></a>
  Upgrade to node 20 (<a
              href="https://redirect.github.com/arduino/setup-protoc/issues/95">#95</a>)</li>
  <li><a
              href="https://github.com/arduino/setup-protoc/commit/52a53b4e2d968277c5c749dac537d0b14a6f5272"><code>52a53b4</code></a>
  Merge pull request <a
              href="https://redirect.github.com/arduino/setup-protoc/issues/93">#93</a>
  from arduino/dependabot/npm_and_yarn/babel/traverse-7....</li>
  <li><a
              href="https://github.com/arduino/setup-protoc/commit/cf7ab7fe8696fefcafb8135834d49955e824a56b"><code>cf7ab7f</code></a>
  Bump <code>@​babel/traverse</code> from 7.22.1 to 7.23.2</li>
  <li><a
              href="https://github.com/arduino/setup-protoc/commit/e2995ba278e6b4bca9bac954e72667db122abed1"><code>e2995ba</code></a>
  Correct <code>convetion</code> typo in README (<a
              href="https://redirect.github.com/arduino/setup-protoc/issues/91">#91</a>)</li>
  <li>See full diff in <a
              href="https://github.com/arduino/setup-protoc/compare/v2...v3">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=arduino/setup-protoc&package-manager=github_actions&previous-version=2&new-version=3)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 2 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#151](https://github.com/substrait-io/substrait-rs/issues/151)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#151](https://github.com/substrait-io/substrait-rs/issues/151)**
  - Bump arduino/setup-protoc from 2 to 3 ([`2c39212`](https://github.com/substrait-io/substrait-rs/commit/2c392122195e3059f8afbffeb8bdd7f988ed7fe5))
- **Uncategorized** - Release substrait v0.24.1 ([`452a41e`](https://github.com/substrait-io/substrait-rs/commit/452a41e6a7dce461fcf094ea0fe833af7fe74963))
</details>

## 0.24.0 (2024-01-29)

<csr-id-4a3583f8d7a416ba34af9ad1966bc349e7bd1d83/>

### Chore (BREAKING)

- <csr-id-4a3583f8d7a416ba34af9ad1966bc349e7bd1d83/> bump substrait from `0.42.0` to `0.42.1`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `3251b1f` to `4734478`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/47344783dce74645dcb636cb646cd3628df37ef0"><code>4734478</code></a>
  chore(release): 0.42.1</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/a4cf7417e348ce9683b558f1b0a274bf55af8c30"><code>a4cf741</code></a>
  chore(deps): Bump bufbuild/buf-setup-action from 1.28.1 to 1.29.0 (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/593">#593</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/8b9535c9a55ef68f83ca6fee1552646bd33757e6"><code>8b9535c</code></a>
  docs: add more explanation of comparison function (in hash/merge join)
  (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/586">#586</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/54454cbfec562af987edfa410e75a3dd203c12ad"><code>54454cb</code></a>
  chore(deps): Bump actions/cache from 3 to 4 (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/589">#589</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/543f740bcb7455650a1ded8f21729406926146d7"><code>543f740</code></a>
  refactor: prefer boolean over BOOLEAN (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/590">#590</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/d55703a18a7a8f2ecf695f9367ca33fab6b1ef33"><code>d55703a</code></a>
  fix: add missing RelCommon field to WriteRel and DdlRel (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/591">#591</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/3251b1fc5ede5788502be989b8eab778051d7a4d...47344783dce74645dcb636cb646cd3628df37ef0">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 7 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#150](https://github.com/substrait-io/substrait-rs/issues/150)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#150](https://github.com/substrait-io/substrait-rs/issues/150)**
  - Bump substrait from `0.42.0` to `0.42.1` ([`4a3583f`](https://github.com/substrait-io/substrait-rs/commit/4a3583f8d7a416ba34af9ad1966bc349e7bd1d83))
- **Uncategorized** - Release substrait v0.24.0 ([`9c2bf1b`](https://github.com/substrait-io/substrait-rs/commit/9c2bf1bee99650005e18aa3f4aed77711f7d0db7))
</details>

## 0.23.0 (2024-01-22)

<csr-id-952c90f50a1aeaac48039e6ac8f0d4f2e70e2ad3/>
<csr-id-33aaf9b613ccc804265678f192f864d7e5a2b5a7/>
<csr-id-c2e596b124f95fc1364f96174d0bb104081be62d/>

### Chore

- <csr-id-952c90f50a1aeaac48039e6ac8f0d4f2e70e2ad3/> bump actions/upload-pages-artifact from 2 to 3
  Bumps
  [actions/upload-pages-artifact](https://github.com/actions/upload-pages-artifact)
  from 2 to 3.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/actions/upload-pages-artifact/releases">actions/upload-pages-artifact's
  releases</a>.</em></p>
  <blockquote>
  <h2>v3.0.0</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Use <code>v4</code> upload-artifact tag <a
              href="https://github.com/robherley"><code>@​robherley</code></a> (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/80">#80</a>)</li>
  <li>Upload pages artifact with upload-artifact v4-beta <a
              href="https://github.com/konradpabjan"><code>@​konradpabjan</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/78">#78</a>)</li>
  </ul>
  <p>To deploy a GitHub Pages site which has been uploaded with his
  version of <code>actions/upload-pages-artifact</code>, you must also use
  <code>actions/deploy-pages@v4</code> or newer.</p>
  <p>See details of <a
              href="https://github.com/actions/upload-pages-artifact/compare/v2.0.0...v3.0.0">all
  code changes</a> since previous release.</p>
  </blockquote>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/0252fc4ba7626f0298f0cf00902a25c6afc77fa8"><code>0252fc4</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/81">#81</a>
  from actions/artifacts-next</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/2a5c1440746537a84f699a04465e7ac8a64879e4"><code>2a5c144</code></a>
  Use actions/download-artifact@v4 in test</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/7e3f6bb53bf41c0cc32c369403f7fcbb8989c9ae"><code>7e3f6bb</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/80">#80</a>
  from robherley/patch-1</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/257e666c0505b64d304755738c09e8a1f32ff7e0"><code>257e666</code></a>
  Use <code>v4</code> upload-artifact tag</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/0313a19afa9efdf74b4e31c9554d35f744213f0e"><code>0313a19</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/78">#78</a>
  from konradpabjan/main</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/1228e656868935b3c8b085b28ea3b89b5763c64e"><code>1228e65</code></a>
  Update action.yml</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/eb31309b6cdf94f92e47abb173a0cbed9b5056cf"><code>eb31309</code></a>
  Update artifact names in tests</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/241a975ec2c9ecb3effc2875203cce4b9e25078d"><code>241a975</code></a>
  Correct artifact name during download</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/ef95519d72c61a2dd9137c79470bacfc5bdbbaa8"><code>ef95519</code></a>
  Unique artifact name per job</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/ecdd3edff76d306e98d046dfc846e651354ac144"><code>ecdd3ed</code></a>
  Switch to using download@v4-beta</li>
  <li>Additional commits viewable in <a
              href="https://github.com/actions/upload-pages-artifact/compare/v2...v3">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/upload-pages-artifact&package-manager=github_actions&previous-version=2&new-version=3)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

- <csr-id-33aaf9b613ccc804265678f192f864d7e5a2b5a7/> bump actions/deploy-pages from 3 to 4
  Bumps [actions/deploy-pages](https://github.com/actions/deploy-pages)
  from 3 to 4.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/actions/deploy-pages/releases">actions/deploy-pages's
  releases</a>.</em></p>
  <blockquote>
  <h2>v4.0.0</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Deploy pages using artifact IDs <a
              href="https://github.com/konradpabjan"><code>@​konradpabjan</code></a>
  (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/251">#251</a>)</li>
  </ul>
  <hr />
  <p>:warning: This version of <code>actions/deploy-pages</code> is
  <strong>ONLY</strong> compatible with artifacts uploaded by either:</p>
  <ul>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/releases/tag/v3.0.0"><code>actions/upload-pages-artifact@v3</code></a>
  or newer</li>
  <li><a
              href="https://github.com/actions/upload-artifact/releases/tag/v4.0.0"><code>actions/upload-artifact@v4</code></a>
  or newer.</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/deploy-pages/compare/v3.0.1...v4.0.0">all
  code changes</a> since previous release.</p>
  <p>:warning: For use with products other than GitHub.com, such as GitHub
  Enterprise Server, please consult the <a
              href="https://github.com/actions/deploy-pages/#compatibilty">compatibility
  table</a>.</p>
  <h2>v3.0.1</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Bump eslint from 8.54.0 to 8.55.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/266">#266</a>)</li>
  <li>Bump nock from 13.3.8 to 13.4.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/267">#267</a>)</li>
  <li>Bump eslint-config-prettier from 9.0.0 to 9.1.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/268">#268</a>)</li>
  <li>Bump <code>@​actions/core</code> from 1.10.0 to 1.10.1 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/269">#269</a>)</li>
  <li>Bump <code>@​actions/github</code> from 5.1.1 to 6.0.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/261">#261</a>)</li>
  <li>Update compatibility table for v3 <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/270">#270</a>)</li>
  </ul>
  <h2>🧰 Maintenance</h2>
  <ul>
  <li>chore/docs: update version, fix typos <a
              href="https://github.com/kbdharun"><code>@​kbdharun</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/272">#272</a>)</li>
  </ul>
  <hr />
  <p>See details of <a
              href="https://github.com/actions/deploy-pages/compare/v3.0.0...v3.0.1">all
  code changes</a> since previous release.</p>
  <p>:warning: For use with products other than GitHub.com, such as GitHub
  Enterprise Server, please consult the <a
              href="https://github.com/actions/deploy-pages/#compatibilty">compatibility
  table</a>.</p>
  </blockquote>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/actions/deploy-pages/commit/f33f41b675f0ab2dc5a6863c9a170fe83af3571e"><code>f33f41b</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/deploy-pages/issues/279">#279</a>
  from actions/artifacts-next-ga</li>
  <li><a
              href="https://github.com/actions/deploy-pages/commit/0d45f33cdeb74801a7a5c5af95dceac9fa933818"><code>0d45f33</code></a>
  pretty</li>
  <li><a
              href="https://github.com/actions/deploy-pages/commit/d1e23d0efd9866b3e474883905cb86712868e97b"><code>d1e23d0</code></a>
  Merge branch 'main' into artifacts-next-ga</li>
  <li>See full diff in <a
              href="https://github.com/actions/deploy-pages/compare/v3...v4">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/deploy-pages&package-manager=github_actions&previous-version=3&new-version=4)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Chore (BREAKING)

- <csr-id-c2e596b124f95fc1364f96174d0bb104081be62d/> bump substrait from `0.41.0` to `0.42.0`

### Commit Statistics

<csr-read-only-do-not-edit/>

- 4 commits contributed to the release over the course of 13 calendar days.
- 13 days passed between releases.
- 3 commits were understood as [conventional](https://www.conventionalcommits.org).
- 3 unique issues were worked on: [#145](https://github.com/substrait-io/substrait-rs/issues/145), [#146](https://github.com/substrait-io/substrait-rs/issues/146), [#149](https://github.com/substrait-io/substrait-rs/issues/149)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#145](https://github.com/substrait-io/substrait-rs/issues/145)**
  - Bump actions/deploy-pages from 3 to 4 ([`33aaf9b`](https://github.com/substrait-io/substrait-rs/commit/33aaf9b613ccc804265678f192f864d7e5a2b5a7))
- **[#146](https://github.com/substrait-io/substrait-rs/issues/146)**
  - Bump actions/upload-pages-artifact from 2 to 3 ([`952c90f`](https://github.com/substrait-io/substrait-rs/commit/952c90f50a1aeaac48039e6ac8f0d4f2e70e2ad3))
- **[#149](https://github.com/substrait-io/substrait-rs/issues/149)**
  - Bump substrait from `0.41.0` to `0.42.0` ([`c2e596b`](https://github.com/substrait-io/substrait-rs/commit/c2e596b124f95fc1364f96174d0bb104081be62d))
- **Uncategorized** - Release substrait v0.23.0 ([`4441c9b`](https://github.com/substrait-io/substrait-rs/commit/4441c9b7563ac9654af996b21e3f1602aa4171de))
</details>

## 0.22.0 (2024-01-08)

<csr-id-aec95c21814736286418070bd1e88760f1c1e92f/>

### Chore (BREAKING)

- <csr-id-aec95c21814736286418070bd1e88760f1c1e92f/> bump substrait from `0.40.0` to `0.41.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `cdae23e` to `c7d7e9c`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/c7d7e9c5393aaa5e63b4a18ce65e423faa5eb9dd"><code>c7d7e9c</code></a>
  chore(release): 0.41.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/aba1bc79acc5bf40a719b23276bfa6f7546e7ed5"><code>aba1bc7</code></a>
  fix: renamed modulus to modulo; updated modulo operator defintion (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/583">#583</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/cdae23ea0f5784243efa60a5361fabf31127457f...c7d7e9c5393aaa5e63b4a18ce65e423faa5eb9dd">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 22 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#147](https://github.com/substrait-io/substrait-rs/issues/147)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#147](https://github.com/substrait-io/substrait-rs/issues/147)**
  - Bump substrait from `0.40.0` to `0.41.0` ([`aec95c2`](https://github.com/substrait-io/substrait-rs/commit/aec95c21814736286418070bd1e88760f1c1e92f))
- **Uncategorized** - Release substrait v0.22.0 ([`a605313`](https://github.com/substrait-io/substrait-rs/commit/a605313678b63b7a01be85069282645a9fb60938))
</details>

## 0.21.0 (2023-12-17)

<csr-id-e24c5a3b082bb1c4e8e2329efec4762fd79ff60a/>

### Chore (BREAKING)

- <csr-id-e24c5a3b082bb1c4e8e2329efec4762fd79ff60a/> bump substrait from `0.39.0` to `0.40.0`

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 1 day passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#144](https://github.com/substrait-io/substrait-rs/issues/144)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#144](https://github.com/substrait-io/substrait-rs/issues/144)**
  - Bump substrait from `0.39.0` to `0.40.0` ([`e24c5a3`](https://github.com/substrait-io/substrait-rs/commit/e24c5a3b082bb1c4e8e2329efec4762fd79ff60a))
- **Uncategorized** - Release substrait v0.21.0 ([`33a9669`](https://github.com/substrait-io/substrait-rs/commit/33a966916dae84c65b45d9e7bc550be635225156))
</details>

## 0.20.3 (2023-12-15)

<csr-id-8d9886a420488bf57ae81015571347e043b56e8f/>

### Chore

- <csr-id-8d9886a420488bf57ae81015571347e043b56e8f/> update typify requirement from 0.0.14 to 0.0.15
  Updates the requirements on
  [typify](https://github.com/oxidecomputer/typify) to permit the latest
  version.
  <details>
  <summary>Changelog</summary>
  <p><em>Sourced from <a
  href="https://github.com/oxidecomputer/typify/blob/main/CHANGELOG.adoc">typify's
  changelog</a>.</em></p>
  <blockquote>
  <p>== 0.0.15 (released 2023-12-15)</p>
  <ul>
  <li>Improvements to array merging and mutual exclusivity checks (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/412">#412</a>)</li>
  <li>Support for 32-bit floating-point numbers (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/440">#440</a>)</li>
  <li>Better handling for unsatisfiable merged schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/447">#447</a>)</li>
  <li>Show original JSON Schema in generated type docs (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/454">#454</a>)</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.14%5C...v0.0.15%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.14\...v0.0.15[Full</a>
  list of commits]</p>
  <p>== 0.0.14 (released 2023-09-25)</p>
  <ul>
  <li>Handle arbitrary containment cycles (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/300">#300</a>)</li>
  <li>More permissive of valid (if useless) schema constructions (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/306">#306</a>,
  <a
  href="https://redirect.github.com/oxidecomputer/typify/issues/320">#320</a>)</li>
  <li>Much better handling of <code>allOf</code> constructions by merging
  schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/405">#405</a>)</li>
  <li>Support for more <code>not</code> subschemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/410">#410</a>)</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.13%5C...v0.0.14%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.13\...v0.0.14[Full</a>
  list of commits]</p>
  <p>== 0.0.13 (released 2023-05-14)</p>
  <ul>
  <li>Fixed-length, single-type arrays to <code>[T; N]</code> (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/286">#286</a>)</li>
  <li>Support for reflexive schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/292">#292</a>)</li>
  <li>Much improved support for multi-type schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/291">#291</a>)</li>
  <li>Better error messages on failures</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.12%5C...v0.0.13%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.12\...v0.0.13[Full</a>
  list of commits]</p>
  <p>== 0.0.12 (released 2023-05-03)</p>
  <ul>
  <li>Improved enum generation (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/270">#270</a>)</li>
  <li>Improved integer type selection based on number criteria (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/255">#255</a>)</li>
  <li><code>TypeSpace::add_root_schema()</code> (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/236">#236</a>)</li>
  <li>... and many general improvements</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.11%5C...v0.0.12%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.11\...v0.0.12[Full</a>
  list of commits]</p>
  <p>== 0.0.11 (released 2023-03-18)</p>
  <p>This is a big update with many, many changes to code generation, and
  many more
  JSON schema structures well-handled. Among the many changes:</p>
  <ul>
  <li>Generate a <code>ToString</code> impl for untagged enums with
  trivial variants (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/145">#145</a>)</li>
  <li>Allow conversion overrides by specifying a schema (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/155">#155</a>)</li>
  <li>Handle untyped enums that contain nulls (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/167">#167</a>)</li>
  <li>Handle <code>not</code> schemas for enumerated values (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/168">#168</a>)</li>
  <li>Improve generated code for FromStr and TryFrom impls (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/174">#174</a>)</li>
  <li>Handle format specifiers for enumerated strings (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/188">#188</a>)</li>
  </ul>
  <p>=== <em>Breaking</em>: The removal of
  <code>TypeSpace::to_string()</code></p>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/0894f06e6798f24efcbd7134746962aa0e6eb371"><code>0894f06</code></a>
  release typify 0.0.15</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/cafdeeb82f9af69ceb344ef0e356653dbb0b8521"><code>cafdeeb</code></a>
  use Default::default rather than buidler::#type_name::Default to avoid
  confli...</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/048828b669f4b472db4750dfc599eb8bc5b70b78"><code>048828b</code></a>
  update Cargo.lock</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/41c581a465202b77ff100dcca32b0bfea073d145"><code>41c581a</code></a>
  update changelog (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/455">#455</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/b0df7aa37ae13b61705161075146250224df24f2"><code>b0df7aa</code></a>
  keep json; include in doc comments (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/454">#454</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/d1b6c7d97619b7398e03f4cb27557529c3c7fe75"><code>d1b6c7d</code></a>
  Bump syn from 2.0.39 to 2.0.40 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/453">#453</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/a5a27d5c02d5346a1e4fb027543bdfa22170eaf6"><code>a5a27d5</code></a>
  Bump clap from 4.4.10 to 4.4.11 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/452">#452</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/d6d85998d29825cfcf07072423f567a085db3ee0"><code>d6d8599</code></a>
  Bump clap from 4.4.8 to 4.4.10 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/449">#449</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/2c52a39a0465caf9debfb67443e218d033ed468b"><code>2c52a39</code></a>
  fix handling of references within allOf subschemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/448">#448</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/8b83fbf0a59d8e6d474e49cdd1657c663eba7284"><code>8b83fbf</code></a>
  unsatisfiable allOf construction panics (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/447">#447</a>)</li>
  <li>Additional commits viewable in <a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.14...v0.0.15">compare
  view</a></li>
  </ul>
  </details>
  <br />

  You can trigger a rebase of this PR by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 9 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#143](https://github.com/substrait-io/substrait-rs/issues/143)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#143](https://github.com/substrait-io/substrait-rs/issues/143)**
  - Update typify requirement from 0.0.14 to 0.0.15 ([`8d9886a`](https://github.com/substrait-io/substrait-rs/commit/8d9886a420488bf57ae81015571347e043b56e8f))
- **Uncategorized** - Release substrait v0.20.3 ([`8227aae`](https://github.com/substrait-io/substrait-rs/commit/8227aaef8d5cf0a730f282df006c12ed41469e90))
</details>

## 0.20.2 (2023-12-05)

<csr-id-d8912a529e86b82e200d9bbbb63dafa1d6c06a76/>

### Chore

- <csr-id-d8912a529e86b82e200d9bbbb63dafa1d6c06a76/> bump actions/configure-pages from 3 to 4
  Bumps
  [actions/configure-pages](https://github.com/actions/configure-pages)
  from 3 to 4.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/actions/configure-pages/releases">actions/configure-pages's
  releases</a>.</em></p>
  <blockquote>
  <h2>v4.0.0</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Use a centralized <code>.node-version</code> file <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/configure-pages/issues/117">#117</a>)</li>
  <li>Update action to node20 <a
              href="https://github.com/takost"><code>@​takost</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/108">#108</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/configure-pages/compare/v3.0.7...v4.0.0">all
  code changes</a> since previous release.</p>
  <h2>v3.0.7</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Update Actions workflows to use Node 20.x <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/configure-pages/issues/116">#116</a>)</li>
  <li>Bump eslint-plugin-github from 4.7.0 to 4.10.1 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/114">#114</a>)</li>
  <li>Bump word-wrap from 1.2.3 to 1.2.5 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/113">#113</a>)</li>
  <li>Bump jest from 29.5.0 to 29.7.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/112">#112</a>)</li>
  <li>Bump <code>@​babel/traverse</code> from 7.21.3 to 7.23.5 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/110">#110</a>)</li>
  <li>Bump espree from 9.5.2 to 9.6.1 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/111">#111</a>)</li>
  <li>Bump eslint from 8.38.0 to 8.40.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/93">#93</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/configure-pages/compare/v3.0.6...v3.0.7">all
  code changes</a> since previous release.</p>
  <h2>v3.0.6</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Bump eslint from 8.36.0 to 8.38.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/89">#89</a>)</li>
  <li>Bump eslint-plugin-github from 4.6.1 to 4.7.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/86">#86</a>)</li>
  <li>Bump eslint-config-prettier from 8.7.0 to 8.8.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/85">#85</a>)</li>
  <li>Bump prettier from 2.8.6 to 2.8.7 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/84">#84</a>)</li>
  <li>Make &quot;Get Pages failed&quot; error message more helpful <a
              href="https://github.com/WofWca"><code>@​WofWca</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/83">#83</a>)</li>
  <li>Bump prettier from 2.8.4 to 2.8.6 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/81">#81</a>)</li>
  <li>Add a CodeQL security scanning workflow <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/configure-pages/issues/68">#68</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/configure-pages/compare/v3.0.5...v3.0.6">all
  code changes</a> since previous release.</p>
  <h2>v3.0.5</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Bump jest from 29.4.3 to 29.5.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/77">#77</a>)</li>
  <li>Bump eslint from 8.35.0 to 8.36.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/76">#76</a>)</li>
  <li>Bump espree from 9.4.1 to 9.5.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/78">#78</a>)</li>
  <li>Bump eslint-config-prettier from 8.6.0 to 8.7.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/75">#75</a>)</li>
  <li>Bump eslint from 8.34.0 to 8.35.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/73">#73</a>)</li>
  <li>Bump jest from 29.4.1 to 29.4.3 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/72">#72</a>)</li>
  <li>Bump eslint-plugin-github from 4.6.0 to 4.6.1 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/configure-pages/issues/71">#71</a>)</li>
  <li>Amend token coverage for enablement in Action metadata file <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/configure-pages/issues/51">#51</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/configure-pages/compare/v3.0.4...v3.0.5">all
  code changes</a> since previous release.</p>
  <h2>v3.0.4</h2>
  <h1>Changelog</h1>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/actions/configure-pages/commit/1f0c5cde4bc74cd7e1254d0cb4de8d49e9068c7d"><code>1f0c5cd</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/configure-pages/issues/117">#117</a>
  from actions/use-node-version-file</li>
  <li><a
              href="https://github.com/actions/configure-pages/commit/591bb0deb9c3f137206bbbabf9b07b6161afc6ba"><code>591bb0d</code></a>
  Merge branch 'main' into use-node-version-file</li>
  <li><a
              href="https://github.com/actions/configure-pages/commit/1465f012e6e7718cdf7bc61874e613e1e67ed84f"><code>1465f01</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/configure-pages/issues/108">#108</a>
  from takost/update-to-node-20</li>
  <li><a
              href="https://github.com/actions/configure-pages/commit/f2fc55313325af8b5ee6da799ed072a114df7416"><code>f2fc553</code></a>
  Merge branch 'main' into update-to-node-20</li>
  <li><a
              href="https://github.com/actions/configure-pages/commit/373694e352893db5ef78de6f321e8d55c860f7b6"><code>373694e</code></a>
  Use a centralized .node-version file</li>
  <li><a
              href="https://github.com/actions/configure-pages/commit/3a014131e99a7bf2daf896787a8f92789126d038"><code>3a01413</code></a>
  Update action to node20</li>
  <li>See full diff in <a
              href="https://github.com/actions/configure-pages/compare/v3...v4">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/configure-pages&package-manager=github_actions&previous-version=3&new-version=4)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#141](https://github.com/substrait-io/substrait-rs/issues/141)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#141](https://github.com/substrait-io/substrait-rs/issues/141)**
  - Bump actions/configure-pages from 3 to 4 ([`d8912a5`](https://github.com/substrait-io/substrait-rs/commit/d8912a529e86b82e200d9bbbb63dafa1d6c06a76))
- **Uncategorized** - Release substrait v0.20.2 ([`b8f1e56`](https://github.com/substrait-io/substrait-rs/commit/b8f1e56f9feb9c2be59fea6496ce3c3fe64beb64))
</details>

## 0.20.1 (2023-12-05)

<csr-id-c2d9ea0e39cd5e32a118b650d55c019171b6900e/>

### Chore

- <csr-id-c2d9ea0e39cd5e32a118b650d55c019171b6900e/> bump actions/deploy-pages from 2 to 3
  Bumps [actions/deploy-pages](https://github.com/actions/deploy-pages)
  from 2 to 3.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/actions/deploy-pages/releases">actions/deploy-pages's
  releases</a>.</em></p>
  <blockquote>
  <h2>v3.0.0</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Update action to node20 <a
              href="https://github.com/takost"><code>@​takost</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/256">#256</a>)</li>
  </ul>
  <hr />
  <p>See details of <a
              href="https://github.com/actions/deploy-pages/compare/v2.0.5...v3.0.0">all
  code changes</a> since previous release.</p>
  <p>:warning: For use with products other than GitHub.com, such as GitHub
  Enterprise Server, please consult the <a
              href="https://github.com/actions/deploy-pages/#compatibilty">compatibility
  table</a>.</p>
  <h2>v2.0.5</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Bump prettier from 3.0.0 to 3.1.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/264">#264</a>)</li>
  <li>Bump <code>@​octokit/request-error</code> from 5.0.0 to 5.0.1 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/263">#263</a>)</li>
  <li>Bump <code>@​actions/http-client</code> from 2.1.0 to 2.2.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/262">#262</a>)</li>
  <li>Bump <code>@​vercel/ncc</code> from 0.36.1 to 0.38.1 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/260">#260</a>)</li>
  <li>Bump eslint from 8.44.0 to 8.54.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/257">#257</a>)</li>
  <li>Bump nock from 13.3.1 to 13.3.8 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/254">#254</a>)</li>
  <li>Bump actions/setup-node from 3 to 4 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/246">#246</a>)</li>
  <li>Bump release-drafter/release-drafter from 5.24.0 to 5.25.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/245">#245</a>)</li>
  <li>Bump eslint-plugin-github from 4.8.0 to 4.10.1 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/232">#232</a>)</li>
  <li>Bump jest from 29.6.1 to 29.7.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/224">#224</a>)</li>
  <li>Fix cosmetic error by replacing comma with period <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/210">#210</a>)</li>
  <li>Bump actions/checkout from 3 to 4 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/220">#220</a>)</li>
  <li>Bump eslint-config-prettier from 8.8.0 to 9.0.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/209">#209</a>)</li>
  <li>Remove &quot;beta&quot; disclaimer from README <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/243">#243</a>)</li>
  </ul>
  <hr />
  <p>See details of <a
              href="https://github.com/actions/deploy-pages/compare/v2.0.4...v2.0.5">all
  code changes</a> since previous release.</p>
  <p>:warning: For use with products other than GitHub.com, such as GitHub
  Enterprise Server, please consult the <a
              href="https://github.com/actions/deploy-pages/#compatibilty">compatibility
  table</a>.</p>
  <h2>v2.0.4</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Update GHES compatibility table after verifying with
  <code>3.9.x</code> <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/201">#201</a>)</li>
  <li>Bump <code>@​octokit/request-error</code> from 4.0.1 to 5.0.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/194">#194</a>)</li>
  <li>Bump prettier from 2.8.8 to 3.0.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/196">#196</a>)</li>
  <li>Bump jest from 29.5.0 to 29.6.1 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/195">#195</a>)</li>
  <li>Bump release-drafter/release-drafter from 5.23.0 to 5.24.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/192">#192</a>)</li>
  <li>Bump eslint from 8.42.0 to 8.44.0 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/191">#191</a>)</li>
  <li>Remove circular JSON references for error debugging <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/deploy-pages/issues/197">#197</a>)</li>
  </ul>
  <hr />
  <p>See details of <a
              href="https://github.com/actions/deploy-pages/compare/v2.0.3...v2.0.4">all
  code changes</a> since previous release.</p>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/actions/deploy-pages/commit/77d7344265e1f960dab5c00dbff52287a70b0d4f"><code>77d7344</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/deploy-pages/issues/256">#256</a>
  from takost/update-to-node-20</li>
  <li><a
              href="https://github.com/actions/deploy-pages/commit/84abb89dc54f5055b934ca7dec97480f1a16f6bb"><code>84abb89</code></a>
  Merge branch 'main' into update-to-node-20</li>
  <li>See full diff in <a
              href="https://github.com/actions/deploy-pages/compare/v2...v3">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/deploy-pages&package-manager=github_actions&previous-version=2&new-version=3)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 7 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#140](https://github.com/substrait-io/substrait-rs/issues/140)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#140](https://github.com/substrait-io/substrait-rs/issues/140)**
  - Bump actions/deploy-pages from 2 to 3 ([`c2d9ea0`](https://github.com/substrait-io/substrait-rs/commit/c2d9ea0e39cd5e32a118b650d55c019171b6900e))
- **Uncategorized** - Release substrait v0.20.1 ([`02a5aae`](https://github.com/substrait-io/substrait-rs/commit/02a5aae4cd2a0199676e1c48b6f5448f77ac1b4c))
</details>

## 0.20.0 (2023-11-27)

<csr-id-6c5055baa8b6522a419f02ac550a07f9d9d0fedc/>

### Chore (BREAKING)

- <csr-id-6c5055baa8b6522a419f02ac550a07f9d9d0fedc/> bump substrait from `0.38.0` to `0.39.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `bdff923` to `8f8d85e`.

  ***

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 20 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#139](https://github.com/substrait-io/substrait-rs/issues/139)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#139](https://github.com/substrait-io/substrait-rs/issues/139)**
  - Bump substrait from `0.38.0` to `0.39.0` ([`6c5055b`](https://github.com/substrait-io/substrait-rs/commit/6c5055baa8b6522a419f02ac550a07f9d9d0fedc))
- **Uncategorized** - Release substrait v0.20.0 ([`2f0f42f`](https://github.com/substrait-io/substrait-rs/commit/2f0f42f975364a82e49074fa04a10f3f35386853))
</details>

## 0.19.0 (2023-11-06)

<csr-id-7f91d6243d399014bafbe48ef75ff361bfaab386/>

### Chore (BREAKING)

- <csr-id-7f91d6243d399014bafbe48ef75ff361bfaab386/> bump substrait from `0.37.0` to `0.38.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `52e81a9` to `bdff923`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/bdff9232d858b0b73526bcc115921d39eaaf492d"><code>bdff923</code></a>
  chore(release): 0.38.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/b3071bc9cd77cf916568641c83056a285f8123be"><code>b3071bc</code></a>
  feat: add least and greatest functions to functions_comparison.yml (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/247">#247</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/6c88a1e1a6d8a8a05a3b5b8a141309009d62be29"><code>6c88a1e</code></a>
  chore(deps): Bump bufbuild/buf-setup-action from 1.27.1 to 1.27.2 (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/569">#569</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/2d9f1b95a4bef196aa0e27a8e9425f12ee8f2998"><code>2d9f1b9</code></a>
  chore(deps): Bump actions/setup-node from 3 to 4 (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/568">#568</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/52e81a9fe725881036eddaa77ae0dba8b2ad6f83...bdff9232d858b0b73526bcc115921d39eaaf492d">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 13 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#137](https://github.com/substrait-io/substrait-rs/issues/137)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#137](https://github.com/substrait-io/substrait-rs/issues/137)**
  - Bump substrait from `0.37.0` to `0.38.0` ([`7f91d62`](https://github.com/substrait-io/substrait-rs/commit/7f91d6243d399014bafbe48ef75ff361bfaab386))
- **Uncategorized** - Release substrait v0.19.0 ([`1d8e21f`](https://github.com/substrait-io/substrait-rs/commit/1d8e21f653a92531272ffdf346c9a6bd6aef452f))
</details>

## 0.18.1 (2023-10-24)

<csr-id-47d00137150f3d15b813d0c366d518efd389d792/>

### Chore

- <csr-id-47d00137150f3d15b813d0c366d518efd389d792/> bump actions/setup-node from 3 to 4
  Bumps [actions/setup-node](https://github.com/actions/setup-node) from 3
  to 4.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/actions/setup-node/releases">actions/setup-node's
  releases</a>.</em></p>
  <blockquote>
  <h2>v4.0.0</h2>
  <h2>What's Changed</h2>
  <p>In scope of this release we changed version of node runtime for
  action from node16 to node20 and updated dependencies in <a
              href="https://redirect.github.com/actions/setup-node/pull/866">actions/setup-node#866</a></p>
  <p>Besides, release contains such changes as:</p>
  <ul>
  <li>Upgrade actions/checkout to v4 by <a
              href="https://github.com/gmembre-zenika"><code>@​gmembre-zenika</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/868">actions/setup-node#868</a></li>
  <li>Update actions/checkout for documentation and yaml by <a
              href="https://github.com/dmitry-shibanov"><code>@​dmitry-shibanov</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/876">actions/setup-node#876</a></li>
  </ul>
  <h2>New Contributors</h2>
  <ul>
  <li><a
              href="https://github.com/gmembre-zenika"><code>@​gmembre-zenika</code></a>
  made their first contribution in <a
              href="https://redirect.github.com/actions/setup-node/pull/868">actions/setup-node#868</a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/actions/setup-node/compare/v3...v4.0.0">https://github.com/actions/setup-node/compare/v3...v4.0.0</a></p>
  <h2>v3.8.2</h2>
  <h2>What's Changed</h2>
  <ul>
  <li>Update semver by <a
              href="https://github.com/dmitry-shibanov"><code>@​dmitry-shibanov</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/861">actions/setup-node#861</a></li>
  <li>Update temp directory creation by <a
              href="https://github.com/nikolai-laevskii"><code>@​nikolai-laevskii</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/859">actions/setup-node#859</a></li>
  <li>Bump <code>@​babel/traverse</code> from 7.15.4 to 7.23.2 by <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
              href="https://redirect.github.com/actions/setup-node/pull/870">actions/setup-node#870</a></li>
  <li>Add notice about binaries not being updated yet by <a
              href="https://github.com/nikolai-laevskii"><code>@​nikolai-laevskii</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/872">actions/setup-node#872</a></li>
  <li>Update toolkit cache and core by <a
              href="https://github.com/dmitry-shibanov"><code>@​dmitry-shibanov</code></a>
  and <a
              href="https://github.com/seongwon-privatenote"><code>@​seongwon-privatenote</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/875">actions/setup-node#875</a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/actions/setup-node/compare/v3...v3.8.2">https://github.com/actions/setup-node/compare/v3...v3.8.2</a></p>
  <h2>v3.8.1</h2>
  <h2>What's Changed</h2>
  <p>In scope of this release, the filter was removed within the
  cache-save step by <a
              href="https://github.com/dmitry-shibanov"><code>@​dmitry-shibanov</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/831">actions/setup-node#831</a>.
  It is filtered and checked in the toolkit/cache library.</p>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/actions/setup-node/compare/v3...v3.8.1">https://github.com/actions/setup-node/compare/v3...v3.8.1</a></p>
  <h2>v3.8.0</h2>
  <h2>What's Changed</h2>
  <h3>Bug fixes:</h3>
  <ul>
  <li>Add check for existing paths by <a
              href="https://github.com/dmitry-shibanov"><code>@​dmitry-shibanov</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/803">actions/setup-node#803</a></li>
  <li>Resolve SymbolicLink by <a
              href="https://github.com/dmitry-shibanov"><code>@​dmitry-shibanov</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/809">actions/setup-node#809</a></li>
  <li>Change passing logic for cache input by <a
              href="https://github.com/dmitry-shibanov"><code>@​dmitry-shibanov</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/816">actions/setup-node#816</a></li>
  <li>Fix armv7 cache issue by <a
              href="https://github.com/louislam"><code>@​louislam</code></a> in <a
              href="https://redirect.github.com/actions/setup-node/pull/794">actions/setup-node#794</a></li>
  <li>Update check-dist workflow name by <a
              href="https://github.com/sinchang"><code>@​sinchang</code></a> in <a
              href="https://redirect.github.com/actions/setup-node/pull/710">actions/setup-node#710</a></li>
  </ul>
  <h3>Feature implementations:</h3>
  <ul>
  <li>feat: handling the case where &quot;node&quot; is used for
  tool-versions file. by <a
              href="https://github.com/xytis"><code>@​xytis</code></a> in <a
              href="https://redirect.github.com/actions/setup-node/pull/812">actions/setup-node#812</a></li>
  </ul>
  <h3>Documentation changes:</h3>
  <ul>
  <li>Refer to semver package name in README.md by <a
              href="https://github.com/olleolleolle"><code>@​olleolleolle</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/808">actions/setup-node#808</a></li>
  </ul>
  <h3>Update dependencies:</h3>
  <ul>
  <li>Update toolkit cache to fix zstd by <a
              href="https://github.com/dmitry-shibanov"><code>@​dmitry-shibanov</code></a>
  in <a
              href="https://redirect.github.com/actions/setup-node/pull/804">actions/setup-node#804</a></li>
  <li>Bump tough-cookie and <code>@​azure/ms-rest-js</code> by <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
              href="https://redirect.github.com/actions/setup-node/pull/802">actions/setup-node#802</a></li>
  <li>Bump semver from 6.1.2 to 6.3.1 by <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> in <a
              href="https://redirect.github.com/actions/setup-node/pull/807">actions/setup-node#807</a></li>
  </ul>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/actions/setup-node/commit/8f152de45cc393bb48ce5d89d36b731f54556e65"><code>8f152de</code></a>
  Update actions/checkout for documentation and yaml (<a
              href="https://redirect.github.com/actions/setup-node/issues/876">#876</a>)</li>
  <li><a
              href="https://github.com/actions/setup-node/commit/23755b521f87533c8ed7f8fb13674f9021579e34"><code>23755b5</code></a>
  upgrade actions/checkout to v4 (<a
              href="https://redirect.github.com/actions/setup-node/issues/868">#868</a>)</li>
  <li><a
              href="https://github.com/actions/setup-node/commit/54534a2a9ba7308e8a8995af3104899e6a95b681"><code>54534a2</code></a>
  Change node version for action to node20 (<a
              href="https://redirect.github.com/actions/setup-node/issues/866">#866</a>)</li>
  <li>See full diff in <a
              href="https://github.com/actions/setup-node/compare/v3...v4">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/setup-node&package-manager=github_actions&previous-version=3&new-version=4)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 1 day passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#135](https://github.com/substrait-io/substrait-rs/issues/135)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#135](https://github.com/substrait-io/substrait-rs/issues/135)**
  - Bump actions/setup-node from 3 to 4 ([`47d0013`](https://github.com/substrait-io/substrait-rs/commit/47d00137150f3d15b813d0c366d518efd389d792))
- **Uncategorized** - Release substrait v0.18.1 ([`15f467a`](https://github.com/substrait-io/substrait-rs/commit/15f467a7b276acd76247317b47894737722b999d))
</details>

## 0.18.0 (2023-10-23)

<csr-id-c5734ecd9fe997b8ad68e32c1fa6315445b80637/>
<csr-id-e340c3b4c13988abae264c181f697d2b8bf767f5/>

### Chore

- <csr-id-c5734ecd9fe997b8ad68e32c1fa6315445b80637/> update typify requirement from 0.0.13 to 0.0.14
  Updates the requirements on
  [typify](https://github.com/oxidecomputer/typify) to permit the latest
  version.
  <details>
  <summary>Changelog</summary>
  <p><em>Sourced from <a
  href="https://github.com/oxidecomputer/typify/blob/main/CHANGELOG.adoc">typify's
  changelog</a>.</em></p>
  <blockquote>
  <p>== 0.0.14 (released 2023-09-25)</p>
  <ul>
  <li>Handle arbitrary containment cycles (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/300">#300</a>)</li>
  <li>More permissive of valid (if useless) schema constructions (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/306">#306</a>,
  <a
  href="https://redirect.github.com/oxidecomputer/typify/issues/320">#320</a>)</li>
  <li>Much better handling of <code>allOf</code> constructions by merging
  schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/405">#405</a>)</li>
  <li>Support for more <code>not</code> subschemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/410">#410</a>)</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.13%5C...v0.0.14%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.13\...v0.0.14[Full</a>
  list of commits]</p>
  <p>== 0.0.13 (released 2023-05-14)</p>
  <ul>
  <li>Fixed-length, single-type arrays to <code>[T; N]</code> (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/286">#286</a>)</li>
  <li>Support for reflexive schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/292">#292</a>)</li>
  <li>Much improved support for multi-type schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/291">#291</a>)</li>
  <li>Better error messages on failures</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.12%5C...v0.0.13%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.12\...v0.0.13[Full</a>
  list of commits]</p>
  <p>== 0.0.12 (released 2023-05-03)</p>
  <ul>
  <li>Improved enum generation (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/270">#270</a>)</li>
  <li>Improved integer type selection based on number criteria (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/255">#255</a>)</li>
  <li><code>TypeSpace::add_root_schema()</code> (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/236">#236</a>)</li>
  <li>... and many general improvements</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.11%5C...v0.0.12%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.11\...v0.0.12[Full</a>
  list of commits]</p>
  <p>== 0.0.11 (released 2023-03-18)</p>
  <p>This is a big update with many, many changes to code generation, and
  many more
  JSON schema structures well-handled. Among the many changes:</p>
  <ul>
  <li>Generate a <code>ToString</code> impl for untagged enums with
  trivial variants (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/145">#145</a>)</li>
  <li>Allow conversion overrides by specifying a schema (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/155">#155</a>)</li>
  <li>Handle untyped enums that contain nulls (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/167">#167</a>)</li>
  <li>Handle <code>not</code> schemas for enumerated values (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/168">#168</a>)</li>
  <li>Improve generated code for FromStr and TryFrom impls (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/174">#174</a>)</li>
  <li>Handle format specifiers for enumerated strings (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/188">#188</a>)</li>
  </ul>
  <p>=== <em>Breaking</em>: The removal of
  <code>TypeSpace::to_string()</code></p>
  <p>Previously all transitive consumers required the presence of
  <code>rustfmt</code>. In this
  version we leave formatting to the consumer. See
  link:README.md#formatting[the formatting section of the README] for
  details on formatting.</p>
  <p>=== CLI</p>
  <p>This version adds the <code>cargo-typify</code> crate for stand-alone
  code generation.</p>
  <p>=== Augmented Generation</p>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/85181edf85ca2c7513a7a39d50479017bc737b58"><code>85181ed</code></a>
  release typify 0.0.14</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/277ed501b8e5d76ed5edb3d89e794a0a4d9be189"><code>277ed50</code></a>
  prep for 0.0.14</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/6b2a3601a59cdbc6386922b9fdb241fd7d754d1d"><code>6b2a360</code></a>
  fix stack overflow; improve <code>not</code> subschema handling (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/410">#410</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/6048f17e9bf052d626f360d45eb80751a28bf3fc"><code>6048f17</code></a>
  Bump syn from 2.0.32 to 2.0.37 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/406">#406</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/3131d324f258f5965a61315527a954185295f7ab"><code>3131d32</code></a>
  Bump clap from 4.4.2 to 4.4.4 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/407">#407</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/d28da5cb99bb535ff4d13e9ed1c8209e3a8ffea4"><code>d28da5c</code></a>
  Bump chrono from 0.4.30 to 0.4.31 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/409">#409</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/ffd99b70a40764c5032d114fa90179dbe47af930"><code>ffd99b7</code></a>
  handle <code>allOf</code> by merging schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/405">#405</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/6370ce4a2de669f1040e46bf94544c226ac6cb5c"><code>6370ce4</code></a>
  Bump serde_json from 1.0.106 to 1.0.107 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/399">#399</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/de16c4238a2b34400d0fece086a6469951c3236b"><code>de16c42</code></a>
  Bump trybuild from 1.0.84 to 1.0.85 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/401">#401</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/ce4282e365800845186349630f4b7a43555968ce"><code>ce4282e</code></a>
  Bump prettyplease from 0.2.14 to 0.2.15 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/402">#402</a>)</li>
  <li>Additional commits viewable in <a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.13...v0.0.14">compare
  view</a></li>
  </ul>
  </details>
  <br />

  You can trigger a rebase of this PR by commenting `@dependabot rebase`.

### Chore (BREAKING)

- <csr-id-e340c3b4c13988abae264c181f697d2b8bf767f5/> bump substrait from `0.36.0` to `0.37.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `92302d8` to `52e81a9`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/52e81a9fe725881036eddaa77ae0dba8b2ad6f83"><code>52e81a9</code></a>
  chore(release): 0.37.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/cf327502bdb187ae06d9210e9de460193027679e"><code>cf32750</code></a>
  feat: add NestedLoopJoinRel definition (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/561">#561</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/12f899b29f1cc4c48fa3f61a8e07d1af2515baf7"><code>12f899b</code></a>
  docs: manually configure committers and smc members (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/566">#566</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/c776fda1fb8fa349f22022d1879b5acfb5186c0d"><code>c776fda</code></a>
  chore(deps): Bump bufbuild/buf-setup-action from 1.27.0 to 1.27.1 (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/564">#564</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/4f1868608b126d86e0a40ea161d9e7a30910c0af"><code>4f18686</code></a>
  chore: add vbarua to CODEOWNERS (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/567">#567</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/f0eebab4ab8db68793975f1e35445d543a78b617"><code>f0eebab</code></a>
  chore: add EpsilonPrime to CODEOWNERS (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/565">#565</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/6887fd51152a4c41e58fb24f20d5984a8eb86989"><code>6887fd5</code></a>
  docs: add code of conduct (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/339">#339</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/ddf4d89e0b9d622eb82cdf0e7f00fd68aa0613fb"><code>ddf4d89</code></a>
  chore(deps): Bump bufbuild/buf-setup-action from 1.26.1 to 1.27.0 (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/562">#562</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/92302d840a751e72b760eef5fc8cbb0f16bfef38...52e81a9fe725881036eddaa77ae0dba8b2ad6f83">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 3 commits contributed to the release.
- 11 days passed between releases.
- 2 commits were understood as [conventional](https://www.conventionalcommits.org).
- 2 unique issues were worked on: [#132](https://github.com/substrait-io/substrait-rs/issues/132), [#134](https://github.com/substrait-io/substrait-rs/issues/134)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#132](https://github.com/substrait-io/substrait-rs/issues/132)**
  - Update typify requirement from 0.0.13 to 0.0.14 ([`c5734ec`](https://github.com/substrait-io/substrait-rs/commit/c5734ecd9fe997b8ad68e32c1fa6315445b80637))
- **[#134](https://github.com/substrait-io/substrait-rs/issues/134)**
  - Bump substrait from `0.36.0` to `0.37.0` ([`e340c3b`](https://github.com/substrait-io/substrait-rs/commit/e340c3b4c13988abae264c181f697d2b8bf767f5))
- **Uncategorized** - Release substrait v0.18.0 ([`86a3b70`](https://github.com/substrait-io/substrait-rs/commit/86a3b708e2b56b142314e52ae0ec49db99492524))
</details>

## 0.17.0 (2023-10-11)

<csr-id-bd038395adf909bacc590c0c0c651bc0d6f13902/>

### Chore (BREAKING)

- <csr-id-bd038395adf909bacc590c0c0c651bc0d6f13902/> update prost and pbjson crates
  Updates the requirements on [prost](https://github.com/tokio-rs/prost)
  to permit the latest version.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/10582c2f674a1f4618e9fb8d380081136340ac84"><code>10582c2</code></a>
  release 0.12.0</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/7ce9b9756ba1ca0c6228931e2a9ff7859931ac95"><code>7ce9b97</code></a>
  feat: <code>Name</code> trait + <code>Any</code> encoding support (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/896">#896</a>)</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/f9a3cfff07536a412e4f96f5ea3749c52c50170f"><code>f9a3cff</code></a>
  Make Debug impl optional for types (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/797">#797</a>)</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/6180f9fa3a93e42572633cb0b0610dcb0690c508"><code>6180f9f</code></a>
  docs: fix protoc install link (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/900">#900</a>)</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/1d7405803004e85f58511af2a439460b4237a0e2"><code>1d74058</code></a>
  prost-build: do not escape brackets followed by parenthesis in comments
  (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/851">#851</a>)</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/413b0299e0d0e73b6085ad9b897fb7ec7e63cb2a"><code>413b029</code></a>
  chore: Bump MSRV to 1.64 (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/902">#902</a>)</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/ca73cbe4bbcbdb4f3d30455a090ff50209dc1fc3"><code>ca73cbe</code></a>
  feat: add TryFrom&lt;i32&gt; implementation to Enumeration (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/853">#853</a>)</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/9c877ce32ec0c465e437144c1bf4a27cb3aa705c"><code>9c877ce</code></a>
  chore: Update ci (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/854">#854</a>)</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/65451cc4cbffaca578de1a8f4a218cca2ec1c9ac"><code>65451cc</code></a>
  Fix <code>non_snake_case</code> warning on generated identity functions
  (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/891">#891</a>)</li>
  <li><a
  href="https://github.com/tokio-rs/prost/commit/80fb9d18b7af2c82a8f54fb6c9e7ccfa4d5629ce"><code>80fb9d1</code></a>
  release 0.11.9 (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/842">#842</a>)
  (<a
  href="https://redirect.github.com/tokio-rs/prost/issues/895">#895</a>)</li>
  <li>Additional commits viewable in <a
  href="https://github.com/tokio-rs/prost/compare/prost-build-0.11.1...v0.12.0">compare
  view</a></li>
  </ul>
  </details>
  <br />

  You can trigger a rebase of this PR by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 2 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#121](https://github.com/substrait-io/substrait-rs/issues/121)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#121](https://github.com/substrait-io/substrait-rs/issues/121)**
  - Update prost and pbjson crates ([`bd03839`](https://github.com/substrait-io/substrait-rs/commit/bd038395adf909bacc590c0c0c651bc0d6f13902))
- **Uncategorized** - Release substrait v0.17.0 ([`e3a95d3`](https://github.com/substrait-io/substrait-rs/commit/e3a95d392182f8005bd2098b330b5bef6d4901fc))
</details>

## 0.16.0 (2023-10-09)

<csr-id-f2ee3e734d9849e714051b321de5ea618919a7a6/>

### Chore (BREAKING)

- <csr-id-f2ee3e734d9849e714051b321de5ea618919a7a6/> bump substrait from `0.35.0` to `0.36.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `9e39067` to `92302d8`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/92302d840a751e72b760eef5fc8cbb0f16bfef38"><code>92302d8</code></a>
  chore(release): 0.36.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/8406cf6753b97829b2b5211344822d6f2f840eab"><code>8406cf6</code></a>
  feat: geometry processing functions (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/556">#556</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/9e39067e49f453f1df998c3c4a821e53dd7a1e8f...92302d840a751e72b760eef5fc8cbb0f16bfef38">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 7 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#131](https://github.com/substrait-io/substrait-rs/issues/131)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#131](https://github.com/substrait-io/substrait-rs/issues/131)**
  - Bump substrait from `0.35.0` to `0.36.0` ([`f2ee3e7`](https://github.com/substrait-io/substrait-rs/commit/f2ee3e734d9849e714051b321de5ea618919a7a6))
- **Uncategorized** - Release substrait v0.16.0 ([`99760d8`](https://github.com/substrait-io/substrait-rs/commit/99760d885e4fad739efa1fab1a47762aa5f3aa29))
</details>

## 0.15.0 (2023-10-02)

<csr-id-e38a108eee497b1ce61e5d3bcd962a203725409f/>

### Chore (BREAKING)

- <csr-id-e38a108eee497b1ce61e5d3bcd962a203725409f/> bump substrait from `0.34.0` to `0.35.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `dacc5a9` to `9e39067`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/9e39067e49f453f1df998c3c4a821e53dd7a1e8f"><code>9e39067</code></a>
  chore(release): 0.35.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/727467cc66f4c4984c7a8ea1205a473644f00b23"><code>727467c</code></a>
  feat: add geometry editor functions (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/554">#554</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/784fa9b1702a1df64a8286a25fce377a0aa29fd4"><code>784fa9b</code></a>
  feat: adding geometry accessor functions (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/552">#552</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/3adc04861bcd117bc51419702c9ea283645acdae"><code>3adc048</code></a>
  docs: eliminate early design phase language (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/547">#547</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/20684bba4bb92ceeaa2f2ec185ea081d266fa7ee"><code>20684bb</code></a>
  docs: focus homepage on discussing what Substrait is (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/538">#538</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/02d07fd4e41c54a1915c8fb73a439294efd5208c"><code>02d07fd</code></a>
  docs: add post_join_filter description (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/559">#559</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/30773b2fcb67413625535cd1ada144dccfdcde22"><code>30773b2</code></a>
  fix!: specify nullability for is_not_distinct_from (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/555">#555</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/db52bbd844f7d8db328f1b6f00758f07009ca95b"><code>db52bbd</code></a>
  feat: add geometric data types and functions (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/543">#543</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/c28f056e98b285e2a697e99424314fd2a48c7569"><code>c28f056</code></a>
  docs: add the meeting calendar to the website (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/557">#557</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/da4b32ac41827ae8b53a2833ec34872670904e57"><code>da4b32a</code></a>
  feat: add approval guidelines for documentation updates (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/553">#553</a>)</li>
  <li>Additional commits viewable in <a
  href="https://github.com/substrait-io/substrait/compare/dacc5a96a17fe45aed228f7819de3aa8a404a8b0...9e39067e49f453f1df998c3c4a821e53dd7a1e8f">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 13 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#130](https://github.com/substrait-io/substrait-rs/issues/130)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#130](https://github.com/substrait-io/substrait-rs/issues/130)**
  - Bump substrait from `0.34.0` to `0.35.0` ([`e38a108`](https://github.com/substrait-io/substrait-rs/commit/e38a108eee497b1ce61e5d3bcd962a203725409f))
- **Uncategorized** - Release substrait v0.15.0 ([`4b2e4c5`](https://github.com/substrait-io/substrait-rs/commit/4b2e4c51a79ad4919f02ec97363df4ce543afe91))
</details>

## 0.14.0 (2023-09-18)

<csr-id-c647cae108696d617a8bb65e306b28186d5bf2aa/>

### Chore (BREAKING)

- <csr-id-c647cae108696d617a8bb65e306b28186d5bf2aa/> bump substrait from `0.33.0` to `0.34.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `51765cc` to `dacc5a9`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 12 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#126](https://github.com/substrait-io/substrait-rs/issues/126)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#126](https://github.com/substrait-io/substrait-rs/issues/126)**
  - Bump substrait from `0.33.0` to `0.34.0` ([`c647cae`](https://github.com/substrait-io/substrait-rs/commit/c647cae108696d617a8bb65e306b28186d5bf2aa))
- **Uncategorized** - Release substrait v0.14.0 ([`6faf5d3`](https://github.com/substrait-io/substrait-rs/commit/6faf5d3d24c36d646d314edda3d655942e5aebeb))
</details>

## 0.13.2 (2023-09-05)

<csr-id-96dbe1791a6411a35db1cc1fcf7d05800c6a7e0f/>

### Chore

- <csr-id-96dbe1791a6411a35db1cc1fcf7d05800c6a7e0f/> bump actions/checkout from 3 to 4
  Bumps [actions/checkout](https://github.com/actions/checkout) from 3 to 4.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/actions/checkout/releases">actions/checkout's
  releases</a>.</em></p>
  <blockquote>
  <h2>v4.0.0</h2>
  <h2>What's Changed</h2>
  <ul>
  <li>Update default runtime to node20 by <a
              href="https://github.com/takost"><code>@​takost</code></a> in <a
              href="https://redirect.github.com/actions/checkout/pull/1436">actions/checkout#1436</a></li>
  <li>Support fetching without the --progress option by <a
              href="https://github.com/simonbaird"><code>@​simonbaird</code></a> in <a
              href="https://redirect.github.com/actions/checkout/pull/1067">actions/checkout#1067</a></li>
  <li>Release 4.0.0 by <a
              href="https://github.com/takost"><code>@​takost</code></a> in <a
              href="https://redirect.github.com/actions/checkout/pull/1447">actions/checkout#1447</a></li>
  </ul>
  <h2>New Contributors</h2>
  <ul>
  <li><a href="https://github.com/takost"><code>@​takost</code></a> made
  their first contribution in <a
              href="https://redirect.github.com/actions/checkout/pull/1436">actions/checkout#1436</a></li>
  <li><a
              href="https://github.com/simonbaird"><code>@​simonbaird</code></a> made
  their first contribution in <a
              href="https://redirect.github.com/actions/checkout/pull/1067">actions/checkout#1067</a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/actions/checkout/compare/v3...v4.0.0">https://github.com/actions/checkout/compare/v3...v4.0.0</a></p>
  <h2>v3.6.0</h2>
  <h2>What's Changed</h2>
  <ul>
  <li>Mark test scripts with Bash'isms to be run via Bash by <a
              href="https://github.com/dscho"><code>@​dscho</code></a> in <a
              href="https://redirect.github.com/actions/checkout/pull/1377">actions/checkout#1377</a></li>
  <li>Add option to fetch tags even if fetch-depth &gt; 0 by <a
              href="https://github.com/RobertWieczoreck"><code>@​RobertWieczoreck</code></a>
  in <a
              href="https://redirect.github.com/actions/checkout/pull/579">actions/checkout#579</a></li>
  <li>Release 3.6.0 by <a
              href="https://github.com/luketomlinson"><code>@​luketomlinson</code></a>
  in <a
              href="https://redirect.github.com/actions/checkout/pull/1437">actions/checkout#1437</a></li>
  </ul>
  <h2>New Contributors</h2>
  <ul>
  <li><a
              href="https://github.com/RobertWieczoreck"><code>@​RobertWieczoreck</code></a>
  made their first contribution in <a
              href="https://redirect.github.com/actions/checkout/pull/579">actions/checkout#579</a></li>
  <li><a
              href="https://github.com/luketomlinson"><code>@​luketomlinson</code></a>
  made their first contribution in <a
              href="https://redirect.github.com/actions/checkout/pull/1437">actions/checkout#1437</a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/actions/checkout/compare/v3.5.3...v3.6.0">https://github.com/actions/checkout/compare/v3.5.3...v3.6.0</a></p>
  <h2>v3.5.3</h2>
  <h2>What's Changed</h2>
  <ul>
  <li>Fix: Checkout Issue in self hosted runner due to faulty submodule
  check-ins by <a
              href="https://github.com/megamanics"><code>@​megamanics</code></a> in <a
              href="https://redirect.github.com/actions/checkout/pull/1196">actions/checkout#1196</a></li>
  <li>Fix typos found by codespell by <a
              href="https://github.com/DimitriPapadopoulos"><code>@​DimitriPapadopoulos</code></a>
  in <a
              href="https://redirect.github.com/actions/checkout/pull/1287">actions/checkout#1287</a></li>
  <li>Add support for sparse checkouts by <a
              href="https://github.com/dscho"><code>@​dscho</code></a> and <a
              href="https://github.com/dfdez"><code>@​dfdez</code></a> in <a
              href="https://redirect.github.com/actions/checkout/pull/1369">actions/checkout#1369</a></li>
  <li>Release v3.5.3 by <a
              href="https://github.com/TingluoHuang"><code>@​TingluoHuang</code></a>
  in <a
              href="https://redirect.github.com/actions/checkout/pull/1376">actions/checkout#1376</a></li>
  </ul>
  <h2>New Contributors</h2>
  <ul>
  <li><a
              href="https://github.com/megamanics"><code>@​megamanics</code></a> made
  their first contribution in <a
              href="https://redirect.github.com/actions/checkout/pull/1196">actions/checkout#1196</a></li>
  <li><a
              href="https://github.com/DimitriPapadopoulos"><code>@​DimitriPapadopoulos</code></a>
  made their first contribution in <a
              href="https://redirect.github.com/actions/checkout/pull/1287">actions/checkout#1287</a></li>
  <li><a href="https://github.com/dfdez"><code>@​dfdez</code></a> made
  their first contribution in <a
              href="https://redirect.github.com/actions/checkout/pull/1369">actions/checkout#1369</a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/actions/checkout/compare/v3...v3.5.3">https://github.com/actions/checkout/compare/v3...v3.5.3</a></p>
  <h2>v3.5.2</h2>
  <h2>What's Changed</h2>
  <ul>
  <li>Fix: Use correct API url / endpoint in GHES by <a
              href="https://github.com/fhammerl"><code>@​fhammerl</code></a> in <a
              href="https://redirect.github.com/actions/checkout/pull/1289">actions/checkout#1289</a>
  based on <a
              href="https://redirect.github.com/actions/checkout/issues/1286">#1286</a>
  by <a href="https://github.com/1newsr"><code>@​1newsr</code></a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/actions/checkout/compare/v3.5.1...v3.5.2">https://github.com/actions/checkout/compare/v3.5.1...v3.5.2</a></p>
  <h2>v3.5.1</h2>
  <h2>What's Changed</h2>
  <ul>
  <li>Improve checkout performance on Windows runners by upgrading
  <code>@​actions/github</code> dependency by <a
              href="https://github.com/BrettDong"><code>@​BrettDong</code></a> in <a
              href="https://redirect.github.com/actions/checkout/pull/1246">actions/checkout#1246</a></li>
  </ul>
  <h2>New Contributors</h2>
  <ul>
  <li><a href="https://github.com/BrettDong"><code>@​BrettDong</code></a>
  made their first contribution in <a
              href="https://redirect.github.com/actions/checkout/pull/1246">actions/checkout#1246</a></li>
  </ul>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Changelog</summary>
  <p><em>Sourced from <a
              href="https://github.com/actions/checkout/blob/main/CHANGELOG.md">actions/checkout's
  changelog</a>.</em></p>
  <blockquote>
  <h1>Changelog</h1>
  <h2>v4.0.0</h2>
  <ul>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1067">Support
  fetching without the --progress option</a></li>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1436">Update to
  node20</a></li>
  </ul>
  <h2>v3.6.0</h2>
  <ul>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1377">Fix: Mark
  test scripts with Bash'isms to be run via Bash</a></li>
  <li><a href="https://redirect.github.com/actions/checkout/pull/579">Add
  option to fetch tags even if fetch-depth &gt; 0</a></li>
  </ul>
  <h2>v3.5.3</h2>
  <ul>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1196">Fix:
  Checkout fail in self-hosted runners when faulty submodule are
  checked-in</a></li>
  <li><a href="https://redirect.github.com/actions/checkout/pull/1287">Fix
  typos found by codespell</a></li>
  <li><a href="https://redirect.github.com/actions/checkout/pull/1369">Add
  support for sparse checkouts</a></li>
  </ul>
  <h2>v3.5.2</h2>
  <ul>
  <li><a href="https://redirect.github.com/actions/checkout/pull/1289">Fix
  api endpoint for GHES</a></li>
  </ul>
  <h2>v3.5.1</h2>
  <ul>
  <li><a href="https://redirect.github.com/actions/checkout/pull/1246">Fix
  slow checkout on Windows</a></li>
  </ul>
  <h2>v3.5.0</h2>
  <ul>
  <li><a href="https://redirect.github.com/actions/checkout/pull/1237">Add
  new public key for known_hosts</a></li>
  </ul>
  <h2>v3.4.0</h2>
  <ul>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1209">Upgrade
  codeql actions to v2</a></li>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1210">Upgrade
  dependencies</a></li>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1225">Upgrade
  <code>@​actions/io</code></a></li>
  </ul>
  <h2>v3.3.0</h2>
  <ul>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1045">Implement
  branch list using callbacks from exec function</a></li>
  <li><a href="https://redirect.github.com/actions/checkout/pull/1050">Add
  in explicit reference to private checkout options</a></li>
  <li>[Fix comment typos (that got added in <a
  href="https://redirect.github.com/actions/checkout/issues/770">#770</a>)](<a
              href="https://redirect.github.com/actions/checkout/pull/1057">actions/checkout#1057</a>)</li>
  </ul>
  <h2>v3.2.0</h2>
  <ul>
  <li><a href="https://redirect.github.com/actions/checkout/pull/942">Add
  GitHub Action to perform release</a></li>
  <li><a href="https://redirect.github.com/actions/checkout/pull/967">Fix
  status badge</a></li>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1002">Replace
  datadog/squid with ubuntu/squid Docker image</a></li>
  <li><a href="https://redirect.github.com/actions/checkout/pull/964">Wrap
  pipeline commands for submoduleForeach in quotes</a></li>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1029">Update
  <code>@​actions/io</code> to 1.1.2</a></li>
  <li><a
              href="https://redirect.github.com/actions/checkout/pull/1039">Upgrading
  version to 3.2.0</a></li>
  </ul>
  <h2>v3.1.0</h2>
  <ul>
  <li><a href="https://redirect.github.com/actions/checkout/pull/939">Use
  <code>@​actions/core</code> <code>saveState</code> and
  <code>getState</code></a></li>
  <li><a href="https://redirect.github.com/actions/checkout/pull/922">Add
  <code>github-server-url</code> input</a></li>
  </ul>
  <h2>v3.0.2</h2>
  <ul>
  <li><a href="https://redirect.github.com/actions/checkout/pull/770">Add
  input <code>set-safe-directory</code></a></li>
  </ul>
  <h2>v3.0.1</h2>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/actions/checkout/commit/3df4ab11eba7bda6032a0b82a6bb43b11571feac"><code>3df4ab1</code></a>
  Release 4.0.0 (<a
              href="https://redirect.github.com/actions/checkout/issues/1447">#1447</a>)</li>
  <li><a
              href="https://github.com/actions/checkout/commit/8b5e8b768746b50394015010d25e690bfab9dfbc"><code>8b5e8b7</code></a>
  Support fetching without the --progress option (<a
              href="https://redirect.github.com/actions/checkout/issues/1067">#1067</a>)</li>
  <li><a
              href="https://github.com/actions/checkout/commit/97a652b80035363df47baee5031ec8670b8878ac"><code>97a652b</code></a>
  Update default runtime to node20 (<a
              href="https://redirect.github.com/actions/checkout/issues/1436">#1436</a>)</li>
  <li>See full diff in <a
              href="https://github.com/actions/checkout/compare/v3...v4">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/checkout&package-manager=github_actions&previous-version=3&new-version=4)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 7 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#124](https://github.com/substrait-io/substrait-rs/issues/124)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#124](https://github.com/substrait-io/substrait-rs/issues/124)**
  - Bump actions/checkout from 3 to 4 ([`96dbe17`](https://github.com/substrait-io/substrait-rs/commit/96dbe1791a6411a35db1cc1fcf7d05800c6a7e0f))
- **Uncategorized** - Release substrait v0.13.2 ([`32575ad`](https://github.com/substrait-io/substrait-rs/commit/32575adc0400e41f8db37f4e78c40aa31ae09467))
</details>

## 0.13.1 (2023-08-29)

<csr-id-d8636e4809639c2b7b61fb0bba6d674935648e17/>

### Chore

- <csr-id-d8636e4809639c2b7b61fb0bba6d674935648e17/> update git2 requirement from 0.17.1 to 0.18.0
  Updates the requirements on [git2](https://github.com/rust-lang/git2-rs)
  to permit the latest version.
  <details>
  <summary>Changelog</summary>
  <p><em>Sourced from <a
  href="https://github.com/rust-lang/git2-rs/blob/master/CHANGELOG.md">git2's
  changelog</a>.</em></p>
  <blockquote>
  <h2>0.18.0 - 2023-08-28</h2>
  <p><a
  href="https://github.com/rust-lang/git2-rs/compare/0.17.2...git2-0.18.0">0.17.2...0.18.0</a></p>
  <h3>Added</h3>
  <ul>
  <li>Added <code>Blame::blame_buffer</code> for getting blame data for a
  file that has been modified in memory.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/981">#981</a></li>
  </ul>
  <h3>Changed</h3>
  <ul>
  <li>Updated to libgit2 <a
  href="https://github.com/libgit2/libgit2/releases/tag/v1.7.0">1.7.0</a>.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/968">#968</a></li>
  <li>Updated to libgit2 <a
  href="https://github.com/libgit2/libgit2/releases/tag/v1.7.1">1.7.1</a>.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/982">#982</a></li>
  <li>Switched from bitflags 1.x to 2.1. This brings some small changes to
  types generated by bitflags.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/973">#973</a></li>
  <li>Changed <code>Revwalk::with_hide_callback</code> to take a mutable
  reference to its callback to enforce type safety.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/970">#970</a></li>
  <li>Implemented <code>FusedIterator</code> for many iterators that can
  support it.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/955">#955</a></li>
  </ul>
  <h3>Fixed</h3>
  <ul>
  <li>Fixed builds with cargo's <code>-Zminimal-versions</code>.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/960">#960</a></li>
  </ul>
  <h2>0.17.2 - 2023-05-27</h2>
  <p><a
  href="https://github.com/rust-lang/git2-rs/compare/0.17.1...0.17.2">0.17.1...0.17.2</a></p>
  <h3>Added</h3>
  <ul>
  <li>Added support for stashing with options (which can support partial
  stashing).
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/930">#930</a></li>
  </ul>
  <h2>0.17.1 - 2023-04-13</h2>
  <p><a
  href="https://github.com/rust-lang/git2-rs/compare/0.17.0...0.17.1">0.17.0...0.17.1</a></p>
  <h3>Changed</h3>
  <ul>
  <li>Updated to libgit2 <a
  href="https://github.com/libgit2/libgit2/releases/tag/v1.6.4">1.6.4</a>.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/948">#948</a></li>
  </ul>
  <h2>0.17.0 - 2023-04-02</h2>
  <p><a
  href="https://github.com/rust-lang/git2-rs/compare/0.16.1...0.17.0">0.16.1...0.17.0</a></p>
  <h3>Added</h3>
  <ul>
  <li>Added <code>IntoIterator</code> implementation for
  <code>Statuses</code>.
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/880">#880</a></li>
  <li>Added <code>Reference::symbolic_set_target</code>
  <a
  href="https://redirect.github.com/rust-lang/git2-rs/pull/893">#893</a></li>
  </ul>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/69eea9137ae3dba7b0d72e369d0f241731493aea"><code>69eea91</code></a>
  Merge pull request <a
  href="https://redirect.github.com/rust-lang/git2-rs/issues/984">#984</a>
  from ehuss/update-changelog</li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/2aafd803fb40bb0c37b028a6b1d44d811274fc02"><code>2aafd80</code></a>
  Fix html_root_url for next release</li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/e22951ccda903e505b98fcf642d03754c1210485"><code>e22951c</code></a>
  Update changelog for next release.</li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/4570c391e1954776ac84747ddde2dcd66213d277"><code>4570c39</code></a>
  Merge pull request <a
  href="https://redirect.github.com/rust-lang/git2-rs/issues/981">#981</a>
  from paulvandermeijs/add-git_blame_buffer</li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/19b6d056bfbfb86c21b494a9974e02e7c329d760"><code>19b6d05</code></a>
  Merge pull request <a
  href="https://redirect.github.com/rust-lang/git2-rs/issues/982">#982</a>
  from ehuss/update-libgit2</li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/390c6d6cdfa3a64f21cf92bbd00f7717d1f6c460"><code>390c6d6</code></a>
  Systest no longer requires stable.</li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/e0a329eed1a59668352dfc59cb5008b2576d3eca"><code>e0a329e</code></a>
  Update to libgit2 1.7.1</li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/18f8ad16b6fccd79d80b6d7ccd93fc6ba8e1f969"><code>18f8ad1</code></a>
  Add binding for <code>git_blame_buffer</code></li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/7f2118185d6fc8a9a17048ad22f4d810679ee988"><code>7f21181</code></a>
  Merge pull request <a
  href="https://redirect.github.com/rust-lang/git2-rs/issues/955">#955</a>
  from vallentin/fused</li>
  <li><a
  href="https://github.com/rust-lang/git2-rs/commit/f0d52d3127ca1bbf6300b097f6c776c8e42e3a05"><code>f0d52d3</code></a>
  Implemented FusedIterator for various iterators</li>
  <li>Additional commits viewable in <a
  href="https://github.com/rust-lang/git2-rs/compare/0.17.1...git2-0.18.0">compare
  view</a></li>
  </ul>
  </details>
  <br />

  You can trigger a rebase of this PR by commenting `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 1 day passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#120](https://github.com/substrait-io/substrait-rs/issues/120)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#120](https://github.com/substrait-io/substrait-rs/issues/120)**
  - Update git2 requirement from 0.17.1 to 0.18.0 ([`d8636e4`](https://github.com/substrait-io/substrait-rs/commit/d8636e4809639c2b7b61fb0bba6d674935648e17))
- **Uncategorized** - Release substrait v0.13.1 ([`c07cb27`](https://github.com/substrait-io/substrait-rs/commit/c07cb278a1f494a68df86295836a241cd3c1ad13))
</details>

## 0.13.0 (2023-08-28)

<csr-id-920ba0d9ad7a385eea443b4ea4b173ed02b87303/>

### Chore (BREAKING)

- <csr-id-920ba0d9ad7a385eea443b4ea4b173ed02b87303/> bump substrait from `0.31.0` to `0.33.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `e486775` to `51765cc`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/51765cccd92fa0014e9cf077cc8cc952731f33d0"><code>51765cc</code></a>
  chore(release): 0.33.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/2da2afad579a428bb8f7460a153a1799af5c6ee3"><code>2da2afa</code></a>
  feat: add radians and degrees functions (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/544">#544</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/8969a40832c64a4a85ff14e61f5f22374506b161"><code>8969a40</code></a>
  docs: clarify compound extension signature naming for bools (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/545">#545</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/31b999060a6e014717f9ae3e6716986ad3066aaf"><code>31b9990</code></a>
  chore(release): 0.32.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/bd14e0e40782dbd0fa49de597ec30217b48961f2"><code>bd14e0e</code></a>
  feat: add windowrel support in proto (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/399">#399</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/90780416e36003b885437b4922c0aa6194ba834d"><code>9078041</code></a>
  ci: pre-commit updates (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/542">#542</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/ede90abba5d4ef02da97ba618ccbd85de963c2e1"><code>ede90ab</code></a>
  chore(deps): bump bufbuild/buf-setup-action from 1.26.0 to 1.26.1 (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/541">#541</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/65c96636bf60f166802cd60ab4a36734e0cc7f22"><code>65c9663</code></a>
  chore(deps): bump bufbuild/buf-setup-action from 1.16.0 to 1.26.0 (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/539">#539</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/98380b0dd1dd9eb30457800ec49d7912b5dce11f"><code>98380b0</code></a>
  feat: add expand rel (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/368">#368</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/2503beb3c872928483c05f76bf74d18188c84798"><code>2503beb</code></a>
  feat!: require compound functions names in extension references (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/537">#537</a>)</li>
  <li>Additional commits viewable in <a
  href="https://github.com/substrait-io/substrait/compare/e486775009c40e1a010dc54776b976b1eddea7ca...51765cccd92fa0014e9cf077cc8cc952731f33d0">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 25 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#119](https://github.com/substrait-io/substrait-rs/issues/119)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#119](https://github.com/substrait-io/substrait-rs/issues/119)**
  - Bump substrait from `0.31.0` to `0.33.0` ([`920ba0d`](https://github.com/substrait-io/substrait-rs/commit/920ba0d9ad7a385eea443b4ea4b173ed02b87303))
- **Uncategorized** - Release substrait v0.13.0 ([`38f14d4`](https://github.com/substrait-io/substrait-rs/commit/38f14d4662749e7e7fe61b4199abd0f6e4caa219))
</details>

## 0.12.4 (2023-08-02)

<csr-id-e68707c74066bd0c1069d6fc135336edc2cd26d7/>

### Chore

- <csr-id-e68707c74066bd0c1069d6fc135336edc2cd26d7/> bump arduino/setup-protoc from 1 to 2
  Bumps [arduino/setup-protoc](https://github.com/arduino/setup-protoc)
  from 1 to 2.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/arduino/setup-protoc/releases">arduino/setup-protoc's
  releases</a>.</em></p>
  <blockquote>
  <h2>v2.0.0</h2>
  <h2>Changelog</h2>
  <p>Adding support for the <code>MINOR.PATCH</code> tag naming</p>
  <h3>Breaking</h3>
  <ul>
  <li>Support only the new protobuf versioning scheme <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/78">arduino/setup-protoc#78</a></li>
  </ul>
  <p><strong>Full Changelog</strong>: <a
              href="https://github.com/arduino/setup-protoc/compare/v1.3.0...v2.0.0">https://github.com/arduino/setup-protoc/compare/v1.3.0...v2.0.0</a></p>
  <h2>Contributors</h2>
  <ul>
  <li><a href="https://github.com/ajasmin"><code>@​ajasmin</code></a></li>
  <li><a
              href="https://github.com/woodruffw"><code>@​woodruffw</code></a></li>
  <li><a href="https://github.com/zeisss"><code>@​zeisss</code></a></li>
  </ul>
  <h2>v1.3.0</h2>
  <h2>Changelog</h2>
  <h3>Enhancement</h3>
  <ul>
  <li>Support ARM64 and other platforms (<a
              href="https://redirect.github.com/arduino/setup-protoc/issues/44">#44</a>
  )</li>
  </ul>
  <h2>Full Changeset</h2>
  <p><a
              href="https://github.com/arduino/setup-protoc/compare/v1.2.0...v1.3.0">https://github.com/arduino/setup-protoc/compare/v1.2.0...v1.3.0</a></p>
  <h2>Contributors</h2>
  <ul>
  <li><a
              href="https://github.com/adamchalmers"><code>@​adamchalmers</code></a></li>
  <li><a href="https://github.com/nbaztec"><code>@​nbaztec</code></a></li>
  </ul>
  <h2>v1.2.0</h2>
  <h2>Changelog</h2>
  <h3>Enhancement</h3>
  <ul>
  <li>Add security policy link to readme (<a
              href="https://redirect.github.com/arduino/setup-protoc/issues/19">#19</a>)
  in <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/19">arduino/setup-protoc#19</a></li>
  <li>Bump Node version to 16 (<a
              href="https://redirect.github.com/arduino/setup-protoc/issues/48">#48</a>)
  <a
              href="https://redirect.github.com/arduino/setup-protoc/pull/48">arduino/setup-protoc#48</a></li>
  </ul>
  <h3>Full Changeset</h3>
  <p><a
              href="https://github.com/arduino/setup-protoc/compare/v1.1.2...v1.2.0">https://github.com/arduino/setup-protoc/compare/v1.1.2...v1.2.0</a></p>
  <h2>v1.1.2</h2>
  <ul>
  <li>Bump lodash from 4.17.15 to 4.17.19 <a
              href="https://redirect.github.com/arduino/setup-protoc/issues/8">#8</a></li>
  <li>Bump <code>@​actions/core</code> from 1.0.0 to 1.2.6 <a
              href="https://redirect.github.com/arduino/setup-protoc/issues/12">#12</a></li>
  <li>Add pagination logic when checking for versions <a
              href="https://redirect.github.com/arduino/setup-protoc/issues/1">#1</a></li>
  </ul>
  <h2>v1.1.1</h2>
  <ul>
  <li>Define <code>repo-token</code> input in metadata</li>
  </ul>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/arduino/setup-protoc/commit/9b1ee5b22b0a3f1feb8c2ff99b32c89b3c3191e9"><code>9b1ee5b</code></a>
  v2 release note (<a
              href="https://redirect.github.com/arduino/setup-protoc/issues/82">#82</a>)</li>
  <li><a
              href="https://github.com/arduino/setup-protoc/commit/28fd3e5ddcc4ae8820e0c2085bfea8ab68f631e3"><code>28fd3e5</code></a>
  Support only the new protobuf versioning scheme (<a
              href="https://redirect.github.com/arduino/setup-protoc/issues/78">#78</a>)</li>
  <li>See full diff in <a
              href="https://github.com/arduino/setup-protoc/compare/v1...v2">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=arduino/setup-protoc&package-manager=github_actions&previous-version=1&new-version=2)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 14 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#116](https://github.com/substrait-io/substrait-rs/issues/116)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#116](https://github.com/substrait-io/substrait-rs/issues/116)**
  - Bump arduino/setup-protoc from 1 to 2 ([`e68707c`](https://github.com/substrait-io/substrait-rs/commit/e68707c74066bd0c1069d6fc135336edc2cd26d7))
- **Uncategorized** - Release substrait v0.12.4 ([`27bad38`](https://github.com/substrait-io/substrait-rs/commit/27bad38ce578f02bd66951901732920b734a0762))
</details>

## 0.12.3 (2023-07-18)

<csr-id-59644c4b9cab3f0330372dd7883805c0627f02c1/>

### Chore

- <csr-id-59644c4b9cab3f0330372dd7883805c0627f02c1/> bump actions/upload-pages-artifact from 1 to 2
  Bumps
  [actions/upload-pages-artifact](https://github.com/actions/upload-pages-artifact)
  from 1 to 2.
  <details>
  <summary>Release notes</summary>
  <p><em>Sourced from <a
              href="https://github.com/actions/upload-pages-artifact/releases">actions/upload-pages-artifact's
  releases</a>.</em></p>
  <blockquote>
  <h2>v2.0.0</h2>
  <h1>Changelog</h1>
  <ul>
  <li>:warning: <strong>BREAKING CHANGE:</strong> Remove built-in
  <code>chmod</code> commands for <code>v2</code> <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/69">#69</a>)</li>
  <li>Update README for <code>v2</code> <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/70">#70</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/upload-pages-artifact/compare/v1.0.10...v2.0.0">all
  code changes</a> since previous release.</p>
  <h2>v1.0.10</h2>
  <h1>Changelog</h1>
  <ul>
  <li>readme: fix/improve note about permissions <a
              href="https://github.com/tshepang"><code>@​tshepang</code></a> (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/65">#65</a>)</li>
  <li>Revert <code>chmod</code> removal for <code>v1</code> <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/68">#68</a>)</li>
  <li>Add file perms handling <a
              href="https://github.com/tsusdere"><code>@​tsusdere</code></a> (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/64">#64</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/upload-pages-artifact/compare/v1.0.9...v1.0.10">all
  code changes</a> since previous release.</p>
  <h2>v1.0.9</h2>
  <p>Removed <code>chmod</code> as we moved towards trusting correct file
  permissions have been set. In the event this isn't the case then we
  raise an error in the action related to the file permissions.</p>
  <h2>v1.0.8</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Fail if no artifact file is found to upload <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/55">#55</a>)</li>
  <li>Fix link to releases in README <a
              href="https://github.com/waldyrious"><code>@​waldyrious</code></a> (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/53">#53</a>)</li>
  <li>Bump actions/publish-action from 0.2.1 to 0.2.2 <a
              href="https://github.com/dependabot"><code>@​dependabot</code></a> (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/47">#47</a>)</li>
  <li>Add Dependabot config for Actions usage updates <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/46">#46</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/upload-pages-artifact/compare/v1.0.7...v1.0.8">all
  code changes</a> since previous release.</p>
  <h2>v1.0.7</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Don't change file permissions of other files <a
              href="https://github.com/KyeRussell"><code>@​KyeRussell</code></a> (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/44">#44</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/upload-pages-artifact/compare/v1.0.6...v1.0.7">all
  code changes</a> since previous release.</p>
  <h2>v1.0.6</h2>
  <h1>Changelog</h1>
  <ul>
  <li>Customize artifact name <a
              href="https://github.com/yuradanyliuk"><code>@​yuradanyliuk</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/41">#41</a>)</li>
  <li>Fix permissions <a
              href="https://github.com/yoannchaudet"><code>@​yoannchaudet</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/42">#42</a>)</li>
  <li>Print warnings about changed file permissions in bulk <a
              href="https://github.com/TooManyBees"><code>@​TooManyBees</code></a> (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/38">#38</a>)</li>
  <li>Update to latest <code>actions/publish-action</code> <a
              href="https://github.com/JamesMGreene"><code>@​JamesMGreene</code></a>
  (<a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/36">#36</a>)</li>
  </ul>
  <p>See details of <a
              href="https://github.com/actions/upload-pages-artifact/compare/v1.0.5...v1.0.6">all
  code changes</a> since previous release.</p>
  <h2>v1.0.5</h2>
  <h1>Changelog</h1>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/a753861a5debcf57bf8b404356158c8e1e33150c"><code>a753861</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/69">#69</a>
  from actions/reapply-chmod-removal-for-v2</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/dca6bac0e5d658c10e82a0e89fa26dd05f9ef2e3"><code>dca6bac</code></a>
  Merge branch 'main' into reapply-chmod-removal-for-v2</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/3138c054964e4f34b7f958dc2ae603b51877d906"><code>3138c05</code></a>
  Merge pull request <a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/70">#70</a>
  from actions/v2-docs-improvements</li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/07f501f6a0ff8cef2d64e4037d704f79061a4bd5"><code>07f501f</code></a>
  Update README for <code>v2</code></li>
  <li><a
              href="https://github.com/actions/upload-pages-artifact/commit/9c071e6bed590ca0b53a706c2d01ad1c39faf659"><code>9c071e6</code></a>
  Reapply PR <a
              href="https://redirect.github.com/actions/upload-pages-artifact/issues/63">#63</a>
  for v2</li>
  <li>See full diff in <a
              href="https://github.com/actions/upload-pages-artifact/compare/v1...v2">compare
  view</a></li>
  </ul>
  </details>
  <br />

  [![Dependabot compatibility
score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=actions/upload-pages-artifact&package-manager=github_actions&previous-version=1&new-version=2)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 15 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#113](https://github.com/substrait-io/substrait-rs/issues/113)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#113](https://github.com/substrait-io/substrait-rs/issues/113)**
  - Bump actions/upload-pages-artifact from 1 to 2 ([`59644c4`](https://github.com/substrait-io/substrait-rs/commit/59644c4b9cab3f0330372dd7883805c0627f02c1))
- **Uncategorized** - Release substrait v0.12.3 ([`40a5c29`](https://github.com/substrait-io/substrait-rs/commit/40a5c296c2bb12e673d5920340e1bd46b4cd8195))
</details>

## 0.12.2 (2023-07-03)

<csr-id-9d6609152046771cdaee6b2cd4dba39d37dbec36/>

### Chore

- <csr-id-9d6609152046771cdaee6b2cd4dba39d37dbec36/> update `CONTRIBUTING.md` to reflect workflow changes
  We no longer use bors (#103) and the merge workflow was removed as part
  of that change.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#109](https://github.com/substrait-io/substrait-rs/issues/109)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#109](https://github.com/substrait-io/substrait-rs/issues/109)**
  - Update `CONTRIBUTING.md` to reflect workflow changes ([`9d66091`](https://github.com/substrait-io/substrait-rs/commit/9d6609152046771cdaee6b2cd4dba39d37dbec36))
- **Uncategorized** - Release substrait v0.12.2 ([`5da1b55`](https://github.com/substrait-io/substrait-rs/commit/5da1b555e2d67bd2a6b9a50b56d2037bd501120a))
</details>

## 0.12.1 (2023-07-03)

<csr-id-26748b8346c6d349877065d4798f7ecf085a8256/>

### Chore

- <csr-id-26748b8346c6d349877065d4798f7ecf085a8256/> fix doc deployment file permissions
  Change required to handle the [failed deploy
  job](https://github.com/substrait-io/substrait-rs/actions/runs/5438507514/jobs/9889794289).
  Caused by a change in the `upload-pages-artifact` action:
  https://github.com/actions/upload-pages-artifact#file-permissions. Also
  added an index file that redirects the client to the substrait crate
  docs.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#112](https://github.com/substrait-io/substrait-rs/issues/112)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#112](https://github.com/substrait-io/substrait-rs/issues/112)**
  - Fix doc deployment file permissions ([`26748b8`](https://github.com/substrait-io/substrait-rs/commit/26748b8346c6d349877065d4798f7ecf085a8256))
- **Uncategorized** - Release substrait v0.12.1 ([`a96b3b1`](https://github.com/substrait-io/substrait-rs/commit/a96b3b1945c81a59330a0dcf3671a90bf6f50a9b))
</details>

## 0.12.0 (2023-07-02)

<csr-id-c822997737ee92f246258a37265d54dffd0846b7/>

### Chore (BREAKING)

- <csr-id-c822997737ee92f246258a37265d54dffd0846b7/> bump substrait from `0.30.0` to `0.31.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `3259a1b` to `e486775`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/e486775009c40e1a010dc54776b976b1eddea7ca"><code>e486775</code></a>
  chore(release): 0.31.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/a6ead70b1d62b79fad7ba2f9fdaf76c5b6d7696b"><code>a6ead70</code></a>
  feat: add a two-arg variant of substring (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/513">#513</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/694340013433b1c0408c2a1cd77b22dfb9b22ad0"><code>6943400</code></a>
  feat: add timestamp types to max/min function (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/511">#511</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/93a41d16bc3835a30be987d5b2c3cddb363ea7cb"><code>93a41d1</code></a>
  chore(deps): update pymdown-extensions requirement (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/500">#500</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/9b763a008de8fac12622353ce65e856389dd1c0e"><code>9b763a0</code></a>
  chore: add a CODEOWNERS file (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/454">#454</a>)</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/83dd1d3b481bcd93ee03a54d648f57dd2b1f3354"><code>83dd1d3</code></a>
  ci: fetch collaborators from github (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/403">#403</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/3259a1bab342caac73ba3a3aaaff58cd9a91691b...e486775009c40e1a010dc54776b976b1eddea7ca">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 2 commits contributed to the release.
- 26 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#111](https://github.com/substrait-io/substrait-rs/issues/111)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#111](https://github.com/substrait-io/substrait-rs/issues/111)**
  - Bump substrait from `0.30.0` to `0.31.0` ([`c822997`](https://github.com/substrait-io/substrait-rs/commit/c822997737ee92f246258a37265d54dffd0846b7))
- **Uncategorized** - Release substrait v0.12.0 ([`5cfeade`](https://github.com/substrait-io/substrait-rs/commit/5cfeade3c2772f8f4e9352e3edf02ae5b8bbc928))
</details>

## 0.11.0 (2023-06-06)

<csr-id-ca60950ebc61981f5ef583d0a60e9f11e06a4329/>
<csr-id-b75a5b8f39b79a115444bcc6c86a06a8ab924493/>
<csr-id-2f4795a41683276ef21eb91a2428e8df0a984760/>
<csr-id-3ef303863d91d791b1595243f1e048d67906fe28/>
<csr-id-918cd9c201c0c194d371f8e718b687735b24a01c/>
<csr-id-237ce185213f0414209dd68e08ffeed5c3fd0a6c/>

### Chore

- <csr-id-ca60950ebc61981f5ef583d0a60e9f11e06a4329/> bump version to 0.10.0 to fix smart release
  Since the release workflow was broken 0.10.0 was pushed to crates.io,
  but the commit with changelog and version bump was not pushed to main.
  This fixes the version to make sure the next smart release can bump to
  0.11.0.
- <csr-id-b75a5b8f39b79a115444bcc6c86a06a8ab924493/> update typify requirement from 0.0.12 to 0.0.13
  Updates the requirements on
  [typify](https://github.com/oxidecomputer/typify) to permit the latest
  version.
  <details>
  <summary>Changelog</summary>
  <p><em>Sourced from <a
  href="https://github.com/oxidecomputer/typify/blob/main/CHANGELOG.adoc">typify's
  changelog</a>.</em></p>
  <blockquote>
  <p>== 0.0.13 (released 2023-05-14)</p>
  <ul>
  <li>Fixed-length, single-type arrays to <code>[T; N]</code> (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/286">#286</a>)</li>
  <li>Support for reflexive schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/292">#292</a>)</li>
  <li>Much improved support for multi-type schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/291">#291</a>)</li>
  <li>Better error messages on failures</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.12%5C...v0.0.13%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.12\...v0.0.13[Full</a>
  list of commits]</p>
  <p>== 0.0.12 (released 2023-05-03)</p>
  <ul>
  <li>Improved enum generation (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/270">#270</a>)</li>
  <li>Improved integer type selection based on number criteria (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/255">#255</a>)</li>
  <li><code>TypeSpace::add_root_schema()</code> (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/236">#236</a>)</li>
  <li>... and many general improvements</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.11%5C...v0.0.12%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.11\...v0.0.12[Full</a>
  list of commits]</p>
  <p>== 0.0.11 (released 2023-03-18)</p>
  <p>This is a big update with many, many changes to code generation, and
  many more
  JSON schema structures well-handled. Among the many changes:</p>
  <ul>
  <li>Generate a <code>ToString</code> impl for untagged enums with
  trivial variants (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/145">#145</a>)</li>
  <li>Allow conversion overrides by specifying a schema (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/155">#155</a>)</li>
  <li>Handle untyped enums that contain nulls (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/167">#167</a>)</li>
  <li>Handle <code>not</code> schemas for enumerated values (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/168">#168</a>)</li>
  <li>Improve generated code for FromStr and TryFrom impls (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/174">#174</a>)</li>
  <li>Handle format specifiers for enumerated strings (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/188">#188</a>)</li>
  </ul>
  <p>=== <em>Breaking</em>: The removal of
  <code>TypeSpace::to_string()</code></p>
  <p>Previously all transitive consumers required the presence of
  <code>rustfmt</code>. In this
  version we leave formatting to the consumer. See
  link:README.md#formatting[the formatting section of the README] for
  details on formatting.</p>
  <p>=== CLI</p>
  <p>This version adds the <code>cargo-typify</code> crate for stand-alone
  code generation.</p>
  <p>=== Augmented Generation</p>
  <p>Consumers can now affect how code is generated in several ways:</p>
  <ul>
  <li>adding derive macros to all generated types</li>
  <li>modifying specific types by name to rename them or add derive
  macros</li>
  <li>specifying a replacement type by name</li>
  <li>specifying a replacement type by schema pattern</li>
  </ul>
  <p><a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.10%5C...v0.0.11%5BFull">https://github.com/oxidecomputer/typify/compare/v0.0.10\...v0.0.11[Full</a>
  list of commits]</p>
  <!-- raw HTML omitted -->
  </blockquote>
  <p>... (truncated)</p>
  </details>
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/5e966779d4132f3baa128d9df2f83e4a7ff68378"><code>5e96677</code></a>
  release typify 0.0.13</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/db0740ef6dea2212a04ad9c048263ba054894b63"><code>db0740e</code></a>
  prep for 0.0.13</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/99ec168e1f4c4dc37bcc79759dd7f4b3d904102f"><code>99ec168</code></a>
  use serde_json::Map with serde_json::Value (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/226">#226</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/958b86e8e5183b7384257ac8a1194daff9c8b817"><code>958b86e</code></a>
  support for reflexive schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/292">#292</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/59de2f7b03a58c8a9ad2cd81f8bb7fce8c9e59bb"><code>59de2f7</code></a>
  handle more complex type arrays (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/291">#291</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/28db444b60335af3db6e72e9be7f121148b8fdef"><code>28db444</code></a>
  improve error message for external references (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/288">#288</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/28883bdb3868fd1f5c02d25bc5d5bd7ce7d9cc30"><code>28883bd</code></a>
  Bump clap from 4.2.5 to 4.2.7 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/284">#284</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/fde91c3c9eb735f1e288e6dc98bd2ab80a0dca11"><code>fde91c3</code></a>
  Bump serde from 1.0.160 to 1.0.162 (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/285">#285</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/031d3156dbe1593d3b620a1e92ef108e1cc4deee"><code>031d315</code></a>
  emit fixed-length arrays for single-type fixed length array schemas (<a
  href="https://redirect.github.com/oxidecomputer/typify/issues/286">#286</a>)</li>
  <li><a
  href="https://github.com/oxidecomputer/typify/commit/736aa9ec1bb217d98c31c706ef2179974c9feaf1"><code>736aa9e</code></a>
  stray println</li>
  <li>Additional commits viewable in <a
  href="https://github.com/oxidecomputer/typify/compare/v0.0.12...v0.0.13">compare
  view</a></li>
  </ul>
  </details>
  <br />

  You can trigger a rebase of this PR by commenting `@dependabot rebase`.

- <csr-id-2f4795a41683276ef21eb91a2428e8df0a984760/> fix dependabot commit prefix configuration to include colon
  Noticed in https://github.com/substrait-io/substrait-rs/pull/56 that a colon is
  missing from the commit prefix configuration.
  We need this to match the conventional commit spec.
- <csr-id-3ef303863d91d791b1595243f1e048d67906fe28/> use `git2` instead of `gix` to reduce dependency graph
  Use [git2](https://docs.rs/git2/latest/git2/) instead of [gix](https://docs.rs/gix/latest/gix/) to reduce the dependency graph.

### Bug Fixes

- <csr-id-6e5ca090b27e7dce785119060ef9ee9c53822b22/> typo in .gitignore

### Chore (BREAKING)

- <csr-id-918cd9c201c0c194d371f8e718b687735b24a01c/> remove references to bors and `merge_group`
  To fix the release workflow I'm marking this as breaking change.
- <csr-id-237ce185213f0414209dd68e08ffeed5c3fd0a6c/> bump substrait from `0.29.0` to `0.30.0`
  Bumps [substrait](https://github.com/substrait-io/substrait) from
  `16503aa` to `3259a1b`.
  <details>
  <summary>Commits</summary>
  <ul>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/3259a1bab342caac73ba3a3aaaff58cd9a91691b"><code>3259a1b</code></a>
  chore(release): 0.30.0</li>
  <li><a
  href="https://github.com/substrait-io/substrait/commit/aacd25c8fa5eb680c3456d2e0298ca0807eb7b87"><code>aacd25c</code></a>
  feat: control indexing in temporal extraction (<a
  href="https://redirect.github.com/substrait-io/substrait/issues/479">#479</a>)</li>
  <li>See full diff in <a
  href="https://github.com/substrait-io/substrait/compare/16503aaf412a3a4771fc0d17b5ac4883e26954aa...3259a1bab342caac73ba3a3aaaff58cd9a91691b">compare
  view</a></li>
  </ul>
  </details>
  <br />

  Dependabot will resolve any conflicts with this PR as long as you don't
  alter it yourself. You can also trigger a rebase manually by commenting
  `@dependabot rebase`.

### Commit Statistics

<csr-read-only-do-not-edit/>

- 10 commits contributed to the release over the course of 28 calendar days.
- 29 days passed between releases.
- 7 commits were understood as [conventional](https://www.conventionalcommits.org).
- 9 unique issues were worked on: [#102](https://github.com/substrait-io/substrait-rs/issues/102), [#104](https://github.com/substrait-io/substrait-rs/issues/104), [#105](https://github.com/substrait-io/substrait-rs/issues/105), [#106](https://github.com/substrait-io/substrait-rs/issues/106), [#107](https://github.com/substrait-io/substrait-rs/issues/107), [#108](https://github.com/substrait-io/substrait-rs/issues/108), [#58](https://github.com/substrait-io/substrait-rs/issues/58), [#87](https://github.com/substrait-io/substrait-rs/issues/87), [#90](https://github.com/substrait-io/substrait-rs/issues/90)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#102](https://github.com/substrait-io/substrait-rs/issues/102)**
  - Use `git2` instead of `gix` to reduce dependency graph ([`3ef3038`](https://github.com/substrait-io/substrait-rs/commit/3ef303863d91d791b1595243f1e048d67906fe28))
- **[#104](https://github.com/substrait-io/substrait-rs/issues/104)**
  - Remove bors and add merge queue checks ([`c8a7db0`](https://github.com/substrait-io/substrait-rs/commit/c8a7db0bb144fa50e4bf11c3e7e81f95d0da2238))
- **[#105](https://github.com/substrait-io/substrait-rs/issues/105)**
  - Bump substrait from `0.29.0` to `0.30.0` ([`237ce18`](https://github.com/substrait-io/substrait-rs/commit/237ce185213f0414209dd68e08ffeed5c3fd0a6c))
- **[#106](https://github.com/substrait-io/substrait-rs/issues/106)**
  - Update typify requirement from 0.0.12 to 0.0.13 ([`b75a5b8`](https://github.com/substrait-io/substrait-rs/commit/b75a5b8f39b79a115444bcc6c86a06a8ab924493))
- **[#107](https://github.com/substrait-io/substrait-rs/issues/107)**
  - Remove references to bors and `merge_group` ([`918cd9c`](https://github.com/substrait-io/substrait-rs/commit/918cd9c201c0c194d371f8e718b687735b24a01c))
- **[#108](https://github.com/substrait-io/substrait-rs/issues/108)**
  - Bump version to 0.10.0 to fix smart release ([`ca60950`](https://github.com/substrait-io/substrait-rs/commit/ca60950ebc61981f5ef583d0a60e9f11e06a4329))
- **[#58](https://github.com/substrait-io/substrait-rs/issues/58)**
  - Fix dependabot commit prefix configuration to include colon ([`2f4795a`](https://github.com/substrait-io/substrait-rs/commit/2f4795a41683276ef21eb91a2428e8df0a984760))
- **[#87](https://github.com/substrait-io/substrait-rs/issues/87)**
  - Limit release job concurrency ([`5a7cf72`](https://github.com/substrait-io/substrait-rs/commit/5a7cf721800df05ddbe84aaf11091c7a0ed0e69c))
- **[#90](https://github.com/substrait-io/substrait-rs/issues/90)**
  - Typo in .gitignore ([`6e5ca09`](https://github.com/substrait-io/substrait-rs/commit/6e5ca090b27e7dce785119060ef9ee9c53822b22))
- **Uncategorized** - Release substrait v0.11.0 ([`313249e`](https://github.com/substrait-io/substrait-rs/commit/313249e2faffb67268632add1d0902468587a69c))
</details>

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

- 2 commits contributed to the release.
- 10 days passed between releases.
- 1 commit was understood as [conventional](https://www.conventionalcommits.org).
- 1 unique issue was worked on: [#101](https://github.com/substrait-io/substrait-rs/issues/101)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

- **[#101](https://github.com/substrait-io/substrait-rs/issues/101)**
  - Update typify requirement from 0.0.11 to 0.0.12 ([`9ed17a9`](https://github.com/substrait-io/substrait-rs/commit/9ed17a9cb9bfcf31ccee656679ae5217365b44ea))
- **Uncategorized** - Release substrait v0.9.0 ([`1d2d4e7`](https://github.com/substrait-io/substrait-rs/commit/1d2d4e7359ec9cc11976ad5f81a7729e71a41f4f))
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

- **[#97](https://github.com/substrait-io/substrait-rs/issues/97)**
  - Update gix to 0.44 ([`e7b12ac`](https://github.com/substrait-io/substrait-rs/commit/e7b12ac1056fbbe4e227b7019c13cdfafcb63d09))
- **Uncategorized** - Release substrait v0.8.1 ([`74cb1fe`](https://github.com/substrait-io/substrait-rs/commit/74cb1fe0640d5d57b8155ff843d5eb4c15b22d91))
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

- **[#96](https://github.com/substrait-io/substrait-rs/issues/96)**
  - Bump substrait from 0.28.2 to 0.29.0 ([`32f5a9d`](https://github.com/substrait-io/substrait-rs/commit/32f5a9dd33e298798d487830de81a93fea6139f8))
- **Uncategorized** - Release substrait v0.8.0 ([`2feae85`](https://github.com/substrait-io/substrait-rs/commit/2feae85523dde923599dae32bfa383774f380f74))
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

- **[#95](https://github.com/substrait-io/substrait-rs/issues/95)**
  - Skip re-runs if there is no substrait git submodule ([`e8aceb2`](https://github.com/substrait-io/substrait-rs/commit/e8aceb239cd7dba40f8d7dbd471bddaa5065dff9))
- **Uncategorized** - Release substrait v0.7.5 ([`6f064c8`](https://github.com/substrait-io/substrait-rs/commit/6f064c83127924481ad1c57e3150a91dd7caa5b0))
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

- **[#93](https://github.com/substrait-io/substrait-rs/issues/93)**
  - Bump substrait from 0.28.1 to 0.28.2 ([`83a18ee`](https://github.com/substrait-io/substrait-rs/commit/83a18ee71b2174cd3e1118ae789cb0e09d4a3b2e))
- **Uncategorized** - Release substrait v0.7.4 ([`e2f0cc9`](https://github.com/substrait-io/substrait-rs/commit/e2f0cc957122cfa07a88739ea13570e29ffa87c7))
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

- **[#91](https://github.com/substrait-io/substrait-rs/issues/91)**
  - Bump substrait from 0.28.0 to 0.28.1 ([`fb61e47`](https://github.com/substrait-io/substrait-rs/commit/fb61e47daa9ad330bc2fbedbebf992313b5943b1))
- **Uncategorized** - Release substrait v0.7.3 ([`9e9c1c7`](https://github.com/substrait-io/substrait-rs/commit/9e9c1c7c6ae0b84ab4ffa4c504b858c6ff0ff82d))
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

- **[#88](https://github.com/substrait-io/substrait-rs/issues/88)**
  - Add .vscode to gitignore ([`d390c76`](https://github.com/substrait-io/substrait-rs/commit/d390c76ec390d5a346d19339eaffc69e5351452e))
- **Uncategorized** - Release substrait v0.7.2 ([`c83e2a7`](https://github.com/substrait-io/substrait-rs/commit/c83e2a757b298d630ff6cca38106e37524153378))
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

- **[#78](https://github.com/substrait-io/substrait-rs/issues/78)**
  - Remove filter that skipped `extension_types.yaml` deserialize test ([`6445de8`](https://github.com/substrait-io/substrait-rs/commit/6445de8a35a62fd095cd5ae1c9410f51a5f7e17f))
- **Uncategorized** - Release substrait v0.7.1 ([`98b3a7b`](https://github.com/substrait-io/substrait-rs/commit/98b3a7b2ce25c199b6a68181c618b955665e97ab))
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

- **[#75](https://github.com/substrait-io/substrait-rs/issues/75)**
  - Bump actions/deploy-pages from 1 to 2 ([`11d44c8`](https://github.com/substrait-io/substrait-rs/commit/11d44c88162d76cfaf5b9ce479a864f0d3e56879))
- **[#80](https://github.com/substrait-io/substrait-rs/issues/80)**
  - Update gix requirement from 0.42 to 0.43 ([`6d11773`](https://github.com/substrait-io/substrait-rs/commit/6d117739ee41eb8a680931e57ea263539196a96f))
- **[#86](https://github.com/substrait-io/substrait-rs/issues/86)**
  - Bump substrait from `7f272f1` to `26da4f1` ([`b766929`](https://github.com/substrait-io/substrait-rs/commit/b76692943ae2a5cb7c8fa83f4d8c502b265a0980))
- **Uncategorized** - Release substrait v0.7.0 ([`4d94c48`](https://github.com/substrait-io/substrait-rs/commit/4d94c48417901f77f4474560fec6dd1514ee86d0))
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

- **[#60](https://github.com/substrait-io/substrait-rs/issues/60)**
  - Bump actions/configure-pages from 2 to 3 ([`b4bf64c`](https://github.com/substrait-io/substrait-rs/commit/b4bf64c476038d20e5858ed1171397baec7bb5fa))
- **Uncategorized** - Release substrait v0.6.1 ([`f5fa994`](https://github.com/substrait-io/substrait-rs/commit/f5fa9943e6fc1222e8d51f2ec69de8c24216326c))
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

- **[#84](https://github.com/substrait-io/substrait-rs/issues/84)**
  - Update typify, prettyplease and syn ([`d2e7ac8`](https://github.com/substrait-io/substrait-rs/commit/d2e7ac8dad29754ab190207e9a3dd39f3deb1b1f))
- **Uncategorized** - Release substrait v0.6.0 ([`7a12bed`](https://github.com/substrait-io/substrait-rs/commit/7a12bed30c3f8361f4f0a74f3b514f07ab2e8b90))
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

- **[#81](https://github.com/substrait-io/substrait-rs/issues/81)**
  - Checkout repository in pull request check job to get config file ([`63aa213`](https://github.com/substrait-io/substrait-rs/commit/63aa2137f369dde6f1ce8dfe9f3719d569020319))
- **Uncategorized** - Release substrait v0.5.4 ([`8c1ded3`](https://github.com/substrait-io/substrait-rs/commit/8c1ded316241597ce8188fac9b817aba99bbda67))
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

- **[#73](https://github.com/substrait-io/substrait-rs/issues/73)**
  - Replace removed `typfify::TypeSpace::to_string()` with `prettyplease` ([`3991a0f`](https://github.com/substrait-io/substrait-rs/commit/3991a0f9e25edf9e7ecce112916f322f39089702))
- **Uncategorized** - Release substrait v0.5.3 ([`622b4bb`](https://github.com/substrait-io/substrait-rs/commit/622b4bb251e47a7d60b80c372633bd0348034bbe))
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

- **[#77](https://github.com/substrait-io/substrait-rs/issues/77)**
  - Add commitlint config file to disable max line length limits ([`ef41bcf`](https://github.com/substrait-io/substrait-rs/commit/ef41bcf9c346095e0f930211c688387d60e2ec8d))
- **Uncategorized** - Release substrait v0.5.2 ([`7964825`](https://github.com/substrait-io/substrait-rs/commit/7964825e0566b79235ba10e5d58c54d1e839d15e))
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

- **[#46](https://github.com/substrait-io/substrait-rs/issues/46)**
  - Add `version` module with Substrait version information ([`c3b72ae`](https://github.com/substrait-io/substrait-rs/commit/c3b72ae3c29744ed3cf6bb6ad7f438449b1e85ac))
- **Uncategorized** - Release substrait v0.5.1 ([`e177e14`](https://github.com/substrait-io/substrait-rs/commit/e177e14d2387e2b6025c89e67f1e3e03503a5bca))
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

- **[#59](https://github.com/substrait-io/substrait-rs/issues/59)**
  - Bump `prost-wkt` dependencies to 0.4 ([`9a562fa`](https://github.com/substrait-io/substrait-rs/commit/9a562fa749d8fb51806261982d357c282130eea6))
- **Uncategorized** - Release substrait v0.5.0 ([`749f958`](https://github.com/substrait-io/substrait-rs/commit/749f958a61a1969a5325dae3530919a97d7d3789))
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

- **[#57](https://github.com/substrait-io/substrait-rs/issues/57)**
  - Also check PR body for conventional commits ([`39e3441`](https://github.com/substrait-io/substrait-rs/commit/39e3441003de1b4c315a82a75fdbc9bb31bd1de3))
- **[#72](https://github.com/substrait-io/substrait-rs/issues/72)**
  - Update crates index when creating releases ([`e52ab6f`](https://github.com/substrait-io/substrait-rs/commit/e52ab6f38fdd96a113f646de85e1fdd4b38e7548))
- **Uncategorized** - Release substrait v0.4.2 ([`4321970`](https://github.com/substrait-io/substrait-rs/commit/4321970db81bdf5f64bb8beec04713378e94ba19))
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

- **[#69](https://github.com/substrait-io/substrait-rs/issues/69)**
  - Ignore dtolnay/rust-toolchain updates ([`1b193ae`](https://github.com/substrait-io/substrait-rs/commit/1b193ae332a649b2e0d8a07f1cde98fa90131c3a))
- **Uncategorized** - Release substrait v0.4.1 ([`a0e320f`](https://github.com/substrait-io/substrait-rs/commit/a0e320f805789a180f86a0bb1262c37b774d3df7))
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

- **[#1](https://github.com/substrait-io/substrait-rs/issues/1)**
  - Rust bindings for substrait.io ([`2fb8390`](https://github.com/substrait-io/substrait-rs/commit/2fb83909ca5cb6770f1351441a7222f56b2f174e))
- **[#10](https://github.com/substrait-io/substrait-rs/issues/10)**
  - Add repo-token to setup-protoc action to avoid rate limiting ([`1b03e84`](https://github.com/substrait-io/substrait-rs/commit/1b03e8497094fc4338995b05a0d5fd2fd1ae09f0))
- **[#11](https://github.com/substrait-io/substrait-rs/issues/11)**
  - Change gitsubmodule schedule to match substrait weekly release ([`146979a`](https://github.com/substrait-io/substrait-rs/commit/146979a89006ed401879fb7f9b3862f6c4090c14))
- **[#14](https://github.com/substrait-io/substrait-rs/issues/14)**
  - Change schedule time for Dependabot submodule updates ([`1774935`](https://github.com/substrait-io/substrait-rs/commit/1774935d3d1a9bec800b6f8f2599ad8256f6ab23))
- **[#2](https://github.com/substrait-io/substrait-rs/issues/2)**
  - Use latest substrait & bump crate version ([`7852c8a`](https://github.com/substrait-io/substrait-rs/commit/7852c8a33882ae8ebb65dc4fba9a046c6e8d4a2d))
- **[#23](https://github.com/substrait-io/substrait-rs/issues/23)**
  - Bump pbjson from 0.5.0 to 0.5.1 ([`510fb8f`](https://github.com/substrait-io/substrait-rs/commit/510fb8fdd1d2b3016c733c7b7d6f23ee3192d231))
- **[#26](https://github.com/substrait-io/substrait-rs/issues/26)**
  - Replace unmaintained action-rs actions ([`cf9c0dd`](https://github.com/substrait-io/substrait-rs/commit/cf9c0dd5d169f899adcaf0540a4fb29c9b74dcc6))
- **[#27](https://github.com/substrait-io/substrait-rs/issues/27)**
  - Generate types to deserialize simple extensions ([`2747477`](https://github.com/substrait-io/substrait-rs/commit/27474774b8baff84457db94b2c7deefdc152831f))
- **[#29](https://github.com/substrait-io/substrait-rs/issues/29)**
  - Bump prost from 0.11.0 to 0.11.3 ([`acdf53d`](https://github.com/substrait-io/substrait-rs/commit/acdf53d852ee12c51e3ba07938d8a8f5582541fb))
- **[#30](https://github.com/substrait-io/substrait-rs/issues/30)**
  - Bump prost-build from 0.11.1 to 0.11.3 ([`12d759b`](https://github.com/substrait-io/substrait-rs/commit/12d759b77a619c8939131d25bce703b94a76f13e))
- **[#31](https://github.com/substrait-io/substrait-rs/issues/31)**
  - Bump substrait from `018da38` to `7f272f1` ([`a1adbc7`](https://github.com/substrait-io/substrait-rs/commit/a1adbc74a4437abc61567a48b4690340e8fa5544))
- **[#32](https://github.com/substrait-io/substrait-rs/issues/32)**
  - Add `bors.toml` ([`80c0676`](https://github.com/substrait-io/substrait-rs/commit/80c067668d400911bdd0e357aceb767941727acf))
- **[#33](https://github.com/substrait-io/substrait-rs/issues/33)**
  - Disable auto rebase for cargo dependabot updates ([`61d41b1`](https://github.com/substrait-io/substrait-rs/commit/61d41b1f26cc145ccfefa16cf15298cea7704576))
- **[#35](https://github.com/substrait-io/substrait-rs/issues/35)**
  - Bump pbjson-types from 0.5.0 to 0.5.1 ([`9e7c02f`](https://github.com/substrait-io/substrait-rs/commit/9e7c02fbd434f143c229e6f340bf5a36801fe251))
- **[#37](https://github.com/substrait-io/substrait-rs/issues/37)**
  - Bump serde_json from 1.0.85 to 1.0.89 ([`bfb8031`](https://github.com/substrait-io/substrait-rs/commit/bfb8031008eff6dc98eaf3f22f89995f67d82401))
- **[#39](https://github.com/substrait-io/substrait-rs/issues/39)**
  - Bump serde from 1.0.144 to 1.0.150 ([`aa6c650`](https://github.com/substrait-io/substrait-rs/commit/aa6c650f104a509a464e2533aaeeae823b58920d))
- **[#4](https://github.com/substrait-io/substrait-rs/issues/4)**
  - Corrected repo link address in README.md and Cargo.toml ([`d8c6183`](https://github.com/substrait-io/substrait-rs/commit/d8c6183449788fd72f96df854e88f9d515166c02))
- **[#40](https://github.com/substrait-io/substrait-rs/issues/40)**
  - Bump prost-build from 0.11.3 to 0.11.4 ([`bd104c1`](https://github.com/substrait-io/substrait-rs/commit/bd104c1379648f308599192af361a31d626d7d4a))
- **[#41](https://github.com/substrait-io/substrait-rs/issues/41)**
  - Bump version to 0.3.0 ([`b852f01`](https://github.com/substrait-io/substrait-rs/commit/b852f01d09c0d6b8c26c6ceebcf19572402b9e6b))
- **[#42](https://github.com/substrait-io/substrait-rs/issues/42)**
  - Bump substrait from `7f272f1` to `81e34d4` ([`40e8adf`](https://github.com/substrait-io/substrait-rs/commit/40e8adf629f438b57354c23fefad61bba7843f7a))
- **[#48](https://github.com/substrait-io/substrait-rs/issues/48)**
  - Follow conventional commits and setup automated releases ([`f8f50d3`](https://github.com/substrait-io/substrait-rs/commit/f8f50d3907a2fcbc6b5a09ff3c1b8e541c4b227e))
- **[#5](https://github.com/substrait-io/substrait-rs/issues/5)**
  - Setup GitHub actions and Dependabot ([`7418a17`](https://github.com/substrait-io/substrait-rs/commit/7418a17078a8ebfd062a17e4cf989a64a1aa52c2))
- **[#6](https://github.com/substrait-io/substrait-rs/issues/6)**
  - Add `pbjson` feature for Protobuf JSON serde ([`43f97ef`](https://github.com/substrait-io/substrait-rs/commit/43f97ef0e1311bdd909c58afad429b4db54fa30d))
- **Uncategorized** - Release substrait v0.4.0 ([`65d5f3d`](https://github.com/substrait-io/substrait-rs/commit/65d5f3d8f48c3ae990c311ae8ed4590a76e57c3e)) - Merge #24 ([`c726d2f`](https://github.com/substrait-io/substrait-rs/commit/c726d2fbab168487a975601d12c7b9462842b78b)) - Bump pbjson-build from 0.5.0 to 0.5.1 ([`032eba1`](https://github.com/substrait-io/substrait-rs/commit/032eba11a6932fabe4fe1bc7f527c995a1056c9f)) - Update LICENSE ([`06a611b`](https://github.com/substrait-io/substrait-rs/commit/06a611b1e46c39908aa55c51e4a9cb390c9a8091)) - Initial commit ([`8edc827`](https://github.com/substrait-io/substrait-rs/commit/8edc827a4a3c5ae6de704d3d4238774d379c6c23))
</details>
