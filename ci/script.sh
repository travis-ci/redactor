set -ex

main() {
    cross build --target $TARGET

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cross test --target $TARGET
}

if [ -z $TRAVIS_TAG ]; then
    main
fi
