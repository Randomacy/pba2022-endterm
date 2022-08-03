# PBA Midterm
 
## 1. Introduction
A decentralized exchange (DEX) is a peer-to-peer marketplace where transactions occur directly between traders. DEXs foster financial transactions without any intermediaries and are one of the more exciting applications of blockchain technology.

In the PBA 2022 end-term project, I intend to implement a simple multi-token DEX through a simple multi-asset pallet.

## 2. Approach
### Tokens
Tokens are assets, and is supposed to be implemented as:
```
pub struct Token<T:Config> {
    pub hash: T::Hash,
    pub symbol: Vec<u8>,
    pub balance: Balance,
}
```

### Trades
The trades are what happens on the platform, and is implemented as:
```
pub struct Trade<T:Config> {
    hash: T::Hash,
    base: T::Hash,
    quote: T::Hash,
    buyer: T::AccountId,
    seller: T::AccountId, 
    maker: T::AccountId, 
    taker: T::AccountId, 
    otype: OrderType, 
    price: T::Price, 
    base_amount: T::Balance, // base token amount to exchange
    quote_amount: T::Balance, // quote token amount to exchange
}
```

## 3. The Twist
The twist that I wanted to implement in the marketplace was to answer the question of "what if I could swap my excess scores in one subject with another subject?". 

Instead of tokens, I imagined the marketplace to be a place to trade exam scores. Conceptually, it would have been the same as a DEX, i.e. token A ~> score A. 

## 4. Outcome
Nothing was implement due to a lack of time from a lot of factors, ranging from urgent matters from back home to not being able to compile the code on my M1 machine. 

## 5. Learnings
I learned a lot from this exercise, just by going through the painful process of writing custom pallets and reading others' code. 
