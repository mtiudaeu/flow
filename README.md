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
```
sudo docker build -t ubuntu-rust .
```
```
sudo docker-compose run ubuntu
```
