# bash completion for stickler
_stickler() {
  local cur
  COMPREPLY=()
  cur="${COMP_WORDS[COMP_CWORD]}"
  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "check render review census rejudged split contested pattern files borderline outcomes verdicts shapes choices categories grounds judgements standings states flags models" -- "$cur") )
    return 0
  fi
  return 0
}
complete -o bashdefault -o default -F _stickler stickler
