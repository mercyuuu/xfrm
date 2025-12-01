# Contributing to xfrm

<details>
   <summary>thank for your interest in contributing to xfrm! this document pretty much just shows you some guidelines for contributing. </summary>
   "formal" typing whatever below!!!!!
</details>

## Development Setup

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository:**
   ```bash
   git clone https://github.com/mercyuuu/xfrm.git
   cd xfrm
   ```

3. **Install dependencies (for testing):**
   ```bash
   # Ubuntu/Debian
   sudo apt install ffmpeg imagemagick pandoc
   
   # macOS
   brew install ffmpeg imagemagick pandoc
   ```

4. **Build and test:**
   ```bash
   cargo build
   cargo test
   cargo run -- --help
   ```

## Project Structure

```
xfrm/
├── src/
│   ├── main.rs         # cli interface and main entry point
│   ├── converter.rs    # core conversion logic
│   └── formats.rs      # format detection and validation
├── Cargo.toml          
├── README.md           
├── QUICKSTART.md       
└── install.sh          # script for installation
```

## Adding New Features

### Adding a New Format

1. Add the format to the appropriate list in `src/formats.rs`:
   ```rust
   const VIDEO_FORMATS: &[&str] = &[
       "mp4", "avi", "mkv", "your_format",
   ];
   ```

2. If needed, add special handling in `src/converter.rs`

3. Update the README.md with the new format

4. Add tests if applicable

### Adding New CLI Options

1. Add the option to the `Cli` struct in `src/main.rs`:
   ```rust
   #[derive(Parser)]
   struct Cli {
       // ... existing fields ...
       
       /// Your new option description
       #[arg(long)]
       your_option: bool,
   }
   ```

2. Use the option in the appropriate converter method

3. Update documentation

## Code Style

- Follow Rust standard style (`cargo fmt`)
- Run clippy and fix warnings (`cargo clippy`)
- Write descriptive commit messages
- Add comments for complex logic

## Testing

Run tests with:
```bash
cargo test
```

Test the binary manually:
```bash
cargo build --release
./target/release/xfrm test_input.mp4 test_output.mp3
```

## Submitting Changes

1. **Fork the repository**

2. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**

4. **Test thoroughly:**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

5. **Commit with descriptive messages:**
   ```bash
   git commit -m "Add support for AVIF image format"
   ```

6. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**

## Pull Request Guidelines

- Clearly describe what your PR does
- Reference any related issues
- Include tests if applicable
- Update documentation
- Ensure CI passes (once set up)

## Ideas for Contributions

Here are some areas where contributions would be valuable:

### Features
- [ ] Batch conversion support
- [ ] Progress bar for long conversions
- [ ] Configuration file support (~/.xfrmrc)
- [ ] Custom presets (save common conversion settings)
- [ ] Parallel batch processing
- [ ] Smart compression (analyze and choose optimal settings)
- [ ] Format validation before conversion
- [ ] Subtitle/caption handling for videos

### Improvements
- [ ] Better error messages
- [ ] Performance optimizations
- [ ] Cross-platform testing
- [ ] Memory usage optimization
- [ ] Streaming support for large files

### Documentation
- [ ] More examples
- [ ] Video tutorials
- [ ] Comparison with other tools
- [ ] Performance benchmarks

### Infrastructure
- [ ] Automated releases
- [ ] Package for various distros (AUR, Homebrew, etc.)
- [ ] Docker image
- [ ] Windows builds

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow best practices

## Questions?

Feel free to:
- Open an issue for bugs or feature requests
- Start a discussion for questions
- Comment on existing issues

## License

By contributing, you agree that your contributions will be licensed under the GNU Affero General Public License v3.0.

## Finishes
enough formal mumbo, thank you for showing interest in contributing and reading till the end!!!!