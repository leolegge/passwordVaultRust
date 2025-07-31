This is a basic password vault used to hold passwords using a CLI

# Requirements
- Install [Rust](https://rustup.rs/) (includes `cargo` and `rustc`)
- Build tools:
  - **Linux:** Install `build-essential` (e.g., `sudo apt install build-essential`)
  - **Windows:** Install Visual Studio Build Tools (select "Desktop development with C++")
  - **macOS:** Install Xcode command line tools (`xcode-select --install`)

# To compile and run the current build
```
git clone https://github.com/leolegge/passwordVaultRust.git
cd passwordVaultRust
cargo run

```

# Features
- Ability to create multiple vaults which can be accessed with passphrases
- Ability to delete vaults
- Ability to add and delete entries into vaults
- Saving of vaults
