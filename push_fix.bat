@echo off
echo STAGING FILES... > push_log.txt
git add . >> push_log.txt 2>&1

echo COMMITTING... >> push_log.txt
git commit -m "feat: finalize wavehack submission (microchains, docs, cleanup)" >> push_log.txt 2>&1

echo PUSHING... >> push_log.txt
git push origin main >> push_log.txt 2>&1

echo STATUS... >> push_log.txt
git status >> push_log.txt 2>&1
