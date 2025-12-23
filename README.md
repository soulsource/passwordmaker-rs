# MOVED

This repo is no longer being updated. You can find the current version at [https://git.grois.info/passwordmaker-rs/](https://git.grois.info/passwordmaker-rs/).

# Description

This project is a Rust reimplementation of [PasswordMaker Pro](https://passwordmaker.org).

Just like the original software, this library is released under the GNU Lesser General Public License. To be precise, under [GNU Lesser General Public License v3.0 or later](https://spdx.org/licenses/LGPL-3.0-or-later.html). See the `LICENSE` file for the full text of the license.

This is a completely new implementation, but the source code of the JavaScript Edition of Passwordmaker Pro was used as a guideline whenever something was not immediately clear from the behaviour of the original program.

All credit for the development of the PasswordMaker Pro algorithm (and therefore for the high level flow of this library too) goes to the original authors of PasswordMaker Pro, [Miquel Burns](https://github.com/miquelfire) and [Eric H. Jung](https://github.com/ericjung). (I really hope I linked the correct profiles.)

This crate is meant as a building block for an upcoming native Sailfish OS app ("[PassFish](https://github.com/soulsource/passfish)") that aims to be compatible with PasswordMaker Pro, but the public API should be reasonably easy to use for other Rust-based PasswordMaker compatible tools, so feel free to base your own applications on this.

Beware that currently this library is developed in tandem with the corresponding Sailfish app, so the interface might still change as needed. This will of course stop once this library reaches version 1.0.

This library does not include any cryptographic hashes itself, but rather relies on the user to supply implementations of the hash algorithms. You can have a look at the integration tests to see how the syntax looks like for the [RustCrypto Hashes](https://github.com/RustCrypto/hashes). This is to avoid duplicate code, as many GUI frameworks already include their own implementation of the hashes.
