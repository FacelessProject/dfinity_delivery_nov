This is the documentation on the distributed secret-sharing procedure we adopt to derive the master key of the Faceless protocol and how we integrate the distributed secret-sharing scheme with Faceless's IBE scheme. 

## Distributed Shamir's secret-sharing scheme

During the registration process, a user's master key will be derived from the access tokens provided by the authentication mechanisms of various Web 2.0 accounts. The derivation algorithm is a distributed ($t$ of $n$) Shamir secret-sharing procedure. 
Essentially, the secret key will be shared among $n$ parties in a distributed manner, and it requires at least $t$ shares to reconstruct the secret key. 

The following is the distributed secret-sharing scheme we adopt:

**Step 1.**

For each Web 2.0 account $i \in [1, n]$, we derive a secret $s_i$ sampled from a finite field $\mathbb{F}_q$.

**Step 2.**

Define a degree $t-1$ randomized polynomial in space $\mathbb{F}_q$:

$f_i(x) = s_i + r_1x + r_2x^2 + \cdots + r_{t-1}x^{t-1}$

where $r_j, j \in [1, t-1]$ are randomly sampled coefficients from space $\mathbb{F}_q$.

**Step 3.**

Evalute the polynomial at $n$ pre-selected random points to generate $n$ secret shares:

$s_{i1} = f_i(x_1)$

$s_{i2} = f_i(x_2)$

$\cdots$

$s_{in} = f_i(x_n)$


**Step 4.**

Repeat the above procedure for each account $i$, and the secret key to be shared will be </p><img src="https://github.com/HeisenbergLin22/dfinity_delivery_nov/blob/main/Screenshot%202023-12-08%20at%2015.24.51.png" alt="" width="5%"/>, which is the constant coefficient of the polynomial </p><img src="https://github.com/HeisenbergLin22/dfinity_delivery_nov/blob/main/Screenshot%202023-12-08%20at%2015.24.58.png" alt="" width="7%"/> 

The secret share corresponding to each account $j$ would be </p><img src="https://github.com/HeisenbergLin22/dfinity_delivery_nov/blob/main/Screenshot%202023-12-08%20at%2015.25.08.png" alt="" width="12%"/> 


***Reconstruction of secret***

Due to the linearity of Shamir's secret sharing scheme, it's easy to see that as long as we have at least $t$ of these shares, we'll be able to recover the secret key via a standard Lagrange interpolation algorithm.



## Integration of the above Algorithm with Faceless's IBE scheme

Below are more details about tailoring the above secret sharing to Faceless.

Faceless uses an identity-based encryption (IBE) scheme that is currently designed on bilinear map with three groups $\mathbb{G}_1 \times \mathbb{G}_2 \rightarrow \mathbb{G}_t$, and implemented on Elliptive Curve *bn128*. The master secret key of the IBE is a random scalar sampled from $\mathbb{F}_q^*$ where:

```
q = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
```

In the Faceless protocol, the master key will be derived from the access token of Web 2 accounts by running the above distributed secret sharing scheme.


The IBE scheme derives an identity key with $\mathsf{ID}$ as $sk_{\mathsf{ID}} = H(\mathsf{ID})^s$, where $H$ is a cryptographic hash function, and $s$ is the master secret key.


Specifically, in the current version, we use a (2-of-3) Shamir secret-sharing scheme for Faceless.

Therefore, during the registration process, after the user has authenticated to 3 platforms and obtained 3 access tokens, these tokens will be used to derive $3$ secret shares $(s_1, s_2, s_3)$, each of which is related to one platform. In addition, three sub-accounts related to the three platforms will be derived by using the above IBE identity key derivation formula, with $\mathsf{ID}$ being the corresponding platform HRI account. 
These 3 platform-managing accounts will also be registered in the canister.  

During the login process, as long as the user authenticates to 2 platforms, the IBE master secret key will be recovered from the access tokens. Afterward, the sub-accounts that have been derived during the registration process will also be recovered and shown on the home page.

As long as the master key for the underlying IBE scheme is recovered, we'll be able to run all the rest functionalities of Faceless protocol, such as HRI account registration, deposit, withdrawal, transfer, etc. 
