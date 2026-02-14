# Testing Guide

This guide covers testing practices for Doctainr.

## Overview

Doctainr uses Rust's built-in testing framework along with:
- **tokio**: For async test support
- **bollard**: Real Docker API for integration tests
- **cargo test**: Test runner

## Test Types

### Unit Tests

Test individual functions and methods in isolation.

**Location**: Same file as code, in `#[cfg(test)]` module

```rust
// In src/services/docker.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_labels() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }

    #[test]
    fn test_container_state_css_classes() {
        assert_eq!(ContainerState::Running.css_class(), "running");
        assert_eq!(ContainerState::Stopped.css_class(), "stopped");
    }
}
```

### Integration Tests

Test interactions between components and with external systems (Docker).

**Location**: `tests/` directory at project root

```rust
// tests/docker_integration.rs

use doctainr::services::DockerService;

#[tokio::test]
async fn test_docker_connection() {
    let service = DockerService::new()
        .expect("Docker must be running for integration tests");
    
    // Test actual Docker operations
    let containers = service.list_containers().await;
    assert!(containers.is_ok());
}

#[tokio::test]
async fn test_list_images() {
    let service = DockerService::new().expect("Docker required");
    let images = service.list_images().await;
    assert!(images.is_ok());
}
```

### Async Tests

Use `#[tokio::test]` for async functions:

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

## Running Tests

### All Tests

```bash
cargo test
```

### Specific Test

```bash
# By name
cargo test test_container_state

# By module
cargo test services::docker::tests

# By pattern
cargo test docker
```

### With Output

```bash
# Show println! output
cargo test -- --nocapture

# Show only failed test output
cargo test -- --show-output
```

### Integration Tests Only

```bash
cargo test --test '*'
```

### Doc Tests

```bash
cargo test --doc
```

### Single Threaded (for Docker tests)

```bash
# Run tests sequentially (useful for Docker state conflicts)
cargo test -- --test-threads=1
```

## Writing Tests

### Test Structure (AAA Pattern)

```rust
#[test]
fn test_example() {
    // Arrange: Set up test data
    let container_state = ContainerState::Running;
    
    // Act: Execute the operation
    let label = container_state.label();
    
    // Assert: Verify the result
    assert_eq!(label, "Running");
}
```

### Assertions

```rust
// Equality
assert_eq!(actual, expected);
assert_ne!(actual, not_expected);

// Boolean conditions
assert!(value);
assert!(!value);

// Custom message
assert_eq!(actual, expected, "Values should match: {}", reason);

// Panic assertions
#[should_panic]
#[test]
fn test_panic() {
    panic!("Expected panic");
}

// With expected message
#[should_panic(expected = "Invalid input")]
#[test]
fn test_panic_with_message() {
    panic!("Invalid input provided");
}
```

### Testing Results

```rust
#[test]
fn test_result() -> Result<(), Box<dyn std::error::Error>> {
    let value = fallible_operation()?;
    assert_eq!(value, expected);
    Ok(())
}

#[tokio::test]
async fn test_async_result() -> Result<(), Box<dyn std::error::Error>> {
    let value = async_operation().await?;
    assert_eq!(value, expected);
    Ok(())
}
```

## Testing Docker Integration

### Prerequisites

Integration tests require Docker to be running:

```bash
# Verify Docker is running
docker info

# Then run integration tests
cargo test --test '*'
```

### Handling Docker State

Docker tests may conflict if run in parallel. Use setup/teardown:

```rust
#[tokio::test]
async fn test_with_cleanup() {
    let service = DockerService::new().unwrap();
    
    // Setup: Create test container
    // (use Docker CLI or Bollard directly)
    
    // Test operations
    let result = service.list_containers().await;
    assert!(result.is_ok());
    
    // Teardown: Clean up test container
    // (remove test resources)
}
```

### Ignoring Tests Without Docker

```rust
#[tokio::test]
#[ignore]  // Skip unless --ignored flag used
async fn test_requires_docker() {
    let service = DockerService::new().expect("Docker required");
    // Test that needs Docker
}
```

Run ignored tests:
```bash
cargo test -- --ignored
```

## Mocking and Test Doubles

### Trait-Based Mocking

For testing without Docker, define traits:

```rust
// Define trait for Docker operations
pub trait DockerClient {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>>;
    async fn start_container(&self, id: &str) -> Result<()>;
}

// Real implementation
impl DockerClient for DockerService {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
        // Real Docker API call
    }
}

// Mock for testing
#[cfg(test)]
struct MockDockerClient {
    containers: Vec<ContainerInfo>,
}

#[cfg(test)]
impl DockerClient for MockDockerClient {
    async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
        Ok(self.containers.clone())
    }
}
```

### Test Data Builders

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;
    
    pub fn create_test_container(name: &str, state: ContainerState) -> ContainerInfo {
        ContainerInfo {
            id: "test_id".to_string(),
            name: name.to_string(),
            image: "test:latest".to_string(),
            status: state.label().to_string(),
            ports: "8080→80".to_string(),
            state,
        }
    }
    
    pub fn create_running_container(name: &str) -> ContainerInfo {
        create_test_container(name, ContainerState::Running)
    }
}

#[test]
fn test_with_helper() {
    let container = test_helpers::create_running_container("test");
    assert_eq!(container.state, ContainerState::Running);
}
```

## Testing Components

### Component Logic Tests

Test component logic without UI:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_logic() {
        // Test pure functions used by components
        let formatted = format_container_status("Up 2 hours");
        assert_eq!(formatted, "Running");
    }
}
```

### State Management Tests

Test state updates:

```rust
#[test]
fn test_state_update() {
    // Test AppState methods
    let containers = vec![create_test_container()];
    // Verify state changes correctly
}
```

## Test Organization

### File Structure

```
src/
  services/
    docker.rs          # Contains unit tests
tests/
  docker_integration.rs  # Integration tests
  common/
    mod.rs            # Shared test utilities
```

### Shared Test Utilities

```rust
// tests/common/mod.rs
pub fn setup_docker() -> DockerService {
    DockerService::new().expect("Docker required for tests")
}

pub fn create_test_container() -> ContainerInfo {
    // Test data creation
}

// In test file
mod common;

#[tokio::test]
async fn test_something() {
    let service = common::setup_docker();
    // Use service
}
```

## Code Coverage

### Using Tarpaulin

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Open report
open tarpaulin-report.html
```

### Coverage Goals

Aim for:
- **80%+** overall coverage
- **100%** for critical paths (Docker operations)
- **90%+** for business logic

## Continuous Integration

Tests run automatically on:
- Pull requests
- Pushes to main branch
- Scheduled builds

CI configuration: `.github/workflows/ci.yaml`

## Performance Testing

### Benchmarks

```rust
// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_format_size(c: &mut Criterion) {
    c.bench_function("format_size", |b| {
        b.iter(|| format_size(black_box(1_048_576)))
    });
}

criterion_group!(benches, benchmark_format_size);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

## Best Practices

### 1. Test Names Should Be Descriptive

```rust
// Good
#[test]
fn test_container_state_running_returns_correct_label() { }

// Less clear
#[test]
fn test_label() { }
```

### 2. One Assertion Per Test (When Possible)

```rust
// Good: Focused test
#[test]
fn test_running_label() {
    assert_eq!(ContainerState::Running.label(), "Running");
}

#[test]
fn test_stopped_label() {
    assert_eq!(ContainerState::Stopped.label(), "Stopped");
}

// Less ideal: Multiple assertions
#[test]
fn test_all_labels() {
    assert_eq!(ContainerState::Running.label(), "Running");
    assert_eq!(ContainerState::Stopped.label(), "Stopped");
    // If first fails, second never runs
}
```

### 3. Test Edge Cases

```rust
#[test]
fn test_empty_container_list() {
    let containers = vec![];
    assert_eq!(count_running(&containers), 0);
}

#[test]
fn test_all_stopped_containers() {
    let containers = vec![stopped(), stopped()];
    assert_eq!(count_running(&containers), 0);
}
```

### 4. Keep Tests Fast

```rust
// Avoid
#[tokio::test]
async fn slow_test() {
    tokio::time::sleep(Duration::from_secs(10)).await;  // Too slow!
}

// Prefer
#[test]
fn fast_test() {
    // Test logic without delays
}
```

### 5. Isolate Tests

Each test should be independent:

```rust
// Bad: Tests depend on order
#[test]
fn test_part_1() {
    // Modifies shared state
}

#[test]
fn test_part_2() {
    // Depends on part_1 running first
}

// Good: Independent tests
#[test]
fn test_feature_a() {
    // Set up its own state
}

#[test]
fn test_feature_b() {
    // Set up its own state
}
```

## Debugging Tests

### Print Debugging

```rust
#[test]
fn test_debug() {
    let value = compute();
    println!("Value: {:?}", value);  // Won't show unless test fails or --nocapture
    assert_eq!(value, expected);
}
```

### Running Single Test

```bash
# Run and see output
cargo test test_name -- --nocapture

# With debug logging
RUST_LOG=debug cargo test test_name -- --nocapture
```

### Using Debugger

```bash
# Build tests with debug symbols
cargo test --no-run

# Find test binary
ls -la target/debug/deps/doctainr-*

# Run with debugger
rust-gdb target/debug/deps/doctainr-<hash>
```

## Test Documentation

Document complex test scenarios:

```rust
/// Tests that starting an already running container returns an error.
///
/// This verifies that the Docker service properly handles the edge case
/// where a user attempts to start a container that's already running.
#[tokio::test]
async fn test_start_already_running_container() {
    // Test implementation
}
```

## Common Testing Patterns

### Testing Error Cases

```rust
#[tokio::test]
async fn test_error_handling() {
    let result = operation_that_fails().await;
    assert!(result.is_err());
    
    // Check specific error
    if let Err(e) = result {
        assert!(e.to_string().contains("expected error message"));
    }
}
```

### Testing With Timeouts

```rust
#[tokio::test]
async fn test_with_timeout() {
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        slow_operation()
    ).await;
    
    assert!(result.is_ok(), "Operation timed out");
}
```

## Related Documentation

- [Development Guide](development.md)
- [Contributing Guidelines](contributing.md)
- [Code Style Guide](code-style.md)
- [Architecture Overview](../architecture/overview.md)
