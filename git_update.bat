@echo off
setlocal
cd /d %~dp0
git add .
git commit -m %1
git push origin HEAD

endlocal
pause