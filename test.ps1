
param(
	[parameter(mandatory=$true, position=0)][int32]$DayNumber,
	[parameter(mandatory=$false, position=1, ValueFromRemainingArguments=$true)]$remaining
)

$day="day" + $DayNumber.ToString().PadLeft(2, "0")

cargo.exe test --bin "$day" $remaining
