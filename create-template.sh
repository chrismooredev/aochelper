

function help() {
	echo "$0: Copy ./day_template.rs to src/days with pre-populated info"
	echo "$0 <day_number> <day_name>"
	exit 1
}

if [ -z "$1" ] || [ -z "$2" ] ; then
	help
fi

dnum="$(printf "%02d" "$1")"
name="${2/\//\\/}"
newname="src/bin/day$dnum.rs"

cp day_template.rs "$newname"
sed -i "s/{{DayNum}}/$dnum/g" "$newname"
sed -i "s/{{DayName}}/$name/g" "$newname"

