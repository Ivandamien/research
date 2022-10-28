# **Research**
## 2. Computer Science
- Modular multiplicative inverse is an important part of any cryptographic implementation. Write a code that calculates the Modular multiplicative inverse of a number using the Euclidean Algorithm? Please write this in Rust. Please heavily comment your code so it provides a lot of explainer to someone who has experience programming but is new to Rust and the Euclidean Algorithm. 


### Here is the finished solution to the research [Modular Multiplicative Inverse Implemented in Rust](src/main.rs)

screenshot output
![](img/Screenshot%20from%202022-10-28%2011-28-54.png)

## 3. Math
- Please compute the SHA256 of 2009. Perform a transaction on the Görli testnet network and put this computed SHA in the data section of your transaction. Submit the transaction hash of this transaction in the google form.


- Answer : https://goerli.etherscan.io/tx/0x48d3e1bad2daf2723560b40d707af8410f3b9e321fa0d91b2679b8ed08607721
```sh
export PK = "<< Insert you'r private key here >> "
export GORLI_RPC = << You can get a Görli rpc_url from Alchemy or here: https://sourcegraph.com/search?q=context:global+https://eth-goerli.g.alchemy.com&patternType=standard >>
```

```sh
cast send                 \
    --private-key $PK     \
    --rpc-url $GORLI_RPC  \
    --value 0.0069ether 0x000000e2cf0d6a036022a78d694af77456b32394 \
    $(echo 2009 | shasum -a 256  | cast --from-utf8)
```

