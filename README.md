
#### Setup Instructions
------------------
1. FORK then Clone your own forked repo.
2. Open the project in your IDE aka VSCodium
3. Ensure that the JDK is properly set up in your IDE.

# Cøsmverse
[![App](https://github.com/CosmosContracts/Cosmverse/actions/workflows/react.yml/badge.svg)](https://github.com/CosmosContracts/Cosmverse/actions/workflows/react.yml)

Cross-chain #NFT marketplace on 
Juno.

Auction, mint & explore the amazing world of nonfungible assets. 

```
git clone https://github.com:KitBaroness/Cosmverse.git
touch .env.local
```

in `.env.local` add following params:
```
REACT_APP_NETWORK=testnet
REACT_APP_SLATE_KEY=
```

Runs the app in the development mode.
```
yarn
yarn start
```
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.     
The page will reload if you make edits. You will also see any lint errors in the console.


![Cosmverse Logo (Final) Banner 3t](https://user-images.githubusercontent.com/79812965/139874498-591e3fe8-7f21-4ddb-aac4-f1d3c73aeeec.png)

#### FOR DOCKER IMAGE
* Go to Dapp directory in terminal
```
# If you Don't have Docker
mkdir -p ~/.docker/cli-plugins/
curl -L "https://github.com/docker/buildx/releases/download/v0.13.1/buildx-v0.13.1.linux-amd64" -o ~/.docker/cli-plugins/docker-buildx
chmod a+x ~/.docker/cli-plugins/docker-buildx
export DOCKER_CLI_PLUGIN_DIR=~/.docker/cli-plugins
```
```
export DOCKER_BUILDKIT=1
 # Build the Docker image
DOCKER_BUILDKIT=1 docker build -t cosmverse .
 # Run the Docker container
docker run -p 3000:3000 cosmverse
```
