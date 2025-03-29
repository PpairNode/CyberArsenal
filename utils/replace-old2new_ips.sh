#!/bin/bash
# Replace IPS from labs when it has been resetted and IPS have changed

# New range to change (e.g. 192.168.xxx.1 -> 192.168.yyy.1) -> fill in this xxx and yyy
EXTERNAL_OLD=$1
EXTERNAL_NEW=$2
INTERNAL_OLD=$3
INTERNAL_NEW=$4

grep -rlZ "192.168.${EXTERNAL_OLD}\|172.16.${INTERNAL_OLD}" . | xargs -0 sed -i -e "s/192.168.${EXTERNAL_OLD}/192.168.${EXTERNAL_NEW}/g" -e "s/172.16.${INTERNAL_OLD}/172.16.${INTERNAL_NEW}/g"

replace_name() {
    list=$1
    old=$2
    new=$3
    for file in $list; do
        echo -n "File to replace $file to "
        newfile=$(echo $file | sed "s$old$newg")
        echo "$newfile"
        mv $file $newfile
    done
}

externals=$(find . -type d -name "192.168.${EXTERNAL_OLD}*" -printf "%p ")
internals=$(find . -type d -name "172.16.${INTERNAL_OLD}*" -printf "%p ")
replace_name "${externals[@]}" $EXTERNAL_OLD $EXTERNAL_NEW
replace_name "${internals[@]}" $INTERNAL_OLD $INTERNAL_NEW
