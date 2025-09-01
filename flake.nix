{
  description = "Leptos Forms - Type-safe, reactive form handling library for Leptos applications";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Platform-specific browser packages
        browserPackages = if pkgs.stdenv.isDarwin then
          [ pkgs.firefox ]  # Firefox works on macOS
        else
          [ pkgs.firefox pkgs.chromium ];  # Both work on Linux
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            
            # WASM tools
            wasm-pack
            wasm-bindgen-cli
            
            # Development tools
            cargo-watch
            cargo-edit
            cargo-audit
            cargo-tarpaulin
            
            # System tools
            pkg-config
            openssl
            curl
            git
            
            # For wasm-bindgen-test (platform-specific)
            geckodriver
          ] ++ browserPackages;
          
          shellHook = ''
            echo "🚀 Leptos Forms Development Environment"
            echo "📦 Rust: $(rustc --version)"
            echo "🔧 wasm-pack: $(wasm-pack --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run tests"
            echo "  cargo check          - Check compilation"
            echo "  wasm-pack test       - Run WASM tests in browser"
            echo "  make help            - Show all available commands"
            echo ""
            echo "Happy coding! 🎉"
          '';

          # Environment variables for WASM development
          RUST_BACKTRACE = "1";
          RUST_LOG = "info";
        };

        # Development shell for CI/CD
        devShells.ci = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            wasm-pack
            wasm-bindgen-cli
            geckodriver
          ] ++ browserPackages;
        };
      }
    );
}
