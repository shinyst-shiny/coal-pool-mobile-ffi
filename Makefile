# Define paths and library names
TARGET_AARCH64 = target/aarch64-linux-android/release/libdrillxmobile.so
TARGET_ARMV7 = target/armv7-linux-androideabi/release/libdrillxmobile.so
TARGET_X86 = target/i686-linux-android/release/libdrillxmobile.so
TARGET_X86_64 = target/x86_64-linux-android/release/libdrillxmobile.so

JNI_LIBS_DIR = jniLibs
LIB_NAME = libuniffi_drillxmobile.so

# Default target to run
all: build copy-libs generate-kotlin

# Build the project with the given targets
build:
	cargo build --lib --release \
	    --target x86_64-linux-android \
	    --target i686-linux-android \
	    --target armv7-linux-androideabi \
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
	    generate src/drillxmobile.udl \
	    --language kotlin

# Clean up the jniLibs directory
clean:
	rm -rf $(JNI_LIBS_DIR)


