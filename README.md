```
mkdir bootstrap
```
```
cd bootstrap
```
```
curl https://raw.githubusercontent.com/mtiudaeu/flow/master/bootstrap/Dockerfile > Dockerfile
```
```
curl https://raw.githubusercontent.com/mtiudaeu/flow/master/bootstrap/docker-compose.yml > docker-compose.yml
```
Before next command, edit Dockerfile and write "user.email" & "user.name"
```
sudo docker build -t ubuntu-rust .
```
Remove space before "source: ..." & "target: ..." in docker-compose.yml. (Very sadly :(, curl add/remove space on the docker-compose.yml)
```
sudo docker-compose run ubuntu
```
