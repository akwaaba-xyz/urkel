# <h1> Urkel 👨🏾‍🔬 🦀 </h1>

<p><strong>A gRPC client and HTTP wrapping server for Open FGA, built in Rust.</strong></p>

[![dependency status](https://deps.rs/repo/github/akwaaba-xyz/urkel/status.svg?style=flat-square)](https://deps.rs/repo/github/akwaaba-xyz/urkel) [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

| “Security is about how you configure power, who has access to what? That is political.” - Dug Song, Co-Founder of Duo |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Urkel is an opinionated permissions engine for expressing what users and objects can do based on contextual relationships. It presumes that relationships between things and people is the most expressive and natural way to describe how power flows within a system. |
| Leveraging fine-grained authorization, Urkel seeks to address the access control needs of typically underserved, non-hierarchical organizations such as DAOs, non-profits, coops, mutual aid groups and more. |


## Features

-   [x] OpenFGA gRPC client
-   [x] HTTP server for serializing and deserializing JSON
-   [x] Check permissions in bulk
-   [x] n-of-m authorization schemes
-   [x] Horizontal permissions check 
-   [x] Read list of permissions without pagination
-   [x] API-token security
-   [ ] Token-gated permissions checks
-   [ ] Frontend-only Authorization (FOAz) with zKP
-   [ ] Configurable authentication options

## User Warning

This project comes as is. We provide no guarantee of stability or support, as the crates closely follow the needs of the [`Papertree`](https://papertree.earth/) project.

If you use this project in a production environment, it is your responsibility to perform a security audit to ensure that the software meets your requirements.


## Credits

This project would not have been possible without the great work done in:

-   [`openfga`](https://github.com/openfga/openfga/)
-   [`modpol`](https://gitlab.com/medlabboulder/modpol/)
-   [`safe`](https://github.com/safe-global/safe-contracts/)

A lot of the code was inspired and adapted from them, to a unified and opinionated interface,
built with async/await, std futures, and gRPC from the ground up.

## Contributing

Check [CONTRIBUTING.md](CONTRIBUTING.md) if you are interested in contributing to this project.
