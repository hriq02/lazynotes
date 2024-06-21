@echo off
setlocal
cd /d %~dp0
REM set /p commit_message="Enter your commit message: "
REM"%commit_message%"
git add .
git commit -m %1
git push origin HEAD

endlocal
pause