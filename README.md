Steps:

cargo install - force cbindgen or brew install cbindgen
cbindgen - lang c - output include/munchausen.h

rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios

make macos

xcodebuild -create-xcframework \
-library libs/libmunchausen-macos.a \
-headers ./include/ \
-output Munchausen.xcframework

zip -r bundle.zip Munchausen.xcframework
openssl dgst -sha256 bundle.zip