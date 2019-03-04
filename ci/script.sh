set -euxo pipefail

main() {
    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        export RUSTFLAGS="-D warnings"
    fi

    cargo check --target $TARGET

}

main
