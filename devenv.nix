{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

let
  # Bevy Remote Protocol MCP server - provides 27+ tools for real-time
  # entity/component manipulation, screenshots, and keyboard input
  bevy_brp_mcp = pkgs.rustPlatform.buildRustPackage rec {
    pname = "bevy_brp_mcp";
    version = "0.17.0";

    src = pkgs.fetchFromGitHub {
      owner = "natepiano";
      repo = "bevy_brp";
      rev = "v${version}";
      hash = "sha256-hJ5jTxnulp96N7uMpmwCeuOVUX754mOi3dw9yQKf/GE=";
    };

    cargoHash = "sha256-8MFs2Zhy/P/5HRgcSbLXyMbw9ZC6CqDCG42of65fHlk=";

    # Skip doctests - they fail in Nix build environment but actual code compiles fine
    doCheck = false;

    meta = with lib; {
      description = "MCP server for Bevy Remote Protocol - autonomous AI development for Bevy games";
      homepage = "https://github.com/natepiano/bevy_brp";
      license = licenses.mit;
    };
  };

  # Scientific debugging MCP server - hypothesis testing, anomaly detection,
  # and performance profiling for Bevy games
  # Note: No releases yet, building from main branch
  bevy_debugger_mcp = pkgs.rustPlatform.buildRustPackage rec {
    pname = "bevy_debugger_mcp";
    version = "0.1.10-unstable";

    src = pkgs.fetchFromGitHub {
      owner = "ladvien";
      repo = "bevy_debugger_mcp";
      rev = "main";
      hash = "sha256-TnOOi6SdJLeGoH4470VbBJthuSVSNsqms0ZYTPxPdrw=";
    };

    cargoHash = "sha256-QcaPfjAxUMJhzs8swBoJOVqovKYqA/f9M3GOxxHkEjc=";

    # Skip tests for faster builds
    doCheck = false;

    meta = with lib; {
      description = "Scientific debugging and automated QA MCP server for Bevy games";
      homepage = "https://github.com/ladvien/bevy_debugger_mcp";
      license = licenses.mit;
    };
  };
in
{
  languages.rust.enable = true;

  # MCP servers for AI-assisted Bevy development
  # After entering devenv, restart Claude Code conversation to load these tools
  packages = [
    bevy_brp_mcp
    bevy_debugger_mcp
  ];
}
