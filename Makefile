# Makefile for Rust project with custom linker script

# 目标架构 (可通过 ARCH 变量指定)
ARCH ?= aarch64

# 目标名称（可执行文件）
TARGET := kernel.bin

# 架构配置映射
ifeq ($(ARCH),x86_64)
    BSP_PATH := src/bsp/qemu_x86_64
    RUST_TARGET := x86_64-unknown-linux-gnu
    QEMU_BIN := qemu-system-x86_64
    QEMU_MACHINE := pc
    QEMU_EXTRA_FLAGS := -device isa-debug-exit,iobase=0xf4,iosize=0x04
else ifeq ($(ARCH),aarch64)
    BSP_PATH := src/bsp/qemu_aarch64
    RUST_TARGET := aarch64-unknown-none
    QEMU_BIN := qemu-system-aarch64
    QEMU_MACHINE := virt
else ifeq ($(ARCH),riscv64)
    BSP_PATH := src/bsp/qemu_riscv64
    RUST_TARGET := riscv64gc-unknown-none
    QEMU_BIN := qemu-system-riscv64
    QEMU_MACHINE := virt
else
    $(error Unsupported architecture: $(ARCH). Supported: x86_64, aarch64, riscv64)
endif

# 交叉编译工具链前缀
CROSS_PREFIX_x86_64 :=
CROSS_PREFIX_aarch64 :=
CROSS_PREFIX := $(CROSS_PREFIX_$(ARCH))

# 工具链配置
CC_x86_64 := gcc
CC_aarch64 := aarch64-none-elf-gcc
CC := $(CC_$(ARCH))

LD_x86_64 := ld
LD_aarch64 := aarch64-none-elf-ld
LD := $(LD_$(ARCH))

AR_x86_64 := ar
AR_aarch64 := aarch64-none-elf-ar
AR := $(AR_$(ARCH))
CARGO := cargo
QEMU := $(QEMU_BIN)

# 链接脚本路径
LINKER_SCRIPT := $(BSP_PATH)/kernel.ld

# 构建模式：debug 或 release
BUILD_MODE ?= debug

# Rust 编译输出路径
RUST_TARGET_DIR := target/$(RUST_TARGET)/$(BUILD_MODE)

# 编译器参数更新
# Rust 需要通过 RUSTFLAGS 传递链接脚本的路径和禁用 CRT 启动文件
RUSTFLAGS := -C link-arg=-T$(LINKER_SCRIPT)  -C link-arg=-nostdlib

# 默认目标
all: $(TARGET)

# QEMU 相关配置
QEMU_FLAGS := -nographic -machine $(QEMU_MACHINE) $(QEMU_EXTRA_FLAGS) -cpu cortex-a57
QEMU_KERNEL := $(RUST_TARGET_DIR)/kernel.bin
QEMU_DEBUG_FLAGS := -s -S

# 运行内核
run: $(TARGET)
	@echo "🚀 启动 QEMU..."
	$(QEMU) $(QEMU_FLAGS) -kernel $(RUST_TARGET_DIR)/kernel

# 调试内核(启动 QEMU 并等待 GDB 连接)
debug: $(TARGET)
	@echo "🐛 启动 QEMU 调试模式..."
	@echo "   在另一个终端运行: gdb $(RUST_TARGET_DIR)/kernel"
	@echo "   然后在 gdb 中运行: target remote :1234"
	$(QEMU) $(QEMU_FLAGS) $(QEMU_DEBUG_FLAGS) -kernel $(RUST_TARGET_DIR)/kernel

# GDB 连接命令
gdb:
	@echo "📌 连接到 QEMU GDB 服务器..."
	$(CROSS_PREFIX)gdb $(RUST_TARGET_DIR)/kernel -ex "target remote :1234" -ex "break _start"

# 构建目标：使用 Cargo 编译 Rust 代码
# 这里使用 `--target` 指定目标三元组（如 thumbv7em-none-eabihf），
# 或者直接使用 rustc 手动编译（如果你想直接输出 .o 文件）。
$(TARGET): Cargo.toml src/main.rs $(LINKER_SCRIPT)
	@echo "🔧 正在使用 Cargo 编译 Rust 代码..."
	# 编译 Rust 项目
	@if [ "$(BUILD_MODE)" = "release" ]; then \
		CC="$(CC)" AR="$(AR)" RUSTFLAGS="$(RUSTFLAGS)" $(CARGO) build --release --target $(RUST_TARGET); \
	else \
		CC="$(CC)" AR="$(AR)" RUSTFLAGS="$(RUSTFLAGS)" $(CARGO) build --target $(RUST_TARGET); \
	fi
	# 转换为 raw binary 格式（QEMU aarch64 需要）
	llvm-objcopy -O binary $(RUST_TARGET_DIR)/kernel $(RUST_TARGET_DIR)/kernel.bin
	# 如果是 lib 类型，则需要手动链接
	@echo "✅ Rust 编译完成，生成可执行文件: $(RUST_TARGET_DIR)/$(TARGET)"

# 清理构建产物
clean:
	@echo "🧹 清理构建产物..."
	$(CARGO) clean
	rm -f $(TARGET)

# 帮助信息
help:
	@echo "WeNoS Kernel Build System"
	@echo ""
	@echo "Usage:"
	@echo "  make [TARGET] [ARCH=<arch>] [BUILD_MODE=<mode>]"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Build kernel (default)"
	@echo "  run          - Run kernel in QEMU"
	@echo "  debug        - Start QEMU in debug mode (wait for GDB)"
	@echo "  gdb          - Connect GDB to running QEMU instance"
	@echo "  clean        - Clean build artifacts"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "Architectures:"
	@echo "  ARCH=x86_64   - QEMU x86_64 platform (default)"
	@echo "  ARCH=aarch64  - QEMU ARM64 platform"
	@echo "  ARCH=riscv64  - QEMU RISC-V 64-bit platform"
	@echo ""
	@echo "Build Modes:"
	@echo "  BUILD_MODE=debug   - Debug build (default)"
	@echo "  BUILD_MODE=release - Release build"
	@echo ""
	@echo "Examples:"
	@echo "  make                          # Build x86_64 debug kernel"
	@echo "  make run                      # Run x86_64 kernel in QEMU"
	@echo "  make ARCH=aarch64 run    # Run ARM64 kernel"
	@echo "  make BUILD_MODE=release run   # Run release build"

.PHONY: all clean run debug gdb help
