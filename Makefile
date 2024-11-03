# Define paths and library names
TARGET_AARCH64 = target/aarch64-linux-android/release/libcoalpoolmobileffi.so
TARGET_ARMV7 = target/armv7-linux-androideabi/release/libcoalpoolmobileffi.so
TARGET_X86 = target/i686-linux-android/release/libcoalpoolmobileffi.so
TARGET_X86_64 = target/x86_64-linux-android/release/libcoalpoolmobileffi.so

JNI_LIBS_DIR = jniLibs
LIB_NAME = libuniffi_coalpoolmobileffi.so

# Default target to run
all: build-x86_64 build-i686 build-armv7 build-aarch64 copy-libs generate-kotlin

# Build the project with the given targets
build-x86_64:
	CROSS_BUILD_OPTS="--output=type=docker" cross build --lib --release \
	    --target x86_64-linux-android

build-i686:
	CROSS_BUILD_OPTS="--output=type=docker" cross build --lib --release \
	    --target i686-linux-android

build-armv7:
	CROSS_BUILD_OPTS="--output=type=docker" cross build --lib --release \
	    --target armv7-linux-androideabi

build-aarch64:
	CROSS_BUILD_OPTS="--output=type=docker" cross build --lib --release \
	    --target aarch64-linux-android

# Copy .so files to appropriate directories
copy-libs: $(TARGET_AARCH64) $(TARGET_ARMV7) $(TARGET_X86) $(TARGET_X86_64)
	@echo "Copying libraries..."
	mkdir -p $(JNI_LIBS_DIR)/arm64-v8a/ && \
	  cp $(TARGET_AARCH64) $(JNI_LIBS_DIR)/arm64-v8a/$(LIB_NAME)
	mkdir -p $(JNI_LIBS_DIR)/armeabi-v7a/ && \
	  cp $(TARGET_ARMV7) $(JNI_LIBS_DIR)/armeabi-v7a/$(LIB_NAME)
	mkdir -p $(JNI_LIBS_DIR)/x86/ && \
	  cp $(TARGET_X86) $(JNI_LIBS_DIR)/x86/$(LIB_NAME)
	mkdir -p $(JNI_LIBS_DIR)/x86_64/ && \
	  cp $(TARGET_X86_64) $(JNI_LIBS_DIR)/x86_64/$(LIB_NAME)

# Generate Kotlin file from the UDL
generate-kotlin:
	cargo run --features=uniffi/cli \
	    --bin uniffi-bindgen \
	    generate src/coalpoolmobileffi.udl \
	    --language kotlin

# Clean up the jniLibs directory
clean:
	rm -rf $(JNI_LIBS_DIR)


