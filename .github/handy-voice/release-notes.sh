#!/usr/bin/env bash
# Handy Voice: release notes of a fork build, in Russian, to stdout.
# Usage: bash .github/handy-voice/release-notes.sh <Handy tag> <build N> <version> <Handy commit> <fork commit>
# The fork's changes are the subjects of its own commits between the two
# commits, without the workflow's "voice: base Handy …" commits.
set -euo pipefail

base=${1:?Handy tag} build=${2:?build number} version=${3:?version}
from=${4:?Handy commit} to=${5:?fork commit}

if [ "$build" = 0 ]; then
  build_text="0 (первая)"
else
  build_text=$build
fi

changes=$(git log --reverse --no-merges --format='%s' "$from..$to" |
  grep -E '^(voice:|Handy Voice:)' | grep -v '^voice: base Handy ' |
  sed 's/^/- /' || true)

cat <<EOF
**Handy Voice $version** — Handy с доработками Deut Lab.

- Основа: Handy [$base](https://github.com/cjpais/Handy/releases/tag/$base)
- Сборка форка на этой версии Handy: $build_text
- Исходный код: тег \`voice-v$version\`

Установленный Handy Voice предложит обновиться сам. Установщики не подписаны сертификатами Microsoft и Apple: Windows может показать предупреждение SmartScreen («Подробнее» → «Выполнить в любом случае»), macOS — что разработчик не проверен (Системные настройки → «Конфиденциальность и безопасность» → «Всё равно открыть»).

### Доработки форка

${changes:-- (нет)}
EOF
