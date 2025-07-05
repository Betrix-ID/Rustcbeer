TARGET      := arm64
CARGO       := cargo
BUILD_DIR   := $(HOME)/live/target/release
DEBUG_DIR   := $(HOME)/live/target/debug
BIN         := $(BUILD_DIR)/$(TARGET)
PER         := chmod +x
CMP         := upx --best --lzma

# Default entrypoint
all: banner build

# Compile & optimize
build:
	@echo ">> 🛠️  Compiling Rust program..."
	@$(CARGO) build --release
	@$(PER) $(BIN)
	@$(CMP) $(BIN)
	@echo ">> Cleaning up build directories..."
	@find $(BUILD_DIR) -mindepth 1 ! -name '$(TARGET)' -exec rm -rf {} +
	@find $(DEBUG_DIR) -mindepth 1 ! -name '$(TARGET)' -exec rm -rf {} +
	@echo "Build finished. Only binary '$(TARGET)' retained."

# Run the binary
run:
	@echo ">> Running $(TARGET)..."
	@$(BIN)

# Informational banner
banner:
	@echo
	@echo "     ☆=====================================☆"
	@echo "      ~ Rust Binary Compilation System"
	@echo
	@echo "      - Author      :  @UnixeID"
	@echo "      - Version     :  1.0"
	@echo "      - Release     :  05 - Jul - 2025"
	@echo "      - Project     :  RustcBeer WebUI"
	@echo
	@echo "    |_______________________________________|"
	@echo "     \\______________________________________/"
	@echo
	@sleep 1

# Clean everything except the binary
clean:
	@echo ">> Deep cleaning build artifacts..."
	@find $(BUILD_DIR) -mindepth 1 ! -name '$(TARGET)' -exec rm -rf {} +
	@find $(DEBUG_DIR) -mindepth 1 ! -name '$(TARGET)' -exec rm -rf {} +
	@echo "Clean complete."

# Format code using rustfmt
format:
	@echo ">> Formatting Rust source code..."
	@$(CARGO) fmt
	@echo "Source code formatted."