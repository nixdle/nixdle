#! /usr/bin/env bash
# check-commit-message
# see ../CONTRIBUTING.md for commit conventions
# example usage to check last commit: check-commit-message.sh "$(git log -1 --pretty=%B)"

set -eo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <message>"
  exit 1
fi

INPUT="$1"
ALLOWED_SCOPES=("cli" "lib" "docs" "nix" "ci" "chore")

check_message() {
  local message="$1"
  local scope="${message%%:*}"
  local msg_full="${message#*: }"
  readarray -t msg <<<"$msg_full"

  if [[ -z "$msg_full" ]] || [[ "$msg_full" == "$message" ]]; then
    echo "message cannot be empty or same as scope"
    return 1
  fi

  if [[ "$msg_full" =~ ';' ]]; then
    echo "message cannot contain semicolons"
    return 1
  fi

  if [[ ! " ${ALLOWED_SCOPES[*]} " =~ ${scope} ]]; then
    echo "invalid scope:  ${scope}"
    echo "allowed scopes: ${ALLOWED_SCOPES[*]}"
    return 1
  fi

  if [[ ! "${msg[0]}" =~ ^[a-z] ]] || ! [[ "${msg: -1}" =~ ^[a-z] ]]; then
    echo "message must start and end with a lowercase letter"
    return 1
  fi

  if [[ "${msg[1]}" != "" ]]; then
    echo "second line must be empty"
    return 1
  fi

  if [[ -n "${msg[2]}" ]]; then
    return 0
  fi

  for line in "${msg[@]:2}"; do
    if ((${#line} > 72)); then
      echo "body lines cannot exceed 72 characters"
      return 1
    fi
  done

  return 0
}

if (("${#INPUT}" > 50)); then
  echo "commit message cannot exceed 50 characters"
  exit 1
fi

MESSAGES=()
while [[ "$INPUT" ]]; do
  if ! [[ "$INPUT" =~ '; ' ]]; then
    MESSAGES+=("$INPUT")
    break
  fi

  MESSAGES+=("${INPUT%%; *}")
  INPUT="${INPUT#*; }"
done

for message in "${MESSAGES[@]}"; do
  echo "checking '$message'"

  if ! check_message "$message"; then
    exit 1
  fi
done

echo "all good :3"
exit 0
