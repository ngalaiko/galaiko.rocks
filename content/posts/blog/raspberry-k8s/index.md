---
title: "Raspberry Pi: K8s cluster"
date: 2019-06-24T21:28:26+02:00
---

I spent too much time configuring Kubernetes cluster on Raspberry Pi today, so here are notes for my future
self on how to do that:
 
## SD card

* Download operation system. 

	I am using [HypriotOS v1.10.0](https://github.com/hypriot/image-builder-rpi/releases/download/v1.10.0/hypriotos-rpi-v1.10.0.img.zip)
    It is based on __Raspbian Buster Lite__, has a preinstalled docker and some other minor things
    to simplify cluster setup:

        $ uname -a 
        Linux node-red 4.14.98-v7+ #1200 SMP Tue Feb 12 20:27:48 GMT 2019 armv7l GNU/Linux
		$ docker version
        Client:
         Version:           18.06.3-ce
         API version:       1.38
         Go version:        go1.10.3
         Git commit:        d7080c1
         Built:             Wed Feb 20 02:42:54 2019
         OS/Arch:           linux/arm
         Experimental:      false

        Server:
         Engine:
         Version:          18.06.3-ce
         API version:      1.38 (minimum version 1.12)
         Go version:       go1.10.3
         Git commit:       d7080c1
         Built:            Wed Feb 20 02:38:25 2019
         OS/Arch:          linux/arm
         Experimental:     false

* Flash it to SD cards using [flash](https://github.com/hypriot/flash/releases) 

	Don't forget to define a hostname. It will save you some time later.

        $ flash -n node-red ./Downloads/hypriotos-rpi-v1.10.0.img
* Provision all nodes you have like this and boot them up.
* Install Kubernetes on __all__ nodes.

    Note the version here: `1.13.5`. Because of changes in `1.14.0`, Kubernetes requires enabled pids
    cgroup. But kernels before `4.19.46-v7+` do not support it. If you are reading this after some 
    time, it should be fixed, and the latest version of Kubernetes might work just fine.

        $ curl -s https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo apt-key add - && \
         echo "deb http://apt.kubernetes.io/ kubernetes-xenial main" | sudo tee /etc/apt/sources.list.d/kubernetes.list && \
         sudo apt-get update -q && \
         sudo apt-get install -qy kubeadm=1.13.5 kubectl=1.13.5 kubelet=1.13.5
* Initialize Kubernetes on the __master__ node:
    ```
    $ sudo kubeadm init
    ```
    Notice `sudo` here. For some reason, if you do it as `root`, it can fail with an error (couldn't
    find what the error was :( )

    If all goes fine, you will see an output similar to: 
	```
    ...
    Your Kubernetes master has initialized successfully!
    To start using your cluster, you need to run the following as a regular user:
      mkdir -p $HOME/.kube
      sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
      sudo chown $(id -u):$(id -g) $HOME/.kube/config
      You should now deploy a pod network to the cluster.
    Run "kubectl apply -f [podnetwork].yaml" with one of the options listed at:
      https://kubernetes.io/docs/concepts/cluster-administration/addons/
    You can now join any number of machines by running the following on each node
    as root:
      kubeadm join --token TOKEN 192.168.1.100:6443 --discovery-token-ca-cert-hash HASH
	```

* Do what the output says: 
	```
    $ sudo cp /etc/kubernetes/admin.conf $HOME/
    $ sudo chown $(id -u):$(id -g) $HOME/admin.conf
    $ export KUBECONFIG=$HOME/admin.conf
	```
    Now you should be able to list all nodes. There is a single __NotReady__ __master__ 
    node at this point:
	```
    $ kubectl get nodes
    NAME         STATUS     ROLES    AGE   VERSION
    node-red     NotReady   master   17m   v1.13.5
	```

* Add other nodes to the cluster.
    ```
	$ sudo kubeadm join --token TOKEN 192.168.1.100:6443 --discovery-token-ca-cert-hash HASH
    ```
    Notice `sudo` again.

* Now, you should be able to see all nodes from the master node:
    ```
    $ kubectl get nodes
    NAME         STATUS     ROLES    AGE    VERSION
    node-black   NotReady   <none>   106m   v1.13.5
    node-blue    NotReady   <none>   105m   v1.13.5
    node-green   NotReady   <none>   105m   v1.13.5
    node-red     NotReady   master   110m   v1.13.5
    ```

* Deploy container network from the __master__ node. 
    ```
    $ kubectl apply -f "https://cloud.weave.works/k8s/net?k8s-version=1.13.5"
    ```
* After some time, all nodes should be ready.
    ```
    $ kubectl get nodes
    NAME         STATUS     ROLES    AGE    VERSION
    node-black   Ready     <none>   106m   v1.13.5
    node-blue    Ready     <none>   105m   v1.13.5
    node-green   Ready     <none>   105m   v1.13.5
    node-red     Ready     master   110m   v1.13.5
    ```

# Links

* https://blog.hypriot.com/post/setup-kubernetes-raspberry-pi-cluster/ 
* https://kubecloud.io/setting-up-a-kubernetes-1-11-raspberry-pi-cluster-using-kubeadm-952bbda329c8
* https://github.com/hypriot/flash
* https://github.com/teamserverless/k8s-on-raspbian/issues/16
