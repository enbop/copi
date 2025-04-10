set -e

write_version_and_data() {
    # Write the version to a file
    version=$(grep -E 'version = "[^"]+"' crates/copi-mobile-binding/Cargo.toml | head -n 1 | sed -E 's/version = "([^"]+)"/\1/')
    echo $version > android-binding/uniffi/version.txt

    # date > android-binding/uniffi/build-time.txt
}

# target build
# ./build-android.sh target arm64-v8a
if [ "$1" == "target" ]; then
    if [ "$2" == "x86" ]; then
        target_folder="i686-linux-android"
    elif [ "$2" == "x86_64" ]; then
        target_folder="x86_64-linux-android"
    elif [ "$2" == "armeabi-v7a" ]; then
        target_folder="armv7-linux-androideabi"
    elif [ "$2" == "arm64-v8a" ]; then
        target_folder="aarch64-linux-android"
    else
        echo "Unknown abi type: $2"
        exit 1
    fi

    cargo ndk \
        -t $2 \
        -o android-output/jniLibs build \
        -p copi-mobile-binding --release

    cargo run -p uniffi-bindgen generate \
        --library target/$target_folder/debug/libcopi_mobile_binding.so \
        --language kotlin \
        --out-dir android-binding/

    write_version_and_data
    exit 0
fi

# default all targets build

cargo ndk \
    -t x86 \
    -t x86_64 \
    -t armeabi-v7a \
    -t arm64-v8a \
    -o android-output/jniLibs build \
    -p copi-mobile-binding --release

cargo run -p uniffi-bindgen generate \
    --library target/i686-linux-android/release/libcopi_mobile_binding.so \
    --language kotlin \
    --out-dir android-binding/

write_version_and_data