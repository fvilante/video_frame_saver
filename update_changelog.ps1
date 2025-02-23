# Definicao de variaveis
$CHANGELOG_FILE = "CHANGELOG.md"
$CONFIG_FILE = ".cliff.toml"

# Passo 1: Atualiza o CHANGELOG.md
Write-Host "Atualizando $CHANGELOG_FILE com git-cliff..."
git-cliff --config $CONFIG_FILE --prepend $CHANGELOG_FILE -u --bump
