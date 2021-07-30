@echo off

REM Read DNA HASH from file
FOR /F "tokens=* USEBACKQ" %%F IN (`cat dna_hash.txt`) DO (
SET hash=%%F
)
echo %hash%

REM Get package version from Cargo.toml
FOR /F "tokens=* USEBACKQ" %%F IN (`"cat zomes\snapmail\Cargo.toml | findstr /b version"`) DO (
SET version=%%F
)
echo %version%
for /f "tokens=1,2,3 delims= " %%a in ("%version%") do (
  set quoted=%%c
)
REM echo %quoted%
set final=%quoted:~1,-1%
REM echo %final%

REM Using github CLI
gh release create v%final% -d "snapmail.dna" -n "%hash%" -t "%final%"
