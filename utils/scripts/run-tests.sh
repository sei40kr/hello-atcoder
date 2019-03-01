#!/usr/bin/env bash

set -u
IFS=$'\n\t'

# run-tests.sh --- Run AtCoder test cases
# author: Seong Yong-ju <sei40kr@gmail.com>

red="$(tput setaf 1)"
green="$(tput setaf 2)"
yellow="$(tput setaf 3)"
bold="$(tput bold)"
reset="$(tput sgr0)"

cmd="$@"
buffile="$(mktemp)"
errfile="$(mktemp)"

shopt -s nullglob

for infile in in_*.txt; do
	outfile="out_${infile:3}"

	if [[ ! -f "$outfile" ]]; then
		continue
	fi

  echo -n "${infile} - "

	eval "${cmd[@]}" <"$infile" 1>"$buffile" 2>"$errfile"
	if [[ "$?" != 0 ]]; then
		cat <<EOM
${bold}${red}err${reset}

runtime error:
$(cat "$errfile")

EOM
		break
	fi

	if diff -aq --strip-trailing-cr "$buffile" "$outfile" >/dev/null; then
		echo "${bold}${green}ok${reset}"
	else
		cat <<EOM
${bold}${yellow}ng${reset}

expected:
$(cat "$outfile")

actual:
$(cat "$buffile")

EOM
	fi
done

rm -f "$buffile" "$errfile"
