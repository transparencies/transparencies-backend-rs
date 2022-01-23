# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.0.1 (2022-01-23)

### Documentation

 - <csr-id-3f1e21edd28cfdf5ecd59ce906b957d4da5c8e43/> Better error handling for edge cases
 - <csr-id-f500df085fc8a7e9e0ca2a5fc7f0a0ac39e796ea/> Maintenance, Refactoring and Documentation

### New Features

 - <csr-id-9a9d8e61cbd47d3b13ef8960c240d84599d4bff1/> New api-client, other fixes
 - <csr-id-8410f6c221840e11f46d484b7a2d1e95c8a40f81/> Implement requesting with `match_id` as `id_type`
 - <csr-id-32caa4e17bbaac58137055294ed315c3ce3019c5/> Teamgame support
   Sort for requested players in vec
   
   Create a team for each "-1" team member
   
   Add case for team == -1 to create a team from each player with that team
 - <csr-id-29a5d01206ea5bee395669a104895666e1980f97/> Assemble match info response (Part II: Teams & Matchinfo)
 - <csr-id-fadf5d172f1d0e39186a0b8d7c42cdf85ff0fee7/> Assemble match info response (Part I: Players & Rating)
 - <csr-id-0a8e412f9f3afd61361623e36a3f731a6be49675/> Implement indexing of `alias`-list + pull translations into in-memory DB
 - <csr-id-2980a1a2ad4ccd66aeac114ad7c7602e01fd9a78/> Preparation for assembly of response towards the frontend
 - <csr-id-c27782fe98a46ed4857674ad877601bb68c25fd6/> Download datalists from github in the background and expose them to the client
 - <csr-id-c8028b54d575820985ac82f3ea5a38cc6b045893/> Implemented first draft of matchinfo endpoint

### Bug Fixes

 - <csr-id-85e274671d25f74168039ccdf12390d7520bbebd/> Small doc fixes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 394 calendar days.
 - 17 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 16 unique issues were worked on: [#1](https://github.com/transparencies/transparencies-backend-rs/issues/1), [#10](https://github.com/transparencies/transparencies-backend-rs/issues/10), [#11](https://github.com/transparencies/transparencies-backend-rs/issues/11), [#12](https://github.com/transparencies/transparencies-backend-rs/issues/12), [#13](https://github.com/transparencies/transparencies-backend-rs/issues/13), [#14](https://github.com/transparencies/transparencies-backend-rs/issues/14), [#15](https://github.com/transparencies/transparencies-backend-rs/issues/15), [#16](https://github.com/transparencies/transparencies-backend-rs/issues/16), [#17](https://github.com/transparencies/transparencies-backend-rs/issues/17), [#18](https://github.com/transparencies/transparencies-backend-rs/issues/18), [#19](https://github.com/transparencies/transparencies-backend-rs/issues/19), [#2](https://github.com/transparencies/transparencies-backend-rs/issues/2), [#20](https://github.com/transparencies/transparencies-backend-rs/issues/20), [#6](https://github.com/transparencies/transparencies-backend-rs/issues/6), [#7](https://github.com/transparencies/transparencies-backend-rs/issues/7), [#8](https://github.com/transparencies/transparencies-backend-rs/issues/8)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1](https://github.com/transparencies/transparencies-backend-rs/issues/1)**
    - Implemented first draft of matchinfo endpoint ([`c8028b5`](https://github.com/transparencies/transparencies-backend-rs/commit/c8028b54d575820985ac82f3ea5a38cc6b045893))
 * **[#10](https://github.com/transparencies/transparencies-backend-rs/issues/10)**
    - Assemble match info response (Part I: Players & Rating) ([`fadf5d1`](https://github.com/transparencies/transparencies-backend-rs/commit/fadf5d172f1d0e39186a0b8d7c42cdf85ff0fee7))
 * **[#11](https://github.com/transparencies/transparencies-backend-rs/issues/11)**
    - Assemble match info response (Part II: Teams & Matchinfo) ([`29a5d01`](https://github.com/transparencies/transparencies-backend-rs/commit/29a5d01206ea5bee395669a104895666e1980f97))
 * **[#12](https://github.com/transparencies/transparencies-backend-rs/issues/12)**
    - Maintenance, Refactoring and Documentation ([`f500df0`](https://github.com/transparencies/transparencies-backend-rs/commit/f500df085fc8a7e9e0ca2a5fc7f0a0ac39e796ea))
 * **[#13](https://github.com/transparencies/transparencies-backend-rs/issues/13)**
    - Small doc fixes ([`85e2746`](https://github.com/transparencies/transparencies-backend-rs/commit/85e274671d25f74168039ccdf12390d7520bbebd))
 * **[#14](https://github.com/transparencies/transparencies-backend-rs/issues/14)**
    - Setting up Testing ([`dfcf2be`](https://github.com/transparencies/transparencies-backend-rs/commit/dfcf2bec7f7955c81628f7c5fa0eba18a175db08))
 * **[#15](https://github.com/transparencies/transparencies-backend-rs/issues/15)**
    - Refactoring to Dashmap ([`a48f9b7`](https://github.com/transparencies/transparencies-backend-rs/commit/a48f9b7bc78b362db60427f45f2b8e3dbf90ef6e))
 * **[#16](https://github.com/transparencies/transparencies-backend-rs/issues/16)**
    - Refactoring to url::Url ([`ca3320a`](https://github.com/transparencies/transparencies-backend-rs/commit/ca3320a0094e5536874b7c5c0b23c1e75fb91784))
 * **[#17](https://github.com/transparencies/transparencies-backend-rs/issues/17)**
    - Refactoring to TestCases ([`7dae8f2`](https://github.com/transparencies/transparencies-backend-rs/commit/7dae8f254fa49ba579ce5784db3896f63edb4dfe))
 * **[#18](https://github.com/transparencies/transparencies-backend-rs/issues/18)**
    - Better error handling for edge cases ([`3f1e21e`](https://github.com/transparencies/transparencies-backend-rs/commit/3f1e21edd28cfdf5ecd59ce906b957d4da5c8e43))
 * **[#19](https://github.com/transparencies/transparencies-backend-rs/issues/19)**
    - Implement requesting with `match_id` as `id_type` ([`8410f6c`](https://github.com/transparencies/transparencies-backend-rs/commit/8410f6c221840e11f46d484b7a2d1e95c8a40f81))
 * **[#2](https://github.com/transparencies/transparencies-backend-rs/issues/2)**
    - Download datalists from github in the background and expose them to the client ([`c27782f`](https://github.com/transparencies/transparencies-backend-rs/commit/c27782fe98a46ed4857674ad877601bb68c25fd6))
 * **[#20](https://github.com/transparencies/transparencies-backend-rs/issues/20)**
    - New api-client, other fixes ([`9a9d8e6`](https://github.com/transparencies/transparencies-backend-rs/commit/9a9d8e61cbd47d3b13ef8960c240d84599d4bff1))
 * **[#6](https://github.com/transparencies/transparencies-backend-rs/issues/6)**
    - Fix clippy lints ([`380098d`](https://github.com/transparencies/transparencies-backend-rs/commit/380098df71e94af00f73e3c4aa1b2ef295e38186))
 * **[#7](https://github.com/transparencies/transparencies-backend-rs/issues/7)**
    - Preparation for assembly of response towards the frontend ([`2980a1a`](https://github.com/transparencies/transparencies-backend-rs/commit/2980a1a2ad4ccd66aeac114ad7c7602e01fd9a78))
 * **[#8](https://github.com/transparencies/transparencies-backend-rs/issues/8)**
    - Implement indexing of `alias`-list + pull translations into in-memory DB ([`0a8e412`](https://github.com/transparencies/transparencies-backend-rs/commit/0a8e412f9f3afd61361623e36a3f731a6be49675))
 * **Uncategorized**
    - Teamgame support ([`32caa4e`](https://github.com/transparencies/transparencies-backend-rs/commit/32caa4e17bbaac58137055294ed315c3ce3019c5))
    - Initial commit ([`0468128`](https://github.com/transparencies/transparencies-backend-rs/commit/046812893fe16234e5f776e64f8fcf930e54c811))
</details>

