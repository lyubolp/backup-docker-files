# backup-docker-files
I need to backup files from Docker


## Code flow


1. Initialize repo
2. Initialize new backup
3. Discover containers
4. For each container, extract files to staging
5. Copy files from staging to backup
6. Finalize backup