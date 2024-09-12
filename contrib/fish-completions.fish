# Based on the original bluetoothctl fish completions
set -l cmds pair trust untrust block unblock connect disconnect

function __fish_list_bluetoothctl_devices
    bluetoothctl devices 2>/dev/null | cut -d' ' -f 3
end

complete -f -c bt -n "not __fish_seen_subcommand_from $cmds" -a pair -d "Pair with device"
complete -f -c bt -n "not __fish_seen_subcommand_from $cmds" -a trust -d "Trust device"
complete -f -c bt -n "not __fish_seen_subcommand_from $cmds" -a untrust -d "Untrust device"
complete -f -c bt -n "not __fish_seen_subcommand_from $cmds" -a block -d "Block device"
complete -f -c bt -n "not __fish_seen_subcommand_from $cmds" -a unblock -d "Unblock device"
complete -f -c bt -n "not __fish_seen_subcommand_from $cmds" -a connect -d "Connect device"
complete -f -c bt -n "not __fish_seen_subcommand_from $cmds" -a disconnect -d "Disconnect device"
complete -f -c bt -n "__fish_seen_subcommand_from pair trust untrust block unblock connect disconnect" -a "(__fish_list_bluetoothctl_devices)"
