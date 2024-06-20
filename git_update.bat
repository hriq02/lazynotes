@echo off
setlocal
cd /d %~dp0
set /p commit_message="Enter your commit message: "
git add .
git commit -m "%commit_message%"
git push origin HEAD

endlocal
pause