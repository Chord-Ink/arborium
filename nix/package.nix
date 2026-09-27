{ lib, rustPlatform, languageGrammars ? [ ] }:

rustPlatform.buildRustPackage {
  pname = "arborium";
  version = (lib.importTOML ../crates/arborium-cli/Cargo.toml).package.version;
  src = lib.cleanSource ../.;
  cargoRoot = "crates/arborium-cli";
  cargoLock.lockFile = ../crates/arborium-cli/Cargo.lock;

  buildNoDefaultFeatures = languageGrammars != [ ];
  buildFeatures = map (language: "lang-${language}") languageGrammars;

  meta = {
    description = "Tree-sitter syntax highlighting for the terminal and HTML";
    homepage = "https://github.com/Chord-Ink/arborium";
    # The default bundle includes Nginx (GPL) and Uiua (MPL).
    license = with lib.licenses; [ mit asl20 ]
      ++ lib.optional (languageGrammars == [ ] || builtins.elem "nginx" languageGrammars) gpl3Only
      ++ lib.optional (languageGrammars == [ ] || builtins.elem "uiua" languageGrammars) mpl20;
    mainProgram = "arborium";
  };
}
