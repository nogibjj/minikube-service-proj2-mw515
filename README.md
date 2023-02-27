# A music recommendation service using Rust Actix with Spotify Developer API (deployed with minikube)


## Usages
* Choice#1: Obtain a "client id" and a "client secret" from [Spotify developer website](https://developer.spotify.com/dashboard/login), substitute them [here](https://github.com/nogibjj/minikube-service-proj2-mw515/blob/main/src/main.rs#L65), and run `make run` to make it deployed to localhost 

* Choice#2: Run `make run-pulled-image` to pull a ready docker image and launch it (make sure you have Docker install properly and have it started)

* either way you should be able to find the website works: http://localhost:8080/possible_genres, then get recommendations from http://localhost:8080/[genre] with any genre you recognized from the [list](https://github.com/nogibjj/minikube-service-proj2-mw515/blob/main/README.md#appendix-possible-genres-from-spotify-api-as-of-feb-20-2023) (e.g. http://localhost:8080/work-out)

* 3. Example output
```text
Rock House Boogie by John Lee Hooker
Nobody Knows You When You're Down and Out - 78rpm Version by Bessie Smith
Sweet Little Angel by B.B. King
Mr. Highway Man by Howlin' Wolf
She Caught the Katy and Left Me a Mule to Ride by Taj Mahal
Blue Ridge Mountains by Fleet Foxes
Try (Just a Little Bit Harder) by Janis Joplin
Nine Below Zero - Live by Muddy Waters
Wall of Denial by Stevie Ray Vaughan
Sharp Dressed Man - 2008 Remaster by ZZ Top
Sitting On Top Of The World by Howlin' Wolf
I'd Rather Go Blind by Joe Bonamassa
Tears in Heaven by Eric Clapton
At Last - Single Version by Etta James
Playin' With My Friends by B.B. King
Angel of Mercy - Single Version by Albert King
Help Me by Sonny Boy Williamson II
Lonely Avenue - 2005 Remaster by Ray Charles
The Lady Is A Tramp - from the Hootenanny 2008 by Jools Holland & Lily Rose Cooper (née Lily Allen)
'Tain't Nobody's Bizness If I Do by Bessie Smith
```

## Steps and instructions
### 1. Start with a rust cli [program](https://github.com/nogibjj/music-reco-rust-cli-with-spotify-api/tree/main).  
* 1. Develop and run the program locally
```bash
make format-check
make lint
make run
make build-release
```

### 2. To containerize your app and launch a docker container. See [here](https://www.pluralsight.com/guides/create-docker-images-docker-hub)

#### 2.1 To push an image to DockerHub, you could follow these steps:
* `docker login` and enter your docker "user_name" and "user_secret" as prompted
* `docker build -t [NAME OF YOUR APP] .`
  * You could just run `make build` for this program
  * Build an image out of the Dockerfile:
  * `docker tag [NAME OF YOUR APP]:latest [YOUR DOCKER USER NAME]/[NAME OF YOUR APP]:latest`
  * `docker push [YOUR DOCKER USER NAME]/[NAME OF YOUR APP]:latest`
    
#### 2.2 Build and run your own docker container
  * run `make build`
  * run `make run-docker`

### 3. Deploy to a minikube cluster with minikube ctl (command line tool) 
  * install minikube ctl. See [here](https://minikube.sigs.k8s.io/docs/start/)
  * start minikube service:
    * `minikube start`
    * make an alias
      * `alias kubectl="minikube kubectl --"`
    * download the appropriate version of kubectl and access your minikube cluster
      * `kubectl get po -A` (which runs `minikube kubectl -- get po -A`)
    * deploy applications
      * make sure you have pushed your docker image to DockerHub (find instructions [here](https://www.pluralsight.com/guides/create-docker-images-docker-hub))
      * create a deployment and expose it on port 8080 
        * `kubectl create deployment [NAME OF YOUR APP] --image=registry.hub.docker.com/mianwu/musicreco:latest`
        * `kubectl expose deployment [NAME OF YOUR APP] --type=NodePort --port=8080`
      * Get the deployment information
        * `kubectl get services [NAME OF YOUR APP]`
      * Let minikube launch a web browser (the terminal needs to be open to run it)
        * `minikube service [NAME OF YOUR APP]`
      * You could also use kubectl to forward the port from 8080 to local 7080
        * `kubectl port-forward service/[NAME OF YOUR APP] 7080:8080`

![minikube launch]("minikubelaunch.png")
## Note
If you directly run the codes from this Codespaces, it could not work as I failed to install Docker properly in it.

I developed the app on both AWS Cloud9 and Terminal on MAC with an Intel chip, with Docker, Rust and minikube set up correctly.

## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [a-rust-mini-kube-project-from-wanqian](https://github.com/nogibjj/Project2-Wanqian)
* [minikube-doc](https://minikube.sigs.k8s.io/docs/start/)
* [an-example-kubernetes-project-from-nogibjj](https://github.com/nogibjj/coursera-applied-de-kubernetes-lab)
* [an-example-hello-minikube](https://kubernetes.io/docs/tutorials/hello-minikube/)
* [webdocker-github-code-example](https://github.com/nogibjj/rust-mlops-template/tree/main/webdocker)
* [install-docker-on-debian](https://www.fosslinux.com/49959/install-docker-on-debian.html)
* [create-docker-image-pull/push-image](https://www.pluralsight.com/guides/create-docker-images-docker-hub)
* [containerized-rock-paper-scissors-actix-microservice](https://github.com/nogibjj/rust-world-spr23/tree/main/actix-containerized-microservice-wk3/actixdocker)


## Appendix. Possible genres from Spotify API as of Feb 20, 2023
```bash
{
  "genres": [
    "acoustic",
    "afrobeat",
    "alt-rock",
    "alternative",
    "ambient",
    "anime",
    "black-metal",
    "bluegrass",
    "blues",
    "bossanova",
    "brazil",
    "breakbeat",
    "british",
    "cantopop",
    "chicago-house",
    "children",
    "chill",
    "classical",
    "club",
    "comedy",
    "country",
    "dance",
    "dancehall",
    "death-metal",
    "deep-house",
    "detroit-techno",
    "disco",
    "disney",
    "drum-and-bass",
    "dub",
    "dubstep",
    "edm",
    "electro",
    "electronic",
    "emo",
    "folk",
    "forro",
    "french",
    "funk",
    "garage",
    "german",
    "gospel",
    "goth",
    "grindcore",
    "groove",
    "grunge",
    "guitar",
    "happy",
    "hard-rock",
    "hardcore",
    "hardstyle",
    "heavy-metal",
    "hip-hop",
    "holidays",
    "honky-tonk",
    "house",
    "idm",
    "indian",
    "indie",
    "indie-pop",
    "industrial",
    "iranian",
    "j-dance",
    "j-idol",
    "j-pop",
    "j-rock",
    "jazz",
    "k-pop",
    "kids",
    "latin",
    "latino",
    "malay",
    "mandopop",
    "metal",
    "metal-misc",
    "metalcore",
    "minimal-techno",
    "movies",
    "mpb",
    "new-age",
    "new-release",
    "opera",
    "pagode",
    "party",
    "philippines-opm",
    "piano",
    "pop",
    "pop-film",
    "post-dubstep",
    "power-pop",
    "progressive-house",
    "psych-rock",
    "punk",
    "punk-rock",
    "r-n-b",
    "rainy-day",
    "reggae",
    "reggaeton",
    "road-trip",
    "rock",
    "rock-n-roll",
    "rockabilly",
    "romance",
    "sad",
    "salsa",
    "samba",
    "sertanejo",
    "show-tunes",
    "singer-songwriter",
    "ska",
    "sleep",
    "songwriter",
    "soul",
    "soundtracks",
    "spanish",
    "study",
    "summer",
    "swedish",
    "synth-pop",
    "tango",
    "techno",
    "trance",
    "trip-hop",
    "turkish",
    "work-out",
    "world-music"
  ]
}
```
