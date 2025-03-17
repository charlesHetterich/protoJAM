This repository is a personal learning resource to aid my complete understanding of the [JAM grey paper](https://graypaper.com/graypaper.pdf)

## Definitions

## Sets
$ \mathbb{Y} $: the set of octet strings (byte arrays) of arbitrary length. $\mathbb{Y}_{n}$ denotes the subset of byte arrays of length $n$.

$ \mathbb{H} $: the set of 256-bit (32-byte) values expected to be arrived at through a cryptographic function (equivalent to $\mathbb{Y}_{32}$).


## Functions



#### Cryptography functions
$\mathcal{H}(m \in \mathbb{Y}) \to \mathbb{H}$ : [Blake2b](https://www.rfc-editor.org/info/rfc7693) 256-bit hash function

$\mathcal{H}_K(m \in \mathbb{Y}) \to \mathbb{H}$ : [Keccak](https://keccak.team) 256-bit hash function

$\mathcal{M}_{\sigma}(\sigma)$ : state-Merklization function, transforms our state $\sigma$ into a 32-octet commitment

#### Serialization
$\mathcal{E}(x \in \mathbb{T}) \to \mathbb{Y}$ : serialization codec transforms some $x$ into an octet sequence, where $\mathbb{T}$ is some generic set. An octet sequence yields an identity transform.

$\mathcal{E}^{-1}(x \in \mathbb{Y}) \to \mathbb{T}$ : decoder function

$\mathcal{E}_{U}(\bold{H}) \to \mathbb{Y}$ : serialization codec specific to the block header that does not include the header's ***block seal***. To include the block seal in serialization simply $\mathcal{E}(\bold{H})$ is used.


#### Blockchain
$ P(\bold{H})$ : mapping from one block header to its parent block header

$\mathcal{T}$ : gives time relative to the *JAM Common Era*, **12:00 UTC January 1, 2025**

## General Notation
...