<div align="center">
<h1> kubernetes-hy 🚢 </h1>
Solutions to the <a href="https://devopswithkubernetes">DevOps with Kubernetes</a> course offered by the University of Helsinki
<br />
<br />
• Developed on ARM64-based macOS •

<br />

![License](https://img.shields.io/github/license/nidnogg/kubernetes-hy?style=for-the-badge)
![Size](https://img.shields.io/github/repo-size/nidnogg/kubernetes-hy?color=orange&logo=rust&style=for-the-badge)
![Stars](https://img.shields.io/github/stars/nidnogg/kubernetes-hy?color=red&style=for-the-badge)
</div>



# Notes
## Pushing to Docker Hub
To push an image to Docker Hub, first build the image with:
```bash
❯ docker build -t <dockerhub username>/<image name>:<tag>
```
Where `tag` is the intended use for the image.

Then, double-check the list of images so that it's properly created via:
```bash
❯ docker images
```

Make sure that you are logged in to Docker Hub:
```bash
❯ docker login
# or, in case this fails later on,
❯ docker login -u <dockerhub username> -p <dockerhub password>
```

Finally, push the image to Docker Hub using:
```bash
❯ docker push <dockerhub username>/<image name>:<tag>
```

## Creating a k8s deployment
Initially, create the deployment using `kubectl`:
```bash
❯ kubectl create deployment <deployment-name/purpose> --image=<docker hub username>/<image name>:<tag>
```

The first used deployment is:
```bash
❯ kubectl create deployment hashgenerator-dep --image=nidnogg/node-app-k3d:test
```

This will create a deployment and a corresponding pod with the specified image.

> **Warning**: Do not forget the `:<tag>` at the end of the command - this will cause an `ErrImagePull` from Docker Hub.

### Useful deployment commands
```bash
❯ kubectl get deployments
❯ kubectl get pods
❯ kubectl logs -f <pod name> # for pod logging, `-f` will continuously stream output
```







