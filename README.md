<div id="top"></div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="logo.png" alt="Logo" width="160" height="80">
  </a>

  <h3 align="center">Desktop Mania</h3>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#Cluster configuration and installation">Cluster configuration and installation</a></li>
      </ul>
    </li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Desktop mania is a workspace as a service (WAAS) platform where you can access a Linux or Window desktop directly in your browser. The goal is to provide a secure environment for users who would like to work remotely.

Here's why:
* Your time should be focused on working efficiently and not wondering how to configure and secure your workspace
* You will work in a society where remote working will become the norm 
* You must have a workspace and a tool that is designed with cybersecurity concerns.

<p align="right">(<a href="#top">back to top</a>)</p>



### Built With

This section should list any major frameworks/libraries used to bootstrap your project. Leave any add-ons/plugins for the acknowledgements section. Here are a few examples.

* [Rust](https://www.rust-lang.org/fr)
* [HTML 5](https://developer.mozilla.org/fr/docs/Glossary/HTML5)
* [CSS 3](https://developer.mozilla.org/fr/docs/Web/CSS)
* [Javascript](https://developer.mozilla.org/fr/docs/Web/JavaScript)
* [Postgresql](https://www.postgresql.org/)

<p>-----------------------------------------------</p>

* [Kubernetes Rancher](https://docs.rke2.io/)
* [Kube-vip](https://kube-vip.chipzoller.dev/)
* [Metallb](https://metallb.universe.tf/)
* [Longhorn](https://longhorn.io/)
* [Traefik](https://doc.traefik.io/traefik/)
* [Prometheus](https://prometheus.io/)
* [Grafana](https://grafana.com/)
* [NoVNC](https://github.com/novnc/noVNC)
* [Docker](https://www.docker.com/)
* [Kubevirt](https://kubevirt.io/)

* [Notions(link)](https://boggy-daffodil-e82.notion.site/37ebf90ed50f4636a034768a658f0784?v=2d312371afaf4a30a29c1b1bb1984ab7)

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

This is an example on how to run the project. 

### Prerequisites

You just need to have a cluster working with kubernetes and deploy our yaml
* kubectl
  ```sh
  kubectl apply -f <yaml name>
  ```

### Cluster configuration and installation

<h3>Disclaimer:</h3> 

In this article you will learn how to deploy and manage a fully working High-Availability cluster using rke2, kube-vip, metallb and traefik. If you are not familiar with these names, do not worry you will be guided from the beginning, and you do not need any knowledges to follow this article. 


## 1 – why using rke2
As you can read in the official documentation, RKE2 or RKE Government is a Kubernetes distribution made by Rancher that focuses on security and compliance within the U.S Federal Government sector. It is a mix from two previous distributions known as RKE (1) and K3S. The most important feature (among security compliance) is that RKE2 does not rely on Docker as RKE1 does. RKE1 leveraged Docker for deploying and managing the control plane components as well as the container runtime for Kubernetes. RKE2 launches control plane components as static pods, managed by the kubelet. The embedded container runtime is containerd.

### 1.1 – architecture thinking 
Now that we have seen the importance of RKE2 and why It can be a good choice to deploy a “secure” cluster we will discuss some strategic choice on how to deploy this cluster
-	Three node master

Kubernetes is powered by two kinds of entities, Master and Workers. 
To distribute the workload, the masters will use the workers and their resources. These masters are constantly doing what we call a “leader election”. It is used to determine which master is the fastest and detect if another is down. To avoid conflict particularly with ETCD your configuration will need an odd number of server (Master node). The official documentation of rancher recommends three server nodes in a Hight Availability cluster. 
-	Workers

As said previously, workers or agent (with rancher) are useful to manage the workload, run services and pods. In a Hight availability configuration, each worker should be joined to some masters. 
-	Servers or masters

In the architecture we are using bare metal proxmox server in which we are deploying virtual machines. We will use these virtual Machine to run our workers and master under RKE2. Below you can have a bare idea of the architecture we are using and the server configuration.
-	Ip 

In our bare metal servers, we are using static IP from 10.10.20.6 to 10.10.20.9. The virtual machine’s IP are attributed by a DHCP server from 100 to 253. Note that it is important you have access to a static range of IP for the kube-vip configuration. 


## 2 – kube-vip and rke2

### 2.1 setting up the config.yaml

RKE2 can be configured thanks to a config.yaml file but also from envrionement variable. Before running a server, you need to create a config.yaml file specifying the minimum informations required such as TLS-SAN or Container Network Interface (the default CNI in rancher is CANAL, designed to use the best from flannel and calico). 
What we need to do: 
Create a rke2 folder in our server: 
```sh
        mkdir -p /etc/rancher/rke2
```
(in this folder you will find the config.yaml and rke2.yaml files used to setup rke2 and its certificates) 
Once this folder is created you can then create the file config.yaml with 
```sh
        touch config.yaml
```
or directly using nano : nano config.yaml .
		Then you can type : TLS-SAN: Kube-vip ip (don’t worry you will understand why soon) 
		The TLS-SAN is used to tell who the certificate authority for the different masters and workers is

### 2.2 installing rke2 master 

When you have written your config.yaml file and saved it you can now install rke2 server with the following command : 

```sh  
        curl -sfL https://get.rke2.io | sh –
```
This will install the necessary tarball for the rke2 server. 

When it’s installed you can enable and start the rke2-server service with : 
```sh
        systemctl enable rke2-server
        systemctl start rke2-server
```
### 2.3 joining masters between them

Now you should have single master node running. What we want now is a multi-master node architecture. So, in some way you will need to link them together. 

### 2.4 adding some workers
		
### 2.5 testing
3 – Load-balancing
	3.1 using kube-vip for his High Availability feature. 

As we want to have a 99.999% operational service, we must interconnect the nodes to ensure high availability between masters and workers. For this we use kube-vip which allows us to have a virtual ip, which will interconnect all masters and workers between them

To setup kube-vip we will do as follow : 

  - Setup environment variables
```sh
      export VIP=10.10.20.20
      export TAG=v0.3.8
      export INTERFACE=ens18
      export CONTAINER_RUNTIME_ENDPOINT=unix:///run/k3s/containerd/containerd.sock
      export CONTAINERD_ADDRESS=/run/k3s/containerd/containerd.sock
```

now that we setup all we need for kube-vip we can install it and configure it : 

```sh
      # get rbac for kube-vip 
      curl -s https://kube-vip.io/manifests/rbac.yaml > /var/lib/rancher/rke2/server/manifests/kube-vip-rbac.yaml

      #pull the kube-vip image
      crictl pull docker.io/plndr/kube-vip:$TAG

      #create an alias and specify the namespace
      alias kube-vip="ctr --namespace k8s.io run --rm --net-host docker.io/plndr/kube-vip:$TAG vip /kube-vip"

      #and now we can create the manifest to configure kube-vip
      kube-vip manifest daemonset \
      --arp \
      --interface $INTERFACE \
      --address $VIP \
      --controlplane \
      --leaderElection \
      --taint \
      --services \
      --inCluster | tee /var/lib/rancher/rke2/server/manifests/kube-vip.yaml


      #test if kube-vip is running
      kubctl get pods -n kube-system | grep kube-vip
```

```sh
  - the result

      kube-vip-ds-4356m        1/1     Running     0          48s
```

you can run the command : 

```sh
    kubuectl get nodes 
```

to see all your nodes and their states.

3.2 setting up metallb

Now that we have our high availability cluster we will configure a load balancer to be able to access our services from outside the cluster. For this we will use metallb which is an excellent load-balancer for kubernetes.

for that you just need to apply this yaml with the corresponding IP range 

```sh
# Adjust for your local IP address pool
apiVersion: v1
kind: ConfigMap
metadata:
  namespace: metallb-system
  name: config
data:
  config: |
    address-pools:
    - name: default
      protocol: layer2
      addresses:
      - 10.10.20.30-10.10.20.99

```

Congratulation : you now have a fully high Availability cluster running under RKE2 with load balancing feature.


