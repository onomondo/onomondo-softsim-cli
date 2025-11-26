# Changelog

## [0.5.0](https://github.com/onomondo/onomondo-softsim-cli/compare/v0.4.0...v0.5.0) (2025-11-26)


### Features

* make smsp tag optional ([#79](https://github.com/onomondo/onomondo-softsim-cli/issues/79)) ([dbb031e](https://github.com/onomondo/onomondo-softsim-cli/commit/dbb031ed2bb35d39b417c531189aabd280888246))


### Bug Fixes

* remove unsued DecryptedProfile struct ([#78](https://github.com/onomondo/onomondo-softsim-cli/issues/78)) ([a298a91](https://github.com/onomondo/onomondo-softsim-cli/commit/a298a91b2d576de888283df50dc228468b63e9c4))

## [0.4.0](https://github.com/onomondo/onomondo-softsim-cli/compare/v0.3.2...v0.4.0) (2025-05-08)


### Features

* enable 'raw' option for format ([#72](https://github.com/onomondo/onomondo-softsim-cli/issues/72)) ([da0e013](https://github.com/onomondo/onomondo-softsim-cli/commit/da0e0139047b34cfa6bfddc38597bb7db0cb5754))

## [0.3.2](https://github.com/onomondo/onomondo-softsim-cli/compare/v0.3.1...v0.3.2) (2023-12-08)


### Bug Fixes

* 🐛 importing keys could err out with little context ([5f03b98](https://github.com/onomondo/onomondo-softsim-cli/commit/5f03b98d9538e5afdfce9295ace1cf1b8f5728ce))

## [0.3.1](https://github.com/onomondo/onomondo-softsim-cli/compare/v0.3.0...v0.3.1) (2023-11-30)


### Bug Fixes

* 🎸 add additional logging. Some error msg weren't clear ([#42](https://github.com/onomondo/onomondo-softsim-cli/issues/42)) ([cb17a36](https://github.com/onomondo/onomondo-softsim-cli/commit/cb17a3697c16e1a5dda1fedf93a3bb5a7bc91962))

## [0.3.0](https://github.com/onomondo/onomondo-softsim-cli/compare/v0.2.1...v0.3.0) (2023-11-22)


### Features

* addional tags are now encoded. fixing some unclear silent err ([#39](https://github.com/onomondo/onomondo-softsim-cli/issues/39)) ([36cf9d6](https://github.com/onomondo/onomondo-softsim-cli/commit/36cf9d6c154f1158f0e768a83f0f2591374f571c))

## [0.2.1](https://github.com/onomondo/onomondo-softsim-cli/compare/v0.2.0...v0.2.1) (2023-10-20)


### Bug Fixes

* trigger release with updated binary ([79adf9b](https://github.com/onomondo/onomondo-softsim-cli/commit/79adf9b77d49a1e374464261d0793f00037fd998))

## [0.2.0](https://github.com/onomondo/onomondo-softsim-cli/compare/v0.1.1...v0.2.0) (2023-09-18)


### Features

* 🎸 Add proper-ish err handling ([be429ac](https://github.com/onomondo/onomondo-softsim-cli/commit/be429accf77d4de12af841defc656a0f306db94a))
* add cli version string to query param ([756e5bd](https://github.com/onomondo/onomondo-softsim-cli/commit/756e5bdac35bf319b7b8e5b5e7b6f6e3905dba97))
* add imsi and iccid encoding ([cfd693e](https://github.com/onomondo/onomondo-softsim-cli/commit/cfd693ebcfd040945e75625548ae3167086706fb))
* add support for json export format ([820e315](https://github.com/onomondo/onomondo-softsim-cli/commit/820e31598dc732550ebdca7309abad474258e6d5))
* use hex encoder ([c205568](https://github.com/onomondo/onomondo-softsim-cli/commit/c205568152b7d1032b6377e9710118904d34da41))


### Bug Fixes

* **encoder:** wrong KIC KID indicator ([a3a70e9](https://github.com/onomondo/onomondo-softsim-cli/commit/a3a70e9ee186129ab1f3c39dce51de7a8d641ead))

## [0.1.1](https://github.com/onomondo/SoftSIM-CLI/compare/v0.1.0...v0.1.1) (2023-08-31)


### Bug Fixes

* **decrypt:** use Oaep/sha1 padding to match API ([fc38734](https://github.com/onomondo/SoftSIM-CLI/commit/fc38734b62d43129a56156eccedf51468df56c04))

## 0.1.0 (2023-08-31)


### Features

* add bin target ([165fa50](https://github.com/onomondo/SoftSIM-CLI/commit/165fa50f79850bc410fbe599dbea0792f885fb84))
