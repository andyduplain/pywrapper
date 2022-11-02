cargo build --release
if (-not $?) {
    exit 1
}

if (test-path -path bin) { rm -r -force bin }
mkdir -force bin | out-null

foreach($file in Get-Content .\install_names.txt) {
    $file = $file += ".exe"
    echo "   Installing bin\$file"
    copy -force target\release\pywrapper.exe bin\$file
    if (-not $?) {
        exit 2
    }
}

cargo clean --release
