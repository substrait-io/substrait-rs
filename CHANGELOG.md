

## 0.6.0 (2023-03-30)

<csr-id-d2e7ac8dad29754ab190207e9a3dd39f3deb1b1f/>

### Chore (BREAKING)

 - <csr-id-d2e7ac8dad29754ab190207e9a3dd39f3deb1b1f/> update typify, prettyplease and syn
   Cherry-pick of #76, #82 and #83. These need to be combined.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#84](https://github.com/substrait-io/substrait-rs/issues/84)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#84](https://github.com/substrait-io/substrait-rs/issues/84)**
    - Update typify, prettyplease and syn ([`d2e7ac8`](https://github.com/substrait-io/substrait-rs/commit/d2e7ac8dad29754ab190207e9a3dd39f3deb1b1f))
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

