# Hardware Module

This module provides hardware information gathering functionality with platform-specific implementations for better code organization and maintainability.

## Architecture

The hardware module is organized into several components:

### Core Components

- **`traits.rs`** - Defines the `HardwareProvider` trait that serves as a common interface for all hardware operations
- **`base.rs`** - Base hardware implementation containing common functionality shared across platforms
- **`mod.rs`** - Main module file that selects the appropriate platform implementation and provides the public API
- **`macos.rs`** - macOS-specific hardware implementation (extends base functionality)
- **`linux.rs`** - Linux-specific hardware implementation (extends base functionality)

### Structure

```
src/drivers/hardware/
├── README.md           # This documentation
├── mod.rs             # Main module with platform selection
├── traits.rs          # Common interface definitions
├── base.rs            # Base implementation with common functionality
├── macos.rs           # macOS-specific implementation
└── linux.rs           # Linux-specific implementation
```

## Design Patterns

### Platform Selection

The module uses Rust's conditional compilation attributes (`#[cfg(target_os = "...")]`) to automatically select the appropriate implementation based on the target operating system:

- **macOS**: Uses `MacOSHardware` struct
- **Linux**: Uses `LinuxHardware` struct

### Trait-Based Architecture with Base Class Pattern

The module uses a combination of inheritance-like patterns and trait-based architecture:

- **`BaseHardware`** - Contains common implementations for hardware operations that work the same across platforms
- **`HardwareProvider`** - Main trait that defines the public API for hardware information
- **`PlatformSpecificHardware`** - Trait for platform-specific operations that differ between operating systems

All platform implementations extend the base functionality and implement platform-specific methods, ensuring consistent APIs while eliminating code duplication.

### Code Organization and DRY Principle

The module follows the DRY (Don't Repeat Yourself) principle by extracting common functionality:

- **Base Implementation**: Contains shared methods like `cpu_model()`, `memory_info()`, `battery_info()`, etc.
- **Platform-Specific Methods**: Only the methods that differ between platforms are implemented separately:
  - `device_serial()` - Different detection methods per platform
  - `manufactured_date()` - Different date parsing logic
  - `system_manufacturer()` - Linux reads from DMI, macOS returns "Apple Inc."
  - `system_product_name()` - Different system information sources

**Platform-specific tools:**
- **macOS Implementation**: Uses `system_profiler`, `ioreg`, and `sysctl`
- **Linux Implementation**: Uses DMI files (`/sys/class/dmi/id/`), `/proc/` filesystem, and `dmidecode`

## Supported Hardware Information

The module provides the following hardware information:

- **CPU**: Model, cores, cache sizes (L1, L2, L3)
- **Memory**: Total and free RAM
- **Storage**: Disk size and available space
- **Network**: MAC address, local IP, external IP
- **System**: Hostname, manufacturer, product name, manufacturing date
- **Device**: Serial number
- **Power**: Battery information (if available)

## Usage

### Basic Usage

```rust
use crate::drivers::hardware::Hardware;

// Get CPU information
let cpu_model = Hardware::cpu_model();
let cpu_cores = Hardware::cpu_cores();

// Get memory information
let (total_mem, free_mem) = Hardware::memory_info();

// Check if system has battery
let has_battery = Hardware::has_battery();
```

### Legacy API Support

The module maintains backward compatibility with the original function-based API:

```rust
use crate::drivers::hardware::*;

let cpu_model = get_cpu_model();
let cpu_cores = get_cpu_num();
let hostname = get_host_name();
```

## Platform-Specific Features

### macOS Features

- **Serial Number Decoding**: Decodes Apple's manufacturing date from device serial numbers
- **System Profiler Integration**: Uses `system_profiler` for detailed hardware information
- **IORegistry Access**: Fallback to `ioreg` for hardware details

### Linux Features

- **DMI Support**: Reads hardware information from `/sys/class/dmi/id/` files
- **Multiple Fallbacks**: Uses various sources for hardware information (DMI, /proc, dmidecode)
- **Date Parsing**: Attempts to extract manufacturing dates from BIOS information and serial numbers

## Error Handling

The module uses graceful error handling with fallbacks:

- Returns "Unknown" for unavailable information
- Provides multiple detection methods with fallbacks
- Handles missing system tools gracefully

## Extension

To add support for a new platform:

1. Create a new file (e.g., `windows.rs`)
2. Implement both `HardwareProvider` and `PlatformSpecificHardware` traits
3. Use `BaseHardware` methods for common functionality
4. Add conditional compilation in `mod.rs`
5. Update the platform selection logic

Example for Windows support:

```rust
// In windows.rs
use super::base::{BaseHardware, PlatformSpecificHardware};
use super::traits::{constants::UNKNOWN_VALUE, HardwareProvider};

pub struct WindowsHardware;

impl HardwareProvider for WindowsHardware {
    fn cpu_model() -> String {
        BaseHardware::cpu_model()  // Use base implementation
    }
    
    fn device_serial() -> String {
        <Self as PlatformSpecificHardware>::device_serial()  // Use platform-specific
    }
    
    // ... other methods
}

impl PlatformSpecificHardware for WindowsHardware {
    fn device_serial() -> String {
        // Windows-specific serial number detection
        // Implementation here...
    }
    
    // ... other platform-specific methods
}

// In mod.rs
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
use windows::WindowsHardware as PlatformHardware;
```

## Dependencies

The module uses the following external crates:

- `battery` - Battery information
- `sysinfo` - Cross-platform system information
- `sys-info` - System information utilities
- `mac_address` - MAC address detection
- `local-ip-address` - Local IP detection
- `cache-size` - CPU cache information
- `mid` - Device identification
- `reqwest` - External IP detection

## Testing

To test the module on your platform:

```bash
cargo run
```

The application will display comprehensive hardware information using the appropriate platform implementation.