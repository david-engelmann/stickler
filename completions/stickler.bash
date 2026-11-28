# bash completion for stickler
_stickler() {
  local cur prev
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  prev="${COMP_WORDS[COMP_CWORD-1]}"
  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "check render review census rejudged split contested pattern files borderline outcomes verdicts shapes choices categories grounds judgements standings states flags models" -- "$cur") )
    return 0
  fi
  command -v compopt >/dev/null 2>&1 && compopt -o filenames
  local line
  while IFS= read -r line; do
    COMPREPLY+=( "$line" )
  done < <(compgen -f -- "$cur")
  return 0
}
complete -F _stickler stickler
