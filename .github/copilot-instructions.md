# GitHub Copilot Instructions for CallGraphGenerator

This document contains coding guidelines and conventions for the CallGraphGenerator project to ensure consistency across all contributions.

## Language and Documentation Standards

### Code Language
- **All source code MUST be written in English only**
- Variable names, function names, struct names, enum variants, and all identifiers must use English
- No Japanese characters, Chinese characters, or any non-ASCII characters in code identifiers
- Use descriptive English names that clearly convey the purpose and functionality

### Comments and Documentation
- **All comments MUST be written in English**
- Code comments should be clear, concise, and use proper English grammar
- Rust documentation comments (`///` and `//!`) must be in English
- README files, documentation, and all markdown files should be in English
- API documentation should follow Rust documentation standards in English

### Examples

#### ✅ Correct (English)
```rust
/// Represents a call graph node containing function information
pub struct CallGraphNode {
    /// The function name
    pub function_name: String,
    /// List of functions called by this function
    pub called_functions: Vec<String>,
    /// The module path where this function is defined
    pub module_path: String,
}

/// Builds a call graph from LSP analysis results
pub fn build_call_graph(analysis_result: &AnalysisResult) -> CallGraph {
    // Process the analysis result and create nodes
    let mut graph = CallGraph::new();
    // ... implementation
}
```

#### ❌ Incorrect (Japanese/Mixed)
```rust
/// コールグラフのノードを表す構造体
pub struct CallGraphNode {
    /// 関数名
    pub kansu_mei: String,  // Wrong: Japanese romanization
    /// この関数が呼び出す関数のリスト
    pub called_functions: Vec<String>,
}

/// LSP解析結果からコールグラフを構築する
pub fn build_call_graph(kaiseki_kekka: &AnalysisResult) -> CallGraph {
    // 解析結果を処理してノードを作成 - Wrong: Japanese comment
}
```

## Coding Conventions

### Naming Conventions
- Use `snake_case` for variables, functions, and module names
- Use `PascalCase` for types, structs, enums, and traits
- Use `SCREAMING_SNAKE_CASE` for constants and static variables
- Choose descriptive names that are self-documenting

### Error Handling
- Use `Result<T, E>` for functions that can fail
- Create custom error types using `thiserror` crate when appropriate
- Provide meaningful error messages in English

### Dependencies
- Prefer well-established crates from crates.io
- Document the purpose of each dependency in `Cargo.toml`
- Keep dependencies minimal and avoid unnecessary bloat

## Development Approach

### Incremental Implementation
- **Implement features step-by-step** rather than attempting large changes at once
- Focus on one module or feature at a time
- Ensure each increment builds and passes tests before moving to the next
- Prefer working, simple implementations over complex, incomplete ones

### Implementation Strategy
- Start with basic functionality and gradually add complexity
- Implement placeholder functions that return meaningful defaults
- Add TODO comments for future enhancements
- Maintain a working build at each step

### Testing-Driven Development
- Write tests alongside implementation, not as an afterthought
- Add tests for new functionality as it's implemented
- Ensure all tests pass before committing changes
- Use placeholder tests for incomplete functionality

### Commit Frequency
- Make small, frequent commits for each logical change
- Each commit should represent a complete, working increment
- Avoid large commits that combine multiple unrelated changes
- Use descriptive commit messages that explain the increment

### Documentation Updates
- Update documentation as features are implemented
- Keep README examples in sync with actual functionality
- Add inline documentation for new public APIs
- Update architecture diagrams when structure changes

### Code Quality Maintenance
- Run `cargo clippy` and fix warnings after each change
- Ensure `cargo test` passes before each commit
- Format code with `rustfmt` regularly
- Review code for potential improvements during implementation

## LSP Integration Guidelines

### Communication Protocol
- Follow the LSP specification strictly
- Handle all LSP message types gracefully
- Implement proper error handling for LSP communication failures
- Use structured logging for debugging LSP interactions

### Data Structures
- Model LSP types accurately using serde for serialization/deserialization
- Validate incoming LSP data before processing
- Handle optional fields in LSP messages appropriately

## Output Generation

### DOT File Format
- Generate valid DOT syntax that is compatible with Graphviz
- Use clear node and edge labels in English
- Apply consistent styling for different types of nodes
- Include metadata in DOT comments when helpful

### Configuration
- Use TOML format for configuration files
- Provide sensible defaults for all configuration options
- Document all configuration parameters clearly

## Testing Standards

### Unit Tests
- Write comprehensive unit tests for all public functions
- Use descriptive test function names that explain what is being tested
- Follow the Arrange-Act-Assert pattern
- Test both success and failure cases

### Integration Tests
- Test LSP communication with mock servers
- Verify DOT output format and correctness
- Test with sample code repositories

### Documentation Tests
- Include examples in documentation that compile and run
- Use `cargo test` to verify documentation examples

## Performance Considerations

### Memory Management
- Use appropriate data structures for performance-critical paths
- Consider using `Arc` and `Rc` for shared data
- Profile memory usage for large codebases

### Concurrency
- Use async/await for I/O operations
- Consider using `tokio` for async runtime
- Handle cancellation and timeouts appropriately

## Git Commit Guidelines

### Commit Messages
- Write commit messages in English
- Use imperative mood ("Add feature" not "Added feature")
- **Prefer single-line commit messages when possible**
- Keep the first line under 50 characters
- Use concise, descriptive language that clearly explains the change
- Provide detailed explanation in the commit body only when necessary for complex changes
- Examples of good single-line commits:
  - `Add comprehensive test suite with 17 unit tests`
  - `Fix clippy warnings by deriving Default for ConfigFile`
  - `Implement basic DOT file generation with placeholder content`

### Branch Naming
- Use descriptive branch names in English
- Follow pattern: `feature/description` or `fix/description`
- Use kebab-case for branch names

## Code Review Guidelines

### Review Checklist
- Verify all code and comments are in English
- Check for proper error handling
- Ensure code follows Rust idioms and best practices
- Verify tests are comprehensive and pass
- Check documentation is complete and accurate

### Quality Standards
- Code must compile without warnings
- All tests must pass
- Code must be formatted with `rustfmt`
- Code must pass `clippy` lints without warnings

## Documentation Structure

### README Files
- Provide clear installation instructions
- Include usage examples with sample commands
- Document all command-line options
- Explain the output format and how to use it

### API Documentation
- Document all public APIs with examples
- Explain complex algorithms and data structures
- Provide troubleshooting information
- Include performance characteristics when relevant

This instruction file ensures that all contributors maintain consistency in language, style, and quality throughout the CallGraphGenerator project.
