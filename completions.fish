complete -c theme-manager -e

set -l commands previous next list select

complete -c theme-manager -f

complete -c theme-manager \
	 -n "not __fish_seen_subcommand_from $commands" \
	 -a "$commands"

complete -c theme-manager \
	 -n "__fish_seen_subcommand_from select" \
	 -n "test (count (commandline -pco)) -lt 3" \
	 -a "(theme-manager list)"
