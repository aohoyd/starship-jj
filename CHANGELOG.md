# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

## [0.4.0](https://gitlab.com/lanastara_foss/starship-jj/-/compare/0.3.2..0.4.0) - 2025-06-19

### 🚀 Features

- add intel darwin to list of systems- ([6bb1511](https://gitlab.com/lanastara_foss/starship-jj/-/commit/6bb151167f8277ae29793d67aebe1ee675075d6e)) - Joachim Desroches
- flake: use fenix, and nix-systems, add overlay.- ([!14](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/14)) - Joachim Desroches
- update to jj 0.30- ([1711b37](https://gitlab.com/lanastara_foss/starship-jj/-/commit/1711b37b437c7ff3b704450afa448c3bdff47d68)) - Lilly Mannhal
- symbol module- ([43250e4](https://gitlab.com/lanastara_foss/starship-jj/-/commit/43250e43f8ac4a58f5c19d1db34908fac4afed33)) - Lilly Mannhal

### 🐛 Bug Fixes

- fix flake.nix- ([!12](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/12)) - Lilly Mannhal
- typo in nixpkgs input of the flake.nix- ([4a345ee](https://gitlab.com/lanastara_foss/starship-jj/-/commit/4a345ee6ea60f2830157e6a2c961f50d4c31d29b)) - Joachim Desroches
- Update `jj` to `0.29.0` and fix macos config dir- ([!15](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/15)) - Lucio Franco
-  [**breaking**]default colors fixed- ([a5895a4](https://gitlab.com/lanastara_foss/starship-jj/-/commit/a5895a446ebc47c40adc69a57859d9adb9be3547)) - Lilly Mannhal

### 📚 Documentation

- added contributing file- ([!16](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/16)) - Lilly Mannhal
- updated sample configuration- ([!17](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/17)) - Lilly Mannhal

### ⚙️ Miscellaneous Tasks

- crossbeam-channel update (RUSTSEC-2025-024).- ([!13](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/13)) - Joachim Desroches
## [0.3.2](https://gitlab.com/lanastara_foss/starship-jj/-/compare/0.3.1..0.3.2) - 2025-04-06

### 🐛 Bug Fixes

- 'sane' default for search_depth- ([cc17af0](https://gitlab.com/lanastara_foss/starship-jj/-/commit/cc17af0d4dbd835c56669d665c77e8020ecc06d8)) - Lilly Mannhal

### ⚙️ Miscellaneous Tasks

- **(release)** update jj deps- ([9834968](https://gitlab.com/lanastara_foss/starship-jj/-/commit/9834968dbc3b3611a85236173665875da0cf9ce8)) - Lilly Mannhal
- **(release)** prepare 0.3.2- ([!11](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/11)) - Lilly Mannhal
- updated pollster- ([9a33f7a](https://gitlab.com/lanastara_foss/starship-jj/-/commit/9a33f7a853afa7175833386a7689a0039a40446f)) - Lilly Mannhal
- added nix flake- ([652d0f7](https://gitlab.com/lanastara_foss/starship-jj/-/commit/652d0f76e0c447f07b21640714c25c1ed8cdd7a5)) - Lilly Mannhal
## [0.3.1](https://gitlab.com/lanastara_foss/starship-jj/-/compare/v0.3.0..0.3.1) - 2025-03-16

### 🐛 Bug Fixes

- **(env)** error when no .env file exists- ([105cf69](https://gitlab.com/lanastara_foss/starship-jj/-/commit/105cf69c8ae638e57b9fa7480681999fdf69022c)) - Lilly Mannhal
## [0.3.0](https://gitlab.com/lanastara_foss/starship-jj/-/compare/v0.2.0..v0.3.0) - 2025-03-15

### 🚀 Features

- **(ci)** add cargo deny check- ([!5](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/5)) - Lilly Mannhal
- add immutable and empty state- ([ad98020](https://gitlab.com/lanastara_foss/starship-jj/-/commit/ad98020a6c9b6132f7783c87ac1bb4cd8fdbd1a4)) - Lilly Mannhal
- truncate bookmark names- ([f9a6e5c](https://gitlab.com/lanastara_foss/starship-jj/-/commit/f9a6e5c9e33f27b9597962d9bfba03ac827e1ce8)) - Lilly Mannhal
- time runtime- ([a5777b6](https://gitlab.com/lanastara_foss/starship-jj/-/commit/a5777b6a6dd30456fd98e5309ddbec70edebfec8)) - Lilly Mannhal
- actually react to --ignore-working-copy- ([36e486f](https://gitlab.com/lanastara_foss/starship-jj/-/commit/36e486f8c03139807d162af4d9606c950bf645c4)) - Lilly Mannhal
- only load data when required by a module- ([8635357](https://gitlab.com/lanastara_foss/starship-jj/-/commit/8635357c45249dba77bfb8e81d80eb1607a8dca9)) - Lilly Mannhal
- global timeout- ([!4](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/4)) - Lilly Mannhal
- parse .env to overwrite config per repo- ([!7](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/7)) - Lilly Mannhal

### 🐛 Bug Fixes

- 🦀🦀truncating non ascii strings- ([!6](https://gitlab.com/lanastara_foss/starship-jj/-/merge_requests/6)) - Lilly Mannhal

### 📚 Documentation

- **(config)** update schema.json- ([06d705a](https://gitlab.com/lanastara_foss/starship-jj/-/commit/06d705af1eb26d55e84da928abae1d37ae23cc15)) - Lilly Mannhal

### ⚙️ Miscellaneous Tasks

- Move to rust 2024- ([8becc34](https://gitlab.com/lanastara_foss/starship-jj/-/commit/8becc3440a7ebca11c8406657a6e6a0e7fd9ae8d)) - Lilly Mannhal
- move git repo- ([382f11c](https://gitlab.com/lanastara_foss/starship-jj/-/commit/382f11c8798e145bfaad45548dc71fbabfbbef3e)) - Lilly Mannhal
## [0.2.0](https://gitlab.com/lanastara_foss/starship-jj/-/compare/v0.1.0..v0.2.0) - 2025-02-27

### 🚀 Features

- use --starship-config argument- ([4145e01](https://gitlab.com/lanastara_foss/starship-jj/-/commit/4145e014505c046b25672bf48fb1718d00047a2d)) - Lilly Mannhal

### 📚 Documentation

- update readme- ([dd2fbdc](https://gitlab.com/lanastara_foss/starship-jj/-/commit/dd2fbdc2a7449029bbbed175df3ca881bd0da70b)) - Lilly Mannhal
## [0.1.0] - 2025-02-26

### 🚀 Features

- add commit_diff- ([3d96a4d](https://gitlab.com/lanastara_foss/starship-jj/-/commit/3d96a4d6f20134118983c51daf4a30cc7fa851c9)) - Lilly Mannhal
- better default values- ([780d985](https://gitlab.com/lanastara_foss/starship-jj/-/commit/780d985d6a653cd8d42d571696de7a35c0ad1e8f)) - Lilly Mannhal
- change branch management- ([4d95c10](https://gitlab.com/lanastara_foss/starship-jj/-/commit/4d95c10182c20b1aeedfa371b1792c255550800f)) - Lilly Mannhal
- order and filter bookmarks- ([1d0d3eb](https://gitlab.com/lanastara_foss/starship-jj/-/commit/1d0d3eb23ab5c8888c54413f2d53beaa64d1b098)) - Lilly Mannhal
- config path command- ([a50b7d0](https://gitlab.com/lanastara_foss/starship-jj/-/commit/a50b7d00111388e047106a6e2b6330b66ae4a979)) - Lilly Mannhal
- config default command- ([fbcbada](https://gitlab.com/lanastara_foss/starship-jj/-/commit/fbcbadafa81d29684b0f171e6010b8b6bbd187e5)) - Lilly Mannhal
- read and parse config- ([f791f48](https://gitlab.com/lanastara_foss/starship-jj/-/commit/f791f48a8740119b38930ca928b494810cbfeaee)) - Lilly Mannhal
- custom separator between modules- ([ae0d2fc](https://gitlab.com/lanastara_foss/starship-jj/-/commit/ae0d2fc292fd430bea862cb57737ab37a94bf2ee)) - Lilly Mannhal

### 🐛 Bug Fixes

- color printing- ([ac69fe3](https://gitlab.com/lanastara_foss/starship-jj/-/commit/ac69fe3d9909933cb722a8c043182f7eb9bb78aa)) - Lilly Mannhal
- only 5 tags are allowed- ([df5517e](https://gitlab.com/lanastara_foss/starship-jj/-/commit/df5517e82eefb5ae263f8dd06afc847b24b87c2b)) - Lilly Mannhal

### 📚 Documentation

- create json schema for config- ([f074121](https://gitlab.com/lanastara_foss/starship-jj/-/commit/f074121900b447eea302fc5f4119d4d00fb3ee3d)) - Lilly Mannhal
- added sample config- ([eb64656](https://gitlab.com/lanastara_foss/starship-jj/-/commit/eb64656dffc5f134fdfbc20f42579a8f29f802d8)) - Lilly Mannhal

### ⚙️ Miscellaneous Tasks

- fix Cargo.toml for publishing- ([5be560c](https://gitlab.com/lanastara_foss/starship-jj/-/commit/5be560cbc190aba58c7a358e3290324e06c4ea79)) - Lilly Mannhal

