# 
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# Aliases for colors
alias ls="ls --color=auto"
alias grep="grep --color=auto"

# Aliases for misspellings
alias claer="clear"

# Command not found message
# source /usr/share/doc/pkgfile/command-not-found.bash

# Auto "cd"
shopt -s autocd

# Set checking window size after each command
shopt -s checkwinsize

# Bash prompt
RED="\033[48;5;160m"
BLUE="\033[48;5;027m"
MAGENTA="\033[48;5;92m"
RESET="\033[0m"

WHITE_TEXT="\033[38;5;15m"

git_branch() {
    local BRANCH=$(git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/\1/')
    if [ ! "${BRANCH}" = "" ]; then
        echo "${MAGENTA} ${BRANCH} "
    fi
}

right_prompt() {
    local TEXT="$(date '+%A %B %d, %Y  %H:%M')"

    printf "%*s" $COLUMNS "${TEXT}"
}

PS1="${WHITE_TEXT}\[$(tput sc; right_prompt; tput rc)\]${RED} \u@\h ${BLUE} \w $(git_branch)${RESET}
$ "