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
  COMPREPLY=( $(compgen -f -- "$cur") )
  return 0
}
complete -F _stickler stickler
